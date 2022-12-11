from collections import deque
from typing import Optional, TypeAlias

Tree: TypeAlias = Optional["Node"]


class Node:  # pylint: disable=too-few-public-methods,invalid-name
    def __init__(self, L: Tree, R: Tree, n: int) -> None:
        self.left = L
        self.right = R
        self.value = n


def tree_by_levels(node: Tree) -> list[int]:
    """Traverses given tree in BFS order yielding its values"""

    if node is None:
        return []

    queue = deque([node])
    values = []

    while queue:

        n = queue.popleft()
        values.append(n.value)

        if n.left is not None:
            queue.append(n.left)

        if n.right is not None:
            queue.append(n.right)

    return values
