import validate

class Note:
    def __init__(self, number):
        self.set_number(number)

    def get_number(self):
        return self.__number

    def set_number(self, number):
        validate.integer(number, 'Note number')
        validate.midi_range(number, 'Note number')
        self.__number = number

    def semitone_down(self, total):
        number = self.__number - total
        if number < 0:
            return False
        self.__number = number
        return True

    def semitone_up(self, total):
        number = self.__number + total
        if number > 120:
            return False
        self.__number = number
        return True
