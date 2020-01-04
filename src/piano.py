class Piano:
    def __init__(self, output):
        self.output = output

    def __del__(self):
        del self.output

    def note_on(self, note, volume , channel):
        self.output.note_on(note, volume, channel)

    def note_off(self, note, channel):
        self.output.note_off(note, channel)

    def set_instrument(self, instrument_id, channel):
        self.output.set_instrument(instrument_id, channel)
