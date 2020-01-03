import pygame.midi as midi

class Piano:
    def __init__(self):
        midi.init()
        self.output = midi.Output(midi.get_default_output_id(), 0)

    def __del__(self):
        del self.output
        midi.quit()

    def note_on(self, note, volume = 127, channel = 0):
        self.output.note_on(note, volume, channel)

    def note_off(self, note, volume = 127, channel = 0):
        self.output.note_off(note, volume, channel)

    def set_instrument(self, instrument_id, channel = 0):
        self.output.set_instrument(instrument_id, channel)
