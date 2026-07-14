import time
import warnings

import torch

import cflib.crtp

from drone import CrazyflieDrone
from intercept_common import DroneState, CTBRCommand

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
    drone_config = {
        'name': 'pursuer',
        # 'uri': 'radio://0/80/2M/E7E7E7E701',
        'uri': 'udp://127.0.0.1:19850',
        'cache': './cache',
        'max_thrust_pwm': 65535.0,
        'rate_sign': [1.0, 1.0, 1.0],
        'need_rot_speed': False,

        'takeoff_duration': 2.0,
        'control_dt': 0.2,
    }

    cflib.crtp.init_drivers()

    drone = CrazyflieDrone(drone_config)
    drone.connect()
    time.sleep(2)  # Wait for connection
    drone.setup()  # Setup parameters and loggers
    drone.arm()
    drone.takeoff(0.5)  # Take off to 0.5 meter

    while drone.connected:
        try:
            time.sleep(drone.control_dt)
            state = drone.get_state()
            print(f"Drone state after takeoff: {state}")

            # dummy CTBR
            ctbr_command = CTBRCommand(
                body_rate_deg=torch.Tensor([0.0, 20.0, 0.0]),
                thrust_ratio=torch.Tensor([0.5]),
                thrust_pwm=torch.Tensor([50000])
            )

            drone.send_ctbr(ctbr_command)
        except KeyboardInterrupt:
            break

    drone.land()
    drone.disarm()
    drone.disconnect()
