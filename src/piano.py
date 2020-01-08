from channel import Channel

class Piano:
    current_channel = 0
    channels = {}
    notes_on = {}
    start_note = 48

    def __init__(self, notes_manager, output):
        self.notes_manager = notes_manager
        self.output = output
        self.organize_notes()

    def __del__(self):
        del self.output

    def organize_notes(self):
        self.notes_manager.set_start_note(self.start_note)
        self.notes_manager.organize_notes()

    def note_on(self, note, channel_number = None):
        if channel_number is None:
            channel_number = self.current_channel
        channel = self.get_channel(channel_number)
        if note not in self.notes_on[channel_number]:
            self.notes_on[channel_number].append(note)
            self.output.note_on(note.get_number(), 127, channel_number)

    def note_off(self, note, channel_number = None):
        if channel_number is None:
            channel_number = self.current_channel
        channel = self.get_channel(channel_number)
        if note in self.notes_on[channel_number]:
            self.notes_on[channel_number].remove(note)
            self.output.note_off(note.get_number(), channel_number)

    def note_on_multi(self, note):
        for channel_number in range(len(self.channels)):
            self.note_on(note, channel_number)

    def note_off_multi(self, note):
        for channel_number in range(len(self.channels)):
            self.note_off(note, channel_number)

    def all_notes_off(self):
        for channel_number in self.notes_on:
            for note_number in self.notes_on[channel_number]:
                self.note_off(note_number, channel_number)

    def next_instrument(self):
        channel = self.get_channel(self.current_channel)
        channel.next_instrument()
        self.output.set_instrument(channel.get_instrument(), self.current_channel)

    def previous_instrument(self):
        channel = self.get_channel(self.current_channel)
        channel.previous_instrument()
        self.output.set_instrument(channel.get_instrument(), self.current_channel)

    def set_instrument(self, instrument_id, channel_number):
        channel = self.get_channel(channel_number)
        channel.set_instrument(instrument_id)
        self.output.set_instrument(instrument_id, channel_number)

    def octave_down(self):
        self.all_notes_off()
        self.notes_manager.octave_down()

    def octave_up(self):
        self.all_notes_off()
        self.notes_manager.octave_up()

    def next_channel(self):
        if self.current_channel == 15:
            return
        self.all_notes_off()
        self.current_channel += 1
        self.get_channel(self.current_channel)

    def previous_channel(self):
        if self.current_channel == 0:
            return
        self.all_notes_off()
        self.current_channel -= 1
        self.get_channel(self.current_channel)

    def get_channel(self, channel_number):
        if channel_number not in self.channels:
            self.notes_on[channel_number] = []
            self.channels[channel_number] = Channel(0, 127)
        return self.channels[channel_number]

    def delete_current_channel(self):
        self.all_notes_off()
        if self.current_channel > 0:
            self.set_instrument(0, self.current_channel)
            self.channels.pop(self.current_channel)
            self.previous_channel()

    def volume_down(self):
        channel = self.get_channel(self.current_channel)
        channel.volume_down(10)
        self.output.set_channel_volume(channel.get_volume(), self.current_channel)

    def volume_up(self):
        channel = self.get_channel(self.current_channel)
        channel.volume_up(10)
        self.output.set_channel_volume(channel.get_volume(), self.current_channel)
