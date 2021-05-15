import string


def is_pangram(s: str) -> bool:
    letters = set(string.ascii_lowercase)
    found = {symbol for symbol in s.lower() if symbol in letters}
    return len(found) == len(letters)
