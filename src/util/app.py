from os import path
import sys

def base_dir():
    frozen = getattr(sys, 'frozen', False)
    if frozen:
        return path.dirname(path.abspath(__file__))
    return path.dirname(path.dirname(path.dirname(path.abspath(__file__))))

def file_path(name):
    return path.join(base_dir(), name)
