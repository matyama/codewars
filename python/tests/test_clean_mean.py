from collections.abc import Sequence

import pytest

from codewars.clean_mean import clean_mean


@pytest.mark.parametrize(
    "sample,cutoff,expected",
    [
        ([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 100], 3, 5.5),
        (
            [
                1.01,
                0.99,
                1.02,
                1.01,
                0.99,
                0.97,
                1.03,
                0.99,
                1.02,
                0.99,
                3,
                10,
            ],
            2,
            1.0,
        ),
    ],
)
def test_clean_mean(
    sample: Sequence[float], cutoff: int, expected: float
) -> None:
    assert clean_mean(sample, cutoff) == expected
