class Piano:
    current_channel = 0
    channels = {}
    notes_on = {}
    start_note = 48

    def __init__(self, keymap, output):
        self.keymap = keymap
        self.output = output
        self.organize_notes()

    def __del__(self):
        del self.output

    def organize_notes(self):
        self.keymap.set_start_note(self.start_note)
        self.keymap.organize_notes()

    def note_on(self, note, channel = None):
        if channel is None:
            channel = self.current_channel
        self.ensure_channel(channel)
        if note not in self.notes_on[channel]:
            self.notes_on[channel].append(note)
            self.output.note_on(note, self.channels[channel]['volume'], channel)

    def note_off(self, note, channel = None):
        if channel is None:
            channel = self.current_channel
        self.ensure_channel(channel)
        if note in self.notes_on[channel]:
            self.notes_on[channel].remove(note)
            self.output.note_off(note, channel)

    def note_on_multi(self, note):
        for channel in range(len(self.channels)):
            self.note_on(note, channel)

    def note_off_multi(self, note):
        for channel in range(len(self.channels)):
            self.note_off(note, channel)

    def all_notes_off(self):
        for channel in self.notes_on:
            for note in self.notes_on[channel]:
                self.note_off(note, channel)

    def octave_down(self):
        if self.start_note == 0:
            return
        self.all_notes_off()
        self.start_note -= 12
        self.organize_notes()

    def octave_up(self):
        if self.start_note == 120:
            return
        self.all_notes_off()
        self.start_note += 12
        self.organize_notes()

    def next_instrument(self):
        if self.channels[self.current_channel]['instrument'] == 127:
            return
        self.set_instrument(self.channels[self.current_channel]['instrument'] + 1, self.current_channel)

    def previous_instrument(self):
        if self.channels[self.current_channel]['instrument'] == 0:
            return
        self.set_instrument(self.channels[self.current_channel]['instrument'] - 1, self.current_channel)

    def set_instrument(self, instrument_id, channel):
        self.ensure_channel(channel)
        self.channels[channel]['instrument'] = instrument_id
        self.output.set_instrument(instrument_id, channel)

    def next_channel(self):
        if self.current_channel == 15:
            return
        self.all_notes_off()
        self.current_channel += 1
        self.ensure_channel(self.current_channel)

    def previous_channel(self):
        if self.current_channel == 0:
            return
        self.all_notes_off()
        self.current_channel -= 1
        self.ensure_channel(self.current_channel)

    def ensure_channel(self, num):
        if num not in self.channels:
            self.notes_on[num] = []
            self.channels[num] = self.make_channel(0, 127)
        return self.channels[num]

    def make_channel(self, instrument_id, volume = 127):
        return {
            'instrument': instrument_id,
            'volume': volume,
        }

    def delete_current_channel(self):
        self.all_notes_off()
        if self.current_channel > 0:
            self.channels.pop(self.current_channel)
            self.previous_channel()

    def volume_down(self):
        if self.channels[self.current_channel]['volume'] == 0:
            return
        if self.channels[self.current_channel]['volume'] < 10:
            self.channels[self.current_channel]['volume'] = 0
        else:
            self.channels[self.current_channel]['volume'] -= 10

    def volume_up(self):
        if self.channels[self.current_channel]['volume'] == 127:
            return
        if self.channels[self.current_channel]['volume'] >= 118:
            self.channels[self.current_channel]['volume'] = 127
        else:
            self.channels[self.current_channel]['volume'] += 10
