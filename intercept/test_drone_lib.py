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
    drone_config = ic.load_drone_config_from_yaml('config.yaml', 'pursuer')
    mocap_config = ic.load_mocap_config_from_yaml('config.yaml')

    cflib.crtp.init_drivers()

    drone = CrazyflieDrone(drone_config)
    mocap_tf_publisher = MocapTfPublisher(world_frame="world")
    mocap_receiver = MocapReceiver(mocap_config, tf_publisher=mocap_tf_publisher)

    drone.connect()
    time.sleep(2)  # Wait for connection
    drone.setup()

    mocap_receiver.register(rigid_body_id=31, drone=drone)
    mocap_receiver.start()

    drone.arm()
    drone.takeoff(0.5)

    while drone.connected:
        try:
            time.sleep(drone.control_dt)
            state = drone.get_state()
            print(f"Drone state: {state}")

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
