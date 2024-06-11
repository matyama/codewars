from .morse import MORSE_CODE

WORD_SEP = "   "
CHAR_SEP = " "


def time_unit(bits: str) -> int:
    if "0" not in bits:
        return len(bits)

    # Find lengths of consecutive substrings of 0s
    pauses = {len(part) for part in bits.split("1") if part}
    signals = {len(sig) for sig in bits.split("0") if sig}

    # If all sequences have the same lenght, they must separate chars
    if len(pauses) == 1:
        length = next(iter(pauses))
        signal = next(iter(signals))
        if signal == length:
            return signal

        return length // 3 if length % 3 == 0 else length

    # Determine unit by the longest pause separating either words or letters
    for i in (7, 3):
        longest = max((p for p in pauses if p % i == 0), default=None)
        if longest is not None:
            return longest // i

    return 1


def decode_bits(bits: str) -> str:
    # Get rid of leading and trailing 0s
    bits = bits.strip("0")

    # Figure out the time unit and message chunks
    unit = time_unit(bits)

    word_sep = "0" * 7 * unit
    char_sep = "0" * 3 * unit
    pause_sep = "0" * unit
    dash = "1" * 3 * unit

    def decode_char(char_bits: str) -> str:
        return "".join(
            "-" if c == dash else "." for c in char_bits.split(pause_sep) if c
        )

    def decode_word(w: str) -> str:
        return CHAR_SEP.join(decode_char(c) for c in w.split(char_sep) if c)

    return WORD_SEP.join(decode_word(w) for w in bits.split(word_sep) if w)


def decode_morse(morse_code: str) -> str:
    def decode_word(w: str) -> str:
        return "".join(MORSE_CODE[c].upper() for c in w.split(CHAR_SEP))

    return " ".join(decode_word(w) for w in morse_code.strip().split(WORD_SEP))
