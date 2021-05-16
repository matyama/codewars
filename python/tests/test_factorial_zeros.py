from codewars.factorial_zeros import zeros


def test_trailing_zeros() -> None:
    assert zeros(0) == 0
    assert zeros(6) == 1
    assert zeros(30) == 7
    assert zeros(1000) == 249
