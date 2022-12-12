from collections.abc import Sequence

import pytest

from codewars.strip_comments import strip_comments


@pytest.mark.parametrize(
    "string,markers,expected",
    [
        (
            "apples, pears # and bananas\ngrapes\nbananas !apples",
            ["#", "!"],
            "apples, pears\ngrapes\nbananas",
        ),
        (
            "a #b\nc\nd $e f g",
            ["#", "$"],
            "a\nc\nd",
        ),
        (" a #b\nc\nd $e f g", ["#", "$"], " a\nc\nd"),
    ],
)
def test_it(string: str, markers: Sequence[str], expected: str) -> None:
    assert strip_comments(string, markers) == expected
