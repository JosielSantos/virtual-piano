import constants
from midi import pygame

class MidiOutputFactory:
    def factory_pygame(self, output_id):
        output_id = None if output_id == constants.MIDI_OUTPUT_DEFAULT_DRIVER else output_id
        return pygame.PygameMidiOutput(output_id)
