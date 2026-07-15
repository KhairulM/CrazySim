import threading
import time
from typing import Optional

import rclpy
from tf2_msgs.msg import TFMessage
from geometry_msgs.msg import TransformStamped

from NatNetClient import NatNetClient
import intercept_common as ic
from drone import Drone


def _detect_local_ip_for_server(server_ip: str) -> str:
    """Return the local interface IP that routes to ``server_ip``.

    Uses a connect-less UDP socket so no packets are sent; the kernel simply
    resolves the outbound interface. Falls back to ``0.0.0.0`` (bind-any) if the
    route cannot be determined.
    """
    import socket
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    try:
        sock.connect((server_ip, 1))
        return sock.getsockname()[0]
    except OSError:
        return '0.0.0.0'
    finally:
        sock.close()


class MocapTfPublisher:
    """Optional ROS 2 TF publisher for mocap poses.

    ``rclpy`` is imported lazily so the controller keeps its pure-``cflib``
    dependency footprint unless TF publishing is explicitly requested.
    """

    def __init__(self, world_frame: str = 'world',
                 node_name: str = 'intercept_mocap_tf') -> None:
        self._rclpy = rclpy
        self._TFMessage = TFMessage
        self._TransformStamped = TransformStamped
        self._world_frame = world_frame
        self._owns_rclpy = not rclpy.ok()
        if self._owns_rclpy:
            rclpy.init()
        self._node = rclpy.create_node(node_name)
        self._pub = self._node.create_publisher(TFMessage, 'tf', 10)

    def publish(self, child_frame_id: str, pose: ic.MocapPose) -> None:
        transform = self._TransformStamped()
        transform.header.stamp = self._node.get_clock().now().to_msg()
        transform.header.frame_id = self._world_frame
        transform.child_frame_id = child_frame_id
        px, py, pz = pose.position
        transform.transform.translation.x = float(px)
        transform.transform.translation.y = float(py)
        transform.transform.translation.z = float(pz)
        qx, qy, qz, qw = pose.quat_xyzw
        transform.transform.rotation.x = float(qx)
        transform.transform.rotation.y = float(qy)
        transform.transform.rotation.z = float(qz)
        transform.transform.rotation.w = float(qw)
        self._pub.publish(self._TFMessage(transforms=[transform]))

    def close(self) -> None:
        try:
            self._node.destroy_node()
        except Exception:  # pragma: no cover - best-effort teardown
            pass
        if self._owns_rclpy:
            try:
                self._rclpy.shutdown()
            except Exception:  # pragma: no cover - best-effort teardown
                pass


class MocapReceiver:
    """Stream OptiTrack rigid-body poses and forward them to Drone.

    The receiver owns a background NatNet thread (started by :meth:`start`).
    Each rigid-body frame is transformed into the ROS FLU convention via
    :func:`intercept_common.transform_mocap_pose` and pushed to the Drone
    registered under the matching streaming id through ``extpos.send_extpose``.
    Poses are optionally re-published on a ROS 2 TF tree. Frames for
    unregistered ids are ignored.
    """

    def __init__(self, cfg: ic.MocapConfig,
                 tf_publisher: Optional[MocapTfPublisher] = None) -> None:
        self._cfg = cfg
        self._tf_publisher = tf_publisher
        self._targets: dict = {}
        self._last_send_stamp: dict[int, float] = {}
        self._lock = threading.Lock()
        self._client = None

    def register(self, rigid_body_id: int, drone: Drone,
                 frame_id: Optional[str] = None) -> None:
        """Route frames for ``rigid_body_id`` to ``cf`` (and TF ``frame_id``)."""
        rb_id = int(rigid_body_id)
        with self._lock:
            self._targets[rb_id] = (drone, frame_id or f'cf_{rb_id}')

    def start(self) -> None:
        client = NatNetClient()
        client.serverIPAddress = self._cfg.server_ip
        client.localIPAddress = (
            self._cfg.local_ip
            or _detect_local_ip_for_server(self._cfg.server_ip))
        client.multicastAddress = self._cfg.multicast_address
        client.commandPort = self._cfg.command_port
        client.dataPort = self._cfg.data_port
        client.rigidBodyListener = self._on_rigid_body
        self._client = client
        client.run()

    def _on_rigid_body(self, rigid_body_id, position, rotation,
                       tracking_valid) -> None:
        rb_id = int(rigid_body_id)
        now = time.monotonic()
        with self._lock:
            target = self._targets.get(rb_id)
            min_dt = 0.0
            if self._cfg.mocap_send_rate_hz > 0.0:
                min_dt = 1.0 / self._cfg.mocap_send_rate_hz
            if min_dt > 0.0:
                last_stamp = self._last_send_stamp.get(rb_id, 0.0)
                if (now - last_stamp) < min_dt:
                    return
                self._last_send_stamp[rb_id] = now
        if target is None:
            return
        drone, frame_id = target

        pose = ic.transform_mocap_pose(
            self._cfg, int(rigid_body_id), position, rotation, tracking_valid)
        if not pose.tracking_valid:
            return

        try:
            drone.send_mocap_pose(ic.MocapPose(
                rigid_body_id=rb_id,
                position=pose.position,
                quat_xyzw=pose.quat_xyzw,
                tracking_valid=pose.tracking_valid
            ))
        except Exception:  # pragma: no cover - link may be tearing down
            return

        if self._tf_publisher is not None:
            self._tf_publisher.publish(frame_id, pose)

    def stop(self) -> None:
        """Best-effort teardown of the NatNet sockets (threads are daemons)."""
        client = self._client
        self._client = None
        if client is None:
            return
        for sock_attr in ('dataSocket', 'commandSocket'):
            sock = getattr(client, sock_attr, None)
            if sock is not None:
                try:
                    sock.close()
                except Exception:  # pragma: no cover - best-effort teardown
                    pass
