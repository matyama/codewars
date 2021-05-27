from typing import Tuple

from codewars.grid_path import Grid, Node, Position, find_shortest_path


def make_grid(
    grid_blueprint: str, width: int, height: int
) -> Tuple[Grid, Node, Node]:
    """
    Convert a list of lists of 0's and 1's to a graph of Node objects.
    Note: This function is specified in the Kata.
    """

    grid_blueprint = grid_blueprint.strip().replace('\n', '')
    grid: Grid = []
    start_node, end_node = None, None
    for x in range(0, width):
        grid.append([])
        for y in range(0, height):
            char = grid_blueprint[y * width + x]
            node = Node(
                position=Position(x, y),
                passable=char != '1',
            )
            if char == 'S':
                start_node = node
            elif char == 'E':
                end_node = node
            grid[x].append(node)

    assert start_node is not None and end_node is not None
    return grid, start_node, end_node


def test_path() -> None:
    grid_blueprint = """
S0110
01000
01010
00010
0001E
"""

    grid, grid_start, grid_target = make_grid(grid_blueprint, 5, 5)

    expected = [
        grid[0][0],
        grid[0][1],
        grid[0][2],
        grid[0][3],
        grid[1][3],
        grid[2][3],
        grid[2][2],
        grid[2][1],
        grid[3][1],
        grid[4][1],
        grid[4][2],
        grid[4][3],
        grid[4][4],
    ]

    assert find_shortest_path(grid, grid_start, grid_target) == expected
