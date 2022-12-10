import pytest

from codewars.sum_intervals import sum_of_intervals


@pytest.mark.parametrize(
    "intervals, expected",
    [
        ([(1, 5)], 4),
        ([(1, 5), (6, 10)], 8),
        ([(1, 5), (1, 5)], 4),
        ([(1, 4), (7, 10), (3, 5)], 7),
        ([(-1_000_000_000, 1_000_000_000)], 2_000_000_000),
        ([(0, 20), (-100_000_000, 10), (30, 40)], 100_000_030),
    ],
)
def test_examples(intervals: list[tuple[int, int]], expected: int) -> None:
    assert sum_of_intervals(intervals) == expected
