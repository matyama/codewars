from typing import List, Sequence


def parts_sums(ls: Sequence[int]) -> List[int]:
    n = len(ls)

    sums = [0] * (n + 1)
    s = 0

    for i, item in enumerate(reversed(ls)):
        s += item
        sums[n - i - 1] = s

    return sums
