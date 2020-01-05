def midi_range(number, type = 'number'):
    if number < 0 or number > 127:
        raise ValueError('%s must be between 0 and 127. %d given.' %(type, number))

def integer(subject, type = 'str'):
    if not isinstance(subject, int):
        raise ValueError('%s must be integer.' %(type))
