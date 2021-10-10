from configparser import ConfigParser

import constants

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

    def __load_config(self):
        self.__config = ConfigParser()
        self.__config.read(self.__filepath)
