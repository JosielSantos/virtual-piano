from midi import _constants

class AbstractMidiOutput:
    def note_on(self, note, volume = 127, channel_number = 0):
        raise NotImplementedError()

    def note_off(self, note, channel_number = 0):
        raise NotImplementedError()

    def set_instrument(self, instrument_id, channel_number = 0):
        raise NotImplementedError()

    def control_change(self, control, value, channel_number):
        raise NotImplementedError()

    def set_channel_volume(self, volume, channel_number = 0):
        self.control_change(_constants.CONTROL_CHANNEL_VOLUME , volume, channel_number)

    def pan_left(self, channel_number):
        self.control_change(_constants.CONTROL_CHANNEL_PAN, 0, channel_number)

    def pan_middle(self, channel_number):
        self.control_change(_constants.CONTROL_CHANNEL_PAN, 64, channel_number)

    def pan_right(self, channel_number):
        self.control_change(_constants.CONTROL_CHANNEL_PAN, 127, channel_number)
