import math
from collections.abc import Sequence
from functools import partial
from typing import Any, TypeAlias, cast

# Usage of numpy/pandas is forbidden!
# import pandas
# import numpy


Summary: TypeAlias = tuple[int, float, float, float, float, float]


class StatisticalSummary:  # pylint: disable=too-few-public-methods
    def __init__(self, seq: Sequence[Any]) -> None:
        assert seq, "empty data sequence"
        xs = sorted(num for x in seq if (num := parse_num(x)) is not None)
        q = partial(quantile, xs)
        self._n = len(xs)
        self._stats = xs[0], xs[-1], q(0.25), q(0.5), q(0.75)

    def five_figure_summary(self, precision: int | None = None) -> Summary:
        stats = (
            (round(s, precision) for s in self._stats)
            if precision is not None
            else self._stats
        )
        return cast(Summary, (self._n, *stats))


def parse_num(x: Any) -> float | None:
    try:
        return float(x)
    except ValueError:
        return None


def quantile(xs: Sequence[float], q: float) -> float:
    """
    Empirical quantile function where:
     - `quantile(_, 0.25)` is the 1st quartile
     - `quantile(_, 0.5)` is the median
     - `quantile(_, 0.75)` is the 3rd quartile

    Implementation: essentially Method 4 as described on
    [wiki](https://en.wikipedia.org/wiki/Quartile)
    """
    n = len(xs)
    i = q * (n - 1)
    k = math.floor(i)
    alpha = i - k
    return xs[k] + alpha * (xs[k + 1] - xs[k])
