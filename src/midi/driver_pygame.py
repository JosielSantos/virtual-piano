import pygame.midi

from midi._abstracts import AbstractMidiOutput
from midi import _constants

class PygameMidiOutput(AbstractMidiOutput):
    def __init__(self, output_id = None):
        pygame.midi.init()
        if output_id is None:
            output_id = pygame.midi.get_default_output_id()
        self.__pygame = pygame.midi.Output(output_id, 0)

    def __del__(self):
        pygame.midi.quit()

    def note_on(self, note, volume = 127, channel_number = 0):
        self.__pygame.note_on(note, volume, channel_number)

    def note_off(self, note, channel_number = 0):
        self.__pygame.note_off(note, 0, channel_number)

    def set_instrument(self, instrument_id, channel_number = 0):
        self.__pygame.set_instrument(instrument_id, channel_number)

    def control_change(self, control, value, channel_number):
        self.__pygame.write_short(_constants.MIDI_MESSAGE_CONTROL_CHANGE + channel_number, control, value)
