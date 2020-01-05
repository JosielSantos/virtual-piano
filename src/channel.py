import validate

class Channel:
    def __init__(self, instrument = 0, volume = 127):
        self.set_instrument(instrument)
        self.set_volume(volume)

    def get_instrument(self):
        return self.__instrument

    def set_instrument(self, instrument):
        validate.integer(instrument, 'Instrument')
        validate.midi_range(instrument, 'Instrument')
        self.__instrument = instrument

    def get_volume(self):
        return self.__volume

    def set_volume(self, volume):
        validate.integer(volume, 'Volume')
        validate.midi_range(volume, 'Volume')
        self.__volume = volume

    def next_instrument(self):
        instrument = self.__instrument + 1
        if instrument > 127:
            return
        self.__instrument = instrument

    def previous_instrument(self):
        instrument = self.__instrument - 1
        if instrument < 0 :
            return
        self.__instrument = instrument

    def volume_down(self, num = 10):
        volume = self.__volume - num
        if volume < 0:
            volume = 0
        self.__volume = volume

    def volume_up(self, num = 10):
        volume = self.__volume + num
        if volume > 127:
            volume = 127
        self.__volume = volume
