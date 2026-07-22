import time
import warnings

import cflib.crtp

from drone import CrazyflieDrone, DronePosePublisher
from mocap import MocapReceiver, MocapTfPublisher
import intercept_common as ic

# Hide the known cflib warning when firmware still uses legacy hover packet type.
warnings.filterwarnings(
    'ignore',
    message=r'Using legacy TYPE_HOVER_LEGACY\. Please update your crazyflie-firmware\.',
    category=DeprecationWarning,
    module=r'cflib\.crazyflie\.commander',
)

warnings.filterwarnings(
    "ignore",
    message="The supervisor subsystem requires CRTP protocol version 12"
)

if __name__ == "__main__":
    drone_config = ic.load_drone_config_from_yaml('config.yaml', 'pursuer')
    mocap_config = ic.load_mocap_config_from_yaml('config.yaml')

    cflib.crtp.init_drivers()

    drone_pose_publisher = DronePosePublisher(world_frame="world")
    drone = CrazyflieDrone(drone_config, pose_publisher=drone_pose_publisher)
    mocap_tf_publisher = MocapTfPublisher(world_frame="world")
    mocap_receiver = MocapReceiver(mocap_config, tf_publisher=mocap_tf_publisher)

    try:
        drone.connect()
        drone.setup()

        # mocap_receiver.register(rigid_body_id=31, drone=drone)
        mocap_receiver.register_marker(marker_id=50002, drone=drone)
        mocap_receiver.start()

        drone.arm()
        drone.takeoff(0.75)
        position_hold = (0.0, 0.0, 0.75, 0.0)

        while drone.connected:
            try:
                state = drone.get_state()
                print(f"Drone state: {state}")

                drone.publish_pose()

                # dummy CTBR
                # ctbr_command = ic.CTBRCommand(
                #     body_rate_deg=ic.torch.Tensor([0.0, 0.0, 0.0]),
                #     thrust_ratio=ic.torch.Tensor([0.5]),
                #     thrust_pwm=ic.torch.Tensor([50000])
                # )

                # drone.send_ctbr(ctbr_command)

                # drone.send_position_setpoint(*position_hold)
                time.sleep(drone.control_dt)
            except KeyboardInterrupt:
                break
    finally:
        if drone.armed:
            drone.land()
            drone.disarm()

        mocap_tf_publisher.close()
        mocap_receiver.stop()
        drone.disconnect()
