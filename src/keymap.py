from collections import OrderedDict

class Keymap(OrderedDict):
    start_note = 48
    key_list = []

    def load_file(self, filename):
        with open(filename) as file:
            self.set_key_list(file.readlines())
        return True

    def organize_notes(self):
        start_note = self.get_start_note()
        key_list = self.get_key_list()
        for i in range(len(key_list)):
            key = key_list[i].upper()
            self[key] = start_note + i

    def get_start_note(self):
        return self.start_note

    def set_start_note(self, start_note):
        self.start_note = start_note
        return self

    def get_key_list(self):
        return self.key_list

    def set_key_list(self, key_list):
        self.key_list = self.clean_key_list(key_list)
        return self

    def clean_key_list(self, key_list):
        return list(filter(
            lambda key: key != '' and not key.startswith('#'),
            map(lambda key: key.strip(), key_list)
        ))
