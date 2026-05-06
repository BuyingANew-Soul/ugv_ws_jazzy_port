import sys
if sys.prefix == '/usr':
    sys.real_prefix = sys.prefix
    sys.prefix = sys.exec_prefix = '/home/nameless/ugv_ws_jazzy_port/install/ugv_vision'
