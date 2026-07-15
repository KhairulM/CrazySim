import threading
import time
import numpy as np
from dataclasses import dataclass
from abc import ABC, abstractmethod

from typing import Optional

from cflib.crazyflie import Crazyflie
from cflib.crazyflie.log import LogConfig
from cflib.positioning.motion_commander import MotionCommander

import intercept_common as ic

STAB_MODE_ROLL_PARAM = 'flightmode.stabModeRoll'
STAB_MODE_PITCH_PARAM = 'flightmode.stabModePitch'
STAB_ESTIMATOR_PARAM = 'stabilizer.estimator'
STAB_MODE_RATE = 0
STAB_MODE_ANGLE = 1
STAB_ESTIMATOR_DEFAULT = 1
STAB_ESTIMATOR_KALMAN = 2


class StateBuffer:
    """Accumulates the latest state from several cflib log blocks.

    Log callbacks run on a background thread, so all reads/writes go through a
    lock and consumers take an immutable :class:`ic.DroneState` snapshot.
    """

    def __init__(self) -> None:
        self._lock = threading.Lock()
        self._pos = np.zeros(3)
        self._vel = np.zeros(3)
        self._quat_wxyz = np.array([1.0, 0.0, 0.0, 0.0])
        self._ang_vel_body_rad = np.zeros(3)
        self._pos_stamp = 0.0
        self._att_stamp = 0.0

    def update_pos_vel(self, x, y, z, vx, vy, vz) -> None:
        with self._lock:
            self._pos = np.array([x, y, z], dtype=np.float64)
            self._vel = np.array([vx, vy, vz], dtype=np.float64)
            self._pos_stamp = time.time()

    def update_quat(self, qw, qx, qy, qz) -> None:
        with self._lock:
            self._quat_wxyz = np.array([qw, qx, qy, qz], dtype=np.float64)
            self._att_stamp = time.time()

    def update_gyro_deg(self, gx, gy, gz) -> None:
        with self._lock:
            self._ang_vel_body_rad = np.radians(
                np.array([gx, gy, gz], dtype=np.float64))

    def snapshot(self) -> Optional[ic.DroneState]:
        """Return the latest state, or ``None`` if no pose has arrived yet."""
        with self._lock:
            if self._pos_stamp == 0.0 or self._att_stamp == 0.0:
                return None
            rot = ic.quaternion_to_rotation_matrix_np(self._quat_wxyz)
            ang_vel_world = rot @ self._ang_vel_body_rad
            return ic.DroneState(
                pos=self._pos.copy(),
                quat_wxyz=self._quat_wxyz.copy(),
                lin_vel=self._vel.copy(),
                ang_vel=ang_vel_world,
                stamp=min(self._pos_stamp, self._att_stamp),
            )


class Drone(ABC):

    @abstractmethod
    def connect(self):
        pass

    @abstractmethod
    def setup(self):
        pass

    @abstractmethod
    def disconnect(self):
        pass

    @abstractmethod
    def arm(self):
        pass

    @abstractmethod
    def disarm(self):
        pass

    @abstractmethod
    def takeoff(self, height):
        pass

    @abstractmethod
    def land(self):
        pass

    @abstractmethod
    def get_state(self) -> Optional[ic.DroneState]:
        pass

    @abstractmethod
    def set_param(self, group, name, value):
        pass

    @abstractmethod
    def send_mocap_pose(self, pose: ic.MocapPose):
        pass

    @abstractmethod
    def send_ctbr(self, command: ic.CTBRCommand):
        pass


class CrazyflieDrone(Drone):
    def __init__(self, drone_config: ic.DroneConfig):
        self.name = drone_config.name
        self.uri = drone_config.uri
        self.cache = drone_config.cache_dir
        self.drone_config = drone_config

        self.max_thrust_pwm = drone_config.max_thrust_pwm
        self.rate_sign = np.array(drone_config.rate_sign)
        self.takeoff_duration = drone_config.takeoff_duration
        self.control_dt = drone_config.control_dt
        self.log_period_ms = max(10, int(drone_config.log_period_ms))

        self.cf = Crazyflie(rw_cache=self.cache)
        self.state = StateBuffer()
        self.connected = False

    def connect(self):
        self.cf.connected.add_callback(self._on_connected)
        self.cf.connection_lost.add_callback(self._on_disconnected)
        self.cf.open_link(self.uri)

    def disconnect(self):
        self._on_disconnected(self.uri)
        self.cf.close_link()

    def arm(self):
        if not self.connected:
            print(f'[{self.name}] Cannot arm, not connected to {self.uri}')
            return

        self.cf.supervisor.send_arming_request(True)
        time.sleep(1.0)

        # The first setpoint must be a zero-thrust one to unlock the commander.
        self.cf.commander.send_setpoint(0.0, 0.0, 0.0, 0)
        time.sleep(0.1)

        print(f'[{self.name}] Armed. Commander unlocked. Ready to take off.')

    def disarm(self):
        if not self.connected:
            print(f'[{self.name}] Cannot disarm, not connected to {self.uri}')
            return
        self.cf.supervisor.send_arming_request(False)
        time.sleep(1.0)

        print(f'[{self.name}] Disarmed. Commander locked. Motors stopped.')

    def takeoff(self, height):
        if not self.connected:
            print(f'[{self.name}] Cannot takeoff, not connected to {self.uri}')
            return

        steps = max(1, int(self.takeoff_duration / self.control_dt))
        for _ in range(steps):
            self.cf.commander.send_hover_setpoint(0.0, 0.0, 0.0, height)
            time.sleep(self.control_dt)

    def land(self):
        if not self.connected:
            print(f'[{self.name}] Cannot land, not connected to {self.uri}')
            return

        print(f'[{self.name}] Sending zero-thrust setpoint for landing...')
        self.cf.commander.send_setpoint(0.0, 0.0, 0.0, 0)
        time.sleep(1.0)

    def get_state(self) -> Optional[ic.DroneState]:
        return self.state.snapshot()

    def set_param(self, group, name, value):
        self.cf.param.set_value(f'{group}.{name}', value)

    def send_mocap_pose(self, pose: ic.MocapPose):
        px, py, pz = pose.position
        qx, qy, qz, qw = pose.quat_xyzw
        try:
            self.cf.extpos.send_extpose(px, py, pz, qx, qy, qz, qw)
        except Exception:  # pragma: no cover - link may be tearing down
            print(f'[{self.name}] Error sending mocap pose to {self.uri}')

    def send_ctbr(self, command: ic.CTBRCommand):
        rates = command.body_rate_deg.detach().cpu().numpy().reshape(-1)  # deg/s
        sign = self.rate_sign

        roll_rate = float(sign[0]) * float(rates[0])
        pitch_rate = float(sign[1]) * float(rates[1])
        yaw_rate = float(sign[2]) * float(rates[2])
        thrust = int(np.clip(
            float(command.thrust_pwm.detach().cpu().item()),
            0.0, self.max_thrust_pwm))
        self.cf.commander.send_setpoint(roll_rate, pitch_rate, yaw_rate, thrust)

    def setup(self):
        self._setup_params()
        self._setup_loggers()

    def _on_connected(self, link_uri):
        print(f'[{self.name}] Connected to {link_uri}')
        self.connected = True

    def _on_disconnected(self, link_uri):
        print(f'[{self.name}] Disconnected from {link_uri}')
        try:
            self.cf.commander.send_stop_setpoint()
            self.cf.commander.send_notify_setpoint_stop()
        except Exception:
            print(f'[{self.name}] Error sending stop setpoint to {link_uri}')
        self.connected = False

    def _setup_params(self):
        # Set the stabilizer mode to angle mode for roll and pitch
        self.cf.param.set_value(STAB_MODE_ROLL_PARAM, STAB_MODE_RATE)
        self.cf.param.set_value(STAB_MODE_PITCH_PARAM, STAB_MODE_RATE)
        # Set the stabilizer estimator to Kalman filter
        self.cf.param.set_value(STAB_ESTIMATOR_PARAM, STAB_ESTIMATOR_KALMAN)

    def _setup_loggers(self):
        pos_log = LogConfig(name='pos_vel', period_in_ms=self.log_period_ms)
        for var in ('stateEstimate.x', 'stateEstimate.y', 'stateEstimate.z',
                    'stateEstimate.vx', 'stateEstimate.vy', 'stateEstimate.vz'):
            pos_log.add_variable(var, 'float')
        pos_log.data_received_cb.add_callback(self._pos_vel_cb)

        att_log = LogConfig(name='attitude', period_in_ms=self.log_period_ms)
        for var in ('stateEstimate.qw', 'stateEstimate.qx', 'stateEstimate.qy', 'stateEstimate.qz'):
            att_log.add_variable(var, 'float')
        att_log.data_received_cb.add_callback(self._att_cb)

        self.cf.log.add_config(pos_log)
        self.cf.log.add_config(att_log)

        pos_log.start()
        att_log.start()

        if self.drone_config.need_rot_speed:
            gyro_log = LogConfig(name='gyro', period_in_ms=self.log_period_ms)
            for var in ('gyro.x', 'gyro.y', 'gyro.z'):
                gyro_log.add_variable(var, 'float')
            gyro_log.data_received_cb.add_callback(self._gyro_cb)
            self.cf.log.add_config(gyro_log)
            gyro_log.start()

    def _pos_vel_cb(self, timestamp, data, logconf_name):
        self.state.update_pos_vel(
            data['stateEstimate.x'],
            data['stateEstimate.y'],
            data['stateEstimate.z'],
            data['stateEstimate.vx'],
            data['stateEstimate.vy'],
            data['stateEstimate.vz'],
        )

    def _att_cb(self, timestamp, data, logconf_name):
        self.state.update_quat(
            data['stateEstimate.qw'],
            data['stateEstimate.qx'],
            data['stateEstimate.qy'],
            data['stateEstimate.qz'],
        )

    def _gyro_cb(self, timestamp, data, logconf_name):
        self.state.update_gyro_deg(
            data['gyro.x'],
            data['gyro.y'],
            data['gyro.z'],
        )
