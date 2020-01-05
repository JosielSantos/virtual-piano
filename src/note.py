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
        if number > 127:
            return False
        self.__number = number
        self.__octave += 1
        return True

    def get_octave(self):
        return self.__octave

    def calc_octave(self):
        self.__octave = self.__number // 12
