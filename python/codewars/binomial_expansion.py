import re
from math import comb
from typing import NamedTuple, cast

EXPR_PATTERN = re.compile(
    r"\((?P<m>-)?(?P<a>\d+)?(?P<x>[a-zA-z])(?P<o>[+-])(?P<b>\d+)\)\^(?P<n>\d+)"
)


class Expr(NamedTuple):
    a: int
    x: str
    b: int
    n: int


def parse(expr: str) -> Expr:
    """Parse an expression of the form: `(ax+b)^n`"""
    match = EXPR_PATTERN.match(expr)
    assert match is not None, f"invalid expression: {expr}"

    a = int(match.group("a") or "1")
    if match.group("m") is not None:
        a = -a

    x = match.group("x")
    assert x is not None

    b = int(match.group("b") or "0")
    if match.group("o") == "-":
        b = -b

    n = int(match.group("n") or "1")

    return Expr(a, x, b, n)


def expand(expr: str) -> str:
    a, x, b, n = parse(expr)

    def term(k: int, c: int, unary_plus: bool = False) -> str:
        if c == 0 or k == n:
            t = str(c)
        elif c == 1:
            t = x
        elif c == -1:
            t = f"-{x}"
        else:
            t = f"{c}{x}"
        if n - k > 1:
            t = f"{t}^{n - k}"
        return f"+{t}" if unary_plus and c > 0 else t

    if a == 0:
        return str(b**n)

    if b == 0:
        return term(k=0, c=a**n)

    def coef(k: int) -> int:
        return cast(int, comb(n, k) * (a ** (n - k)) * (b**k))

    coefs = map(coef, range(n + 1))
    return "".join(term(k, c, unary_plus=k > 0) for k, c in enumerate(coefs))
