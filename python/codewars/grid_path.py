from collections import deque
from dataclasses import dataclass, field
from heapq import heappop, heappush
from typing import Deque, Iterable, List, Optional


class Position:
    """Data structure specified by the Kata"""

    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y

    def __repr__(self) -> str:
        return f"({self.x},{self.y})"


class Node:
    """Data structure specified by the Kata"""

    def __init__(self, position: Position, passable: bool = True) -> None:
        self.position = position
        self.passable = passable

    def __repr__(self) -> str:
        return self.position.__repr__()


Grid = List[List[Node]]


def find_shortest_path(
    grid: Grid, start_node: Node, end_node: Node
) -> List[Node]:
    """
    Implements A* with Manhattan distance as heuristic.
    """
    if not grid:
        return []

    height, width = len(grid), len(grid[0])
    goal = end_node.position

    def heuristic(state: Node) -> int:
        """Consistent heuristic function (l1 distance)"""
        return abs(state.position.x - goal.x) + abs(state.position.y - goal.y)

    @dataclass(order=True)
    class SearchNode:
        priority: int
        dist: int = field(compare=False)
        state: Node = field(compare=False)
        prev: Optional["SearchNode"] = field(  # noqa: F821
            compare=False, default=None
        )

        def transition(self, state: Node) -> "SearchNode":
            dist = self.dist + 1
            return SearchNode(
                priority=dist + heuristic(state),
                dist=dist,
                state=state,
                prev=self,
            )

        def expand(self) -> Iterable["SearchNode"]:  # noqa: F821
            x, y = self.state.position.x, self.state.position.y

            if x > 0 and grid[x - 1][y].passable:
                yield self.transition(grid[x - 1][y])

            if x + 1 < height and grid[x + 1][y].passable:
                yield self.transition(grid[x + 1][y])

            if y > 0 and grid[x][y - 1].passable:
                yield self.transition(grid[x][y - 1])

            if y + 1 < width and grid[x][y + 1].passable:
                yield self.transition(grid[x][y + 1])

        def reconstruct_path(self) -> List[Node]:
            path: Deque[Node] = deque()
            node = self
            while node.prev is not None:
                path.appendleft(node.state)
                node = node.prev
            path.appendleft(node.state)
            return list(path)

    # Initial search node
    node = SearchNode(priority=heuristic(start_node), dist=0, state=start_node)

    closed = set()
    pq: List[SearchNode] = []
    heappush(pq, node)

    while pq:
        # Retrieve the front of the open list
        node = heappop(pq)

        # Check if goal has been reached and reconstruct path
        if node.state is end_node:
            return node.reconstruct_path()

        state_id = hash(node.state)

        if state_id not in closed:
            closed.add(state_id)

            # Otherwise expand current position
            for succ in node.expand():
                heappush(pq, succ)

    # There's no valid path between start and end positions
    return []
