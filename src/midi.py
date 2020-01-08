import pygame.midi

class Midi:
    def __init__(self):
        pygame.midi.init()

    def __del__(self):
        pygame.midi.quit()

    def get_default_output_id(self):
        return pygame.midi.get_default_output_id()


class Output(pygame.midi.Output):
    def note_on(self, note, volume = 127, channel_number = 0):
        super().note_on(note, volume, channel_number)

    def note_off(self, note, channel_number = 0):
        super().note_off(note, 0, channel_number)

    def set_instrument(self, instrument_id, channel_number = 0):
        super().set_instrument(instrument_id, channel_number)

    def set_channel_volume(self, volume, channel_number = 0):
        self.control_change(7, volume, channel_number)

    def control_change(self, control, value, channel_number):
        super().write_short(0xB0 + channel_number, control, value)
