import sys


def sum_of_intervals(intervals: list[tuple[int, int]]) -> int:
    """
    Compute the total length of `n` overlapping intervals in `O(n*log(n))` time
    """
    total = 0
    end = -sys.maxsize - 1

    for a, b in sorted(intervals):
        total += max(0, b - max(a, end))
        end = max(b, end)

    return total
