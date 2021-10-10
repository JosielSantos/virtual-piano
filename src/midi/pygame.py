import pygame.midi

from midi._abstracts import AbstractMidiOutput

class PygameMidiOutput(AbstractMidiOutput):
    def __init__(self, output_id = None):
        pygame.midi.init()
        if output_id is None:
            output_id = pygame.midi.get_default_output_id()
        self.__handler = pygame.midi.Output(output_id, 0)

    def __del__(self):
        pygame.midi.quit()

    def note_on(self, note, volume = 127, channel_number = 0):
        self.__handler.note_on(note, volume, channel_number)

    def note_off(self, note, channel_number = 0):
        self.__handler.note_off(note, 0, channel_number)

    def set_instrument(self, instrument_id, channel_number = 0):
        self.__handler.set_instrument(instrument_id, channel_number)

    def control_change(self, control, value, channel_number):
        self.__handler.write_short(0xB0 + channel_number, control, value)
