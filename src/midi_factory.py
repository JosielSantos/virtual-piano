import constants
from midi import driver_fluidsynth, driver_pygame

class MidiOutputFactory:
    def factory_pygame(self, output_id):
        output_id = None if output_id == constants.MIDI_OUTPUT_DEFAULT_DRIVER else output_id
        return driver_pygame.PygameMidiOutput(output_id)

    def factory_fluidsynth(self, soundfont_filepath):
        return driver_fluidsynth.FluidSynthMidiOutput(soundfont_filepath)
