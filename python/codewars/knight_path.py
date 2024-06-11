import itertools
import sys
from heapq import heappop, heappush
from typing import Dict, Iterable, List, NamedTuple, Tuple, cast

COL_IX = {col: i for i, col in enumerate("abcdefgh")}

# Row and column offsets for knight moves
MOVES = (
    (-2, 1),
    (-1, 2),
    (1, 2),
    (2, 1),
    (2, -1),
    (1, -2),
    (-1, -2),
    (-2, -1),
)


class Position(NamedTuple):
    row: int
    col: int

    @classmethod
    def at(cls, pos: str) -> "Position":
        col, row = pos[0], pos[1]
        return cls(8 - int(row), COL_IX[col])

    def offset(self, move: Tuple[int, int]) -> "Position":
        ro, co = move
        return Position(self.row + ro, self.col + co)

    @property
    def valid(self) -> bool:
        return 0 <= self.row < 8 and 0 <= self.col < 8


def expand(pos: Position) -> Iterable[Position]:
    for move in MOVES:
        next_pos = pos.offset(move)
        if next_pos.valid:
            yield next_pos


def knight(src: str, dst: str) -> int:
    init, goal = Position.at(src), Position.at(dst)

    pq: List[List] = []
    pq_entries: Dict[Position, List] = {}
    counter = itertools.count()

    # Heap operations that allow decrease-key

    def pq_push(pos: Position, moves: int) -> None:
        if pos in pq_entries:
            # Deprecate existing entry
            entry = pq_entries.pop(pos)
            entry[-1] = None

        # Add new entry, possibly with updated priority
        entry = [moves, next(counter), pos]
        pq_entries[pos] = entry
        heappush(pq, entry)

    def pq_pop() -> Position:
        while pq:
            # Discard deprecated entries
            _, _, pos = heappop(pq)
            if pos is not None:
                # Pop first non-deprecated item
                del pq_entries[pos]
                return cast(Position, pos)
        raise KeyError("Priority queue is empty")

    # Dijkstra's SP / Uniform-Cost Search

    moves = {init: 0}
    pq_push(init, 0)

    while pq:
        pos = pq_pop()

        if pos == goal:
            return moves[pos]

        for next_pos in expand(pos):
            next_moves = moves[pos] + 1
            if next_moves < moves.get(next_pos, sys.maxsize):
                moves[next_pos] = next_moves
                pq_push(next_pos, next_moves)

    return -1
