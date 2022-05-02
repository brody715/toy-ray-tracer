import sys
import os

__script_dir = os.path.dirname(__file__)
__root_dir = os.path.realpath(os.path.join(__script_dir, '..'))

sys.path.append(__root_dir)
