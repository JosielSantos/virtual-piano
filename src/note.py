from collections import OrderedDict

import validate

class Note:
    def __init__(self, number):
        self.set_number(number)
        self.calc_octave()

    def get_number(self):
        return self.__number

    def set_number(self, number):
        validate.integer(number, 'Note number')
        validate.midi_range(number, 'Note number')
        self.__number = number

    def octave_down(self):
        number = self.__number - 12
        if number < 0:
            return False
        self.__number = number
        self.__octave -= 1
        return True

    def octave_up(self):
        number = self.__number + 12
        if number > 120:
            return False
        self.__number = number
        self.__octave += 1
        return True

    def get_octave(self):
        return self.__octave

    def calc_octave(self):
        self.__octave = self.__number // 12


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
            key = self.__key_list[i].upper()
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
        notes_changed = []
        changed_all_success = True
        for i in self:
            if self[i].octave_down():
                notes_changed.append(self[i])
            else:
                changed_all_success = False
                break
        if not changed_all_success:
            self.__rollback_octave_changes(notes_changed, 'up')

    def octave_up(self):
        notes_changed = []
        changed_all_success = True
        for i in self:
            if self[i].octave_up():
                notes_changed.append(self[i])
            else:
                changed_all_success = False
                break
        if not changed_all_success:
            self.__rollback_octave_changes(notes_changed, 'down')

    def __rollback_octave_changes(self, notes_changed, dir_to_back):
        method = 'octave_up' if dir_to_back == 'up' else 'octave_down'
        for note in notes_changed:
            getattr(note, method)()

    def __clean_key_list(self, key_list):
        return list(filter(
            lambda key: key != '' and not key.startswith('#'),
            map(lambda key: key.strip(), key_list)
        ))
