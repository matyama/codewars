from collections.abc import Iterable
from functools import partial
from typing import cast


def parse_int(string: str) -> int:
    """
    Converts a string representing a number in words into an integer.

    The input is assumed to always represent a valid number in the range
    between zero and one million.
    """

    tokens = string.split()
    tokens = [t for ts in map(normalize, tokens) for t in ts]

    parse_next = partial(parse, tokens)

    orders = (0, 1, 2, 3, 6)
    return sum(map(parse_next, orders))


NUMS = {
    "zero": 0,
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
}

KEYWORDS = {0: "#", 1: "ten", 2: "hundred", 3: "thousand", 6: "million"}


def parse(ts: list[str], order: int) -> int:
    token = KEYWORDS[order]

    if order == 0 and ts and ts[-1] in NUMS:
        # pad numbers without correspoding keywords to treat them equally
        ts.append(token)

    if ts and ts[-1] == token:
        # pop a keyword
        ts.pop()
        if ts:
            if order < 3:
                # parse directly for < 100, allowing standalone keywords
                n = NUMS[ts.pop()] if ts[-1] in NUMS else 1
            else:
                # parse by hundreds for orders of thousand and larger
                n = sum(parse(ts, order=i) for i in range(3))
        else:
            # allow standalone keywords (e.g. "ten" instead of "one ten")
            n = 1 if order > 0 else 0
    else:
        # parts might be skipped in the textual representation
        n = 0

    return cast(int, n * 10**order)


def normalize(token: str) -> Iterable[str]:
    match token:
        case "and":
            return
        case "eleven":
            yield from ("one", "ten", "one")
        case "twelve":
            yield from ("one", "ten", "two")
        case "thirteen":
            yield from ("one", "ten", "three")
        case "fifteen":
            yield from ("one", "ten", "five")
        case "eighteen":
            yield from ("one", "ten", "eight")
        case "twenty":
            yield from ("two", "ten")
        case "thirty":
            yield from ("three", "ten")
        case "forty":
            yield from ("four", "ten")
        case "fifty":
            yield from ("five", "ten")
        case "eighty":
            yield from ("eight", "ten")
        case token if token.endswith("ty"):
            yield from (token.removesuffix("ty"), "ten")
        case token if token.endswith("teen"):
            yield from ("one", "ten", token.removesuffix("teen"))
        case token if "-" in token:
            a, b = token.split("-")
            yield from normalize(a)
            yield from normalize(b)
        case token:
            yield from token.split()
