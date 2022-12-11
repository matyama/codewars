import pytest

from codewars.parse_int import parse_int


@pytest.mark.parametrize(
    "string,expected",
    [
        ("", 0),
        ("zero", 0),
        ("one", 1),
        ("ten", 10),
        ("eighteen", 18),
        ("twenty", 20),
        ("hundred", 100),
        ("one hundred", 100),
        ("two hundred forty-six", 246),
        ("five thousand fifteen", 5_015),
        ("four thousand five hundred and one", 4_501),
        ("fourty-three thousand five hundred and one", 43_501),
        (
            "six hundred ninety-one thousand five hundred and eighty-two",
            691_582,
        ),
        (
            "seven hundred eighty-three thousand nine hundred and nineteen",
            783919,
        ),
        ("six hundred and forty-seven thousand six hundred ten", 647_610),
        ("hundred and ten thousand and hundred and ten", 110_110),
        ("three million and twenty-seven", 3_000_027),
    ],
)
def test_examples(string: str, expected: int) -> None:
    assert parse_int(string) == expected
