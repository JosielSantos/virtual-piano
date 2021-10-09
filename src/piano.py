from channel import Channel

class Piano:
    channels = {}

    def __init__(self, output):
        self.output = output

    def __del__(self):
        del self.output

    def note_on(self, note, channel_number):
        channel = self.get_channel(channel_number)
        if note not in channel.notes_on:
            channel.note_on(note)
            self.output.note_on(note.get_number(), 127, channel_number)

    def note_off(self, note, channel_number):
        channel = self.get_channel(channel_number)
        if note in channel.notes_on:
            channel.note_off(note)
            self.output.note_off(note.get_number(), channel_number)

    def all_notes_off(self):
        for channel_number in range(len(self.channels)):
            for note_number in self.channels[channel_number].notes_on:
                self.note_off(note_number, channel_number)

    def next_instrument(self, channel_number):
        channel = self.get_channel(channel_number)
        channel.next_instrument()
        self.output.set_instrument(channel.get_instrument(), channel_number)

    def previous_instrument(self, channel_number):
        channel = self.get_channel(channel_number)
        channel.previous_instrument()
        self.output.set_instrument(channel.get_instrument(), channel_number)

    def set_instrument(self, instrument_id, channel_number):
        channel = self.get_channel(channel_number)
        channel.set_instrument(instrument_id)
        self.output.set_instrument(instrument_id, channel_number)

    def octave_down(self, channel_number):
        self.all_notes_off()
        channel = self.get_channel(channel_number)
        channel.notes_manager.octave_down()

    def octave_up(self, channel_number):
        self.all_notes_off()
        channel = self.get_channel(channel_number)
        channel.notes_manager.octave_up()

    def pan(self, back, channel_number):
        channel = self.get_channel(channel_number)
        direction = channel.get_direction()
        direction = direction -1 if back else direction + 1
        direction = -1 if direction > 1 else direction
        direction = 1 if direction < -1 else direction
        channel.set_direction(direction)
        if direction == channel.DIRECTION_LEFT:
            self.output.pan_left(channel_number)
        elif direction == channel.DIRECTION_MIDDLE:
            self.output.pan_middle(channel_number)
        else:
            self.output.pan_right(channel_number)

    def get_channel(self, channel_number):
        if channel_number not in self.channels:
            self.channels[channel_number] = Channel(0, 127)
        return self.channels[channel_number]

    def delete_channel(self, channel_number):
        self.all_notes_off()
        if channel_number > 0:
            self.set_instrument(0, channel_number)
            self.channels.pop(channel_number)

    def volume_down(self, channel_number):
        channel = self.get_channel(channel_number)
        channel.volume_down(10)
        self.output.set_channel_volume(channel.get_volume(), channel_number)

    def volume_up(self, channel_number):
        channel = self.get_channel(channel_number)
        channel.volume_up(10)
        self.output.set_channel_volume(channel.get_volume(), channel_number)

    def get_note(self, key, channel_number):
        return self.channels[channel_number].notes_manager[key] if key in self.channels[channel_number].notes_manager else None
