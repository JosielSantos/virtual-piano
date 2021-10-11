import constants

class MidiOutputFactory:
    def factory_pygame(self, output_id):
        from midi import driver_pygame
        output_id = None if output_id == constants.MIDI_OUTPUT_DEFAULT_DRIVER else output_id
        return driver_pygame.PygameMidiOutput(output_id)

    def factory_fluidsynth(self, soundfont_filepath):
        from midi import driver_fluidsynth
        return driver_fluidsynth.FluidSynthMidiOutput(soundfont_filepath)
