import time
import warnings

import torch

import cflib.crtp

from drone import CrazyflieDrone
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
    # Example usage
    drone_config = ic.DroneConfig(
        name='pursuer',
        uri='udp://127.0.0.1:19850',
        # uri='radio://0/40/2M/E7E7E7E701',
        cache_dir='./cache',
        max_thrust_pwm=65535.0,
        rate_sign=[1.0, 1.0, 1.0],
        log_period_ms=20,
        takeoff_duration=2.0,
        control_dt=0.02,
        need_rot_speed=False
    )

    mocap_config = ic.MocapConfig(
        server_ip='192.168.0.210',
        local_ip=("" or None),
        multicast_address='239.255.42.99',
        command_port=1510,
        data_port=1511,
        body_to_flu_quat_xyzw=(0.0, 0.0, -0.7071067811865476, 0.7071067811865476),
    )
    cflib.crtp.init_drivers()

    drone = CrazyflieDrone(drone_config)
    mocap_tf_publisher = MocapTfPublisher(world_frame="world")
    mocap_receiver = MocapReceiver(mocap_config, tf_publisher=mocap_tf_publisher)

    drone.connect()
    time.sleep(2)  # Wait for connection
    drone.setup()  # Setup parameters and loggers

    mocap_receiver.register(rigid_body_id=31, drone=drone)
    mocap_receiver.start()  # Start receiving mocap data

    drone.arm()
    drone.takeoff(0.5)  # Take off to 0.5 meter

    while drone.connected:
        try:
            time.sleep(drone.control_dt)
            state = drone.get_state()
            print(f"Drone state after takeoff: {state}")

            # dummy CTBR
            ctbr_command = ic.CTBRCommand(
                body_rate_deg=torch.Tensor([0.0, 0.0, 0.0]),
                thrust_ratio=torch.Tensor([0.5]),
                thrust_pwm=torch.Tensor([50000])
            )

            drone.send_ctbr(ctbr_command)
        except KeyboardInterrupt:
            break

    drone.land()
    drone.disarm()
    drone.disconnect()
