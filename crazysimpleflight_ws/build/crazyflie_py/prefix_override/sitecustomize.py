import sys
if sys.prefix == '/usr':
    sys.real_prefix = sys.prefix
    sys.prefix = sys.exec_prefix = '/home/user/Projects/CrazySim/crazysimpleflight_ws/install/crazyflie_py'
