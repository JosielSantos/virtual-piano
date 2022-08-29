import os.path
from notes_manager import NotesManager
from util import app, validate

class Channel:
    DIRECTION_LEFT = -1
    DIRECTION_MIDDLE = 0
    DIRECTION_RIGHT = 1

    notes_on = []
    __notes_manager = None
    __start_note = 48

    def __init__(self, instrument, volume, direction, keymap_file_path):
        self.set_instrument(instrument)
        self.set_volume(volume)
        self.set_direction(direction)
        self.keymap_file_path = keymap_file_path
        self.__load_notes_manager()

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

    def get_direction(self):
        return self.__direction

    def set_direction(self, direction):
        if direction not in [self.DIRECTION_LEFT, self.DIRECTION_MIDDLE, self.DIRECTION_RIGHT]:
            raise ValueError('Invalid direction')
        self.__direction = direction

    def __load_notes_manager(self):
        self.__notes_manager = NotesManager()
        self.__notes_manager.load_file(self.keymap_file_path)
        self.__notes_manager.set_start_note(self.__start_note)
        self.__notes_manager.organize_notes()

    def note_on(self, note):
        if note not in self.notes_on:
            self.notes_on.append(note)

    def note_off(self, note):
        if note in self.notes_on:
            self.notes_on.remove(note)

    def next_instrument(self):
        instrument = self.__instrument + 1
        if instrument > 127:
            instrument = 0
        self.__instrument = instrument

    def previous_instrument(self):
        instrument = self.__instrument - 1
        if instrument < 0 :
            instrument = 127
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

    def get_note(self, key):
        return self.__notes_manager[key] if key in self.__notes_manager else None

    def octave_down(self):
        self.__start_note = self.__notes_manager.octave_down()
        return self.__start_note

    def octave_up(self):
        self.__start_note = self.__notes_manager.octave_up()
        return self.__start_note

    def semitone_down(self, total):
        self.__start_note = self.__notes_manager.semitone_down(total)
        return self.__start_note

    def semitone_up(self, total):
        self.__start_note = self.__notes_manager.semitone_up(total)
        return self.__start_note

    def get_start_note(self):
        return self.__notes_manager.get_start_note()
