from typing import Sequence


def likes(names: Sequence[str]) -> str:
    names = ["no one"] if not names else names

    if len(names) < 2:
        return f"{names[0]} likes this"

    if len(names) < 3:
        fst, snd = names
        return f"{fst} and {snd} like this"

    initial, rest = names[:2], names[2:]
    fst, snd = initial
    other = f"{len(rest)} others" if len(rest) > 1 else rest[0]
    return f"{fst}, {snd} and {other} like this"
