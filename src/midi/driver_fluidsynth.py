import fluidsynth
from midi._abstracts import AbstractMidiOutput

class FluidSynthMidiOutput(AbstractMidiOutput):
    __synth = None

    def __init__(self, soundfont_filepath):
        self.soundfont_filepath = soundfont_filepath
        self.__init_fluidsynth()

    def __del__(self):
        if self.__synth is not None:
            self.__synth.delete()

    def note_on(self, note, volume = 127, channel_number = 0):
        self.__synth.noteon(channel_number, note, volume)

    def note_off(self, note, channel_number = 0):
        self.__synth.noteoff(channel_number, note)

    def set_instrument(self, instrument_id, channel_number = 0):
        self.__synth.program_change(channel_number, instrument_id)

    def control_change(self, control, value, channel_number):
        self.__synth.cc(channel_number, control, value)

    def __init_fluidsynth(self):
        self.__synth = fluidsynth.Synth()
        self.__synth.start()
        self.__soundfont_id = self.__synth.sfload(self.soundfont_filepath)
