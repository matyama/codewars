from codewars.prime import is_prime


def test_basics() -> None:
    assert not is_prime(0)
    assert not is_prime(1)
    assert is_prime(2)
    assert is_prime(73)
    assert not is_prime(75)
    assert not is_prime(-1)


def test_prime() -> None:
    assert is_prime(3)
    assert is_prime(5)
    assert is_prime(7)
    assert is_prime(41)
    assert is_prime(5099)


def test_not_prime() -> None:
    assert not is_prime(4)
    assert not is_prime(6)
    assert not is_prime(8)
    assert not is_prime(9)
    assert not is_prime(45)
    assert not is_prime(-5)
    assert not is_prime(-8)
    assert not is_prime(-41)
