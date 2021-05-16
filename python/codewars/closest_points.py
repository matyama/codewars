import math
from itertools import combinations
from typing import Iterable, List, Sequence, Tuple

NAN_POINT = (float('nan'), float('nan'))


Point = Tuple[float, float]
Pair = Tuple[Point, Point]


def dist(pair: Pair) -> float:
    (x1, y1), (x2, y2) = pair
    return math.sqrt((x1 - x2) ** 2 + (y1 - y2) ** 2)


def find_min(pairs: Iterable[Tuple[Pair, float]]) -> Tuple[Pair, float]:
    return min(
        pairs,
        key=lambda pd: pd[1],
        default=((NAN_POINT, NAN_POINT), float('inf')),
    )


# pylint: disable=too-many-locals
def closest_pair(points: Sequence[Point]) -> Sequence[Point]:
    def find_closest(xs: List[Point], ys: List[Point]) -> Tuple[Pair, float]:

        # Brute-force search for closest pair if current set of points is small
        if len(xs) <= 3:
            return find_min((pair, dist(pair)) for pair in combinations(xs, 2))

        # Divide
        #  - Split all the sets to "left" and "right" halves
        #  - Points in "left" set are those which are to the left of a
        #    vertical line that bisects the original set

        half = len(xs) // 2 + 1
        xs_left, xs_right = xs[:half], xs[half:]

        # Vertical split that bisects input sets
        x_split, _ = xs_left[-1]

        ys_left: List[Point] = []
        ys_right: List[Point] = []
        for p in ys:
            (ys_left if p[0] <= x_split else ys_right).append(p)

        # Conquer
        #  - Recursive call for the left and right splits

        pair_left, min_dist_left = find_closest(xs_left, ys_left)
        pair_right, min_dist_right = find_closest(xs_right, ys_right)

        # Find the closest pair and distnace
        if min_dist_left < min_dist_right:
            pair_min = pair_left
            dist_min = min_dist_left
        else:
            pair_min = pair_right
            dist_min = min_dist_right

        # Combine

        # Find all points in ys which are within 2 delta of the vertical split
        ys_close = [(x, y) for x, y in ys if abs(x - x_split) < dist_min]

        n_ys_close = len(ys_close)

        if n_ys_close <= 1:
            return pair_min, dist_min

        # For each point in ys_close, find points within dist_min
        #  - Note that it's been proven that only 7 points need to be checked
        close_pairs = (
            (ys_close[i], ys_close[j])
            for i in range(n_ys_close - 1)
            for j in range(i + 1, min(i + 8, n_ys_close))
        )

        close_pair_min, close_dist_min = find_min(
            (pair, dist(pair)) for pair in close_pairs
        )

        # Final comparison of result of the recursive call and
        # the closest pair in ys_close.
        return (
            (pair_min, dist_min)
            if dist_min <= close_dist_min
            else (close_pair_min, close_dist_min)
        )

    closest, _ = find_closest(
        xs=sorted(points, key=lambda p: p[0]),
        ys=sorted(points, key=lambda p: p[1]),
    )
    return closest
