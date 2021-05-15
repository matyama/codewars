from collections import deque
from enum import IntEnum
from typing import Deque, List, Sequence


class Direction(IntEnum):
    NORTH = 0
    SOUTH = 1
    EAST = 2
    WEST = 3


OPPOSITE = (
    Direction.SOUTH,
    Direction.NORTH,
    Direction.WEST,
    Direction.EAST,
)


# pylint: disable=invalid-name
def dirReduc(arr: Sequence[str]) -> List[str]:
    path: Deque[Direction] = deque(maxlen=len(arr))

    for d in (Direction[d] for d in arr):
        if path and d is OPPOSITE[path[-1]]:
            _ = path.pop()
        else:
            path.append(d)

    return [d.name for d in path]
