import os.path
import sys

def get_app_dir():
    frozen = getattr(sys, 'frozen', False)
    if frozen:
        return os.path.dirname(os.path.abspath(__file__))
    return os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
