#I don't know a name for this function... Patience
def unicode_workaround(char):
    if len(char) == 1:
        return ord(char)
    sum = 0
    for i in range(len(char)):
        sum += char[i] if isinstance(char[i], int) else ord(char[i])
    return sum
