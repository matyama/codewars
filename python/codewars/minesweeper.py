from itertools import product

OFFSET = [-1, 0, 1]


def minesweeper(matrix: list[list[bool]]) -> list[list[int]]:
    """
    Given a flagged MineSweeper board `matrix`, counts the number of
    neighboring mines for each board position.
    """
    rows, cols = len(matrix), len(matrix[0])

    def count_neighbors(row: int, col: int) -> int:
        return sum(
            int(matrix[row + i][col + j])
            for i, j in product(OFFSET, OFFSET)
            if (i, j) != (0, 0) and 0 <= row + i < rows and 0 <= col + j < cols
        )

    return [
        [count_neighbors(row, col) for col in range(cols)]
        for row in range(rows)
    ]
