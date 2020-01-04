import pygame.midi

class Midi:
    def __init__(self):
        pygame.midi.init()

    def __del__(self):
        pygame.midi.quit()

    def get_default_output_id(self):
        return pygame.midi.get_default_output_id()


class Output(pygame.midi.Output):
    def note_on(self, note, volume, channel):
        super().note_on(note, volume, channel)

    def note_off(self, note, channel):
        super().note_off(note, 0, channel)

    def set_instrument(self, instrument_id, channel):
        super().set_instrument(instrument_id, channel)
