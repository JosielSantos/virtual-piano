from os import path

from configparser import ConfigParser

import constants
from util import app

class Config:
    __filepath = None
    __config = None

    def __init__(self, filepath):
        self.__filepath = filepath
        self.__load_config()

    def get_midi_output_driver(self, default = constants.MIDI_OUTPUT_DEFAULT_DRIVER):
        return self.__config['midi'].getint('output_driver', default)

    def get_soundfont_file_path(self, default = None):
        return self.__config['soundfont'].get('file_path', default)

    def get_keymap_file_path(self, default):
        file_path = self.__config['general'].get('keymap_file_path', default)
        if path.isabs(file_path):
            return file_path
        return path.realpath(app.file_path(path.join('keymaps', file_path)))

    def __load_config(self):
        self.__config = ConfigParser()
        self.__config.read(self.__filepath)
