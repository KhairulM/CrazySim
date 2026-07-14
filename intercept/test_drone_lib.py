import time

from drone import CrazyflieDrone
from intercept_common import DroneState

if __name__ == "__main__":
    # Example usage
    drone_config = {
        'name': 'Drone1',
        'uri': 'radio://0/80/2M/E7E7E7E701',
        'cache': './cache',
        'max_thrust_pwm': 65535.0,
        'rate_sign': [1.0, 1.0, 1.0],
        'need_rot_speed': False,
    }

    drone = CrazyflieDrone(drone_config)
    drone.connect()
    time.sleep(2)  # Wait for connection
    drone.arm()
    drone.takeoff(1.0)  # Take off to 1 meter
    state = drone.get_state()
    print(f"Drone state after takeoff: {state}")
    time.sleep(5)  # Hover for 5 seconds
    drone.land()
    drone.disarm()
    drone.disconnect()
