import pytest

from codewars.rm_calculator import calculate_1RM


@pytest.mark.parametrize(
    "w,r,expected",
    [
        (135, 20, 282),
        (200, 8, 253),
        (270, 2, 289),
        (360, 1, 360),
        (400, 0, 0),
    ],
)
def test_examples(w: float, r: int, expected: int) -> None:
    assert calculate_1RM(w, r) == expected
