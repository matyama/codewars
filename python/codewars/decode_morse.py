from .morse import MORSE_CODE


# pylint: disable=invalid-name
def decodeMorse(morse_code: str) -> str:
    words = morse_code.strip().split('   ')

    def decode_word(word: str) -> str:
        return ''.join(MORSE_CODE.get(c, '').upper() for c in word.split())

    return ' '.join(map(decode_word, words))
