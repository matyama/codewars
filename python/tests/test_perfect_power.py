from math import floor, log
from random import random, randrange

from codewars.perfect_power import isPP


def test_simple_cases() -> None:
    assert isPP(4) == [2, 2]
    assert isPP(9) == [3, 2]
    assert isPP(5) is None


def test_first_perfect_powers() -> None:
    powers = [
        4,
        8,
        9,
        16,
        25,
        27,
        32,
        36,
        49,
        64,
        81,
        100,
        121,
        125,
        128,
        144,
        169,
        196,
        216,
        225,
        243,
        256,
        289,
        324,
        343,
        361,
        400,
        441,
        484,
    ]
    for n in powers:
        assert isPP(n) is not None


def test_random_perfect_powers() -> None:
    for _ in range(100):
        m = 2 + floor(random() * 255)
        k = 2 + floor(random() * log(268435455) / log(m))

        n = m ** k
        result = isPP(n)

        if result is None:
            assert result is not None

        res_m, res_k = result
        res_n = res_m ** res_k
        if res_n != n:
            assert res_n == n


def test_valid_pairs_for_random_inputs() -> None:
    for _ in range(100):
        n = randrange(65535)
        result = isPP(n)
        if result is not None:
            m, k = result
            assert m ** k == n
