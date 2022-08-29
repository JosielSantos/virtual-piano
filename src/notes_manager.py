from collections import OrderedDict

from models.note import Note
from util import char, validate

class NotesManager(OrderedDict):
    '''A dictionary of notes. This class is responsible for associating keys with notes and apply transformations (octave change, for example) to all notes'''

    __start_note = 48

    def load_file(self, filename):
        with open(filename) as file:
            self.set_key_list(file.readlines())
        return True

    def organize_notes(self):
        start_note = self.get_start_note()
        for i in range(len(self.__key_list)):
            key = char.unicode_workaround(self.__key_list[i].upper())
            self[key] = Note(start_note + i)

    def get_start_note(self):
        return self.__start_note

    def set_start_note(self, start_note):
        validate.midi_range(start_note, 'Note number')
        self.__start_note = start_note
        return self

    def set_key_list(self, key_list):
        self.__key_list = self.__clean_key_list(key_list)

    def octave_down(self):
        return self.semitone_down(12)

    def octave_up(self):
        return self.semitone_up(12)

    def semitone_down(self, total):
        start_note = self.get_start_note()
        notes_changed = []
        changed_all_success = True
        for i in self:
            if self[i].semitone_down(total):
                notes_changed.append(self[i])
            else:
                changed_all_success = False
                break
        if not changed_all_success:
            self.__rollback_semitone_changes(total, notes_changed, 'up')
        else:
            start_note = notes_changed [0]
        self.set_start_note(start_note.get_number())
        return start_note

    def semitone_up(self, total):
        start_note = self.get_start_note()
        notes_changed = []
        changed_all_success = True
        for i in self:
            if self[i].semitone_up(total):
                notes_changed.append(self[i])
            else:
                changed_all_success = False
                break
        if not changed_all_success:
            self.__rollback_semitone_changes(total, notes_changed, 'down')
        else:
            start_note = notes_changed [0]
        self.set_start_note(start_note.get_number())
        return start_note

    def __rollback_semitone_changes(self, total, notes_changed, dir_to_back):
        method = 'semitone_up' if dir_to_back == 'up' else 'semitone_down'
        for note in notes_changed:
            getattr(note, method)(total)

    def __clean_key_list(self, key_list):
        return list(filter(
            lambda key: key != '' and not key.startswith('#'),
            map(lambda key: key.strip(), key_list)
        ))
