import pytest

from codewars.minesweeper import minesweeper


@pytest.mark.parametrize(
    "matrix,expected",
    [
        (
            [
                [True, False, False],
                [False, True, False],
                [False, False, False],
            ],
            [[1, 2, 1], [2, 1, 1], [1, 1, 1]],
        ),
        (
            [[False, False, False], [False, False, False]],
            [[0, 0, 0], [0, 0, 0]],
        ),
        (
            [
                [True, False, False, True],
                [False, False, True, False],
                [True, True, False, True],
            ],
            [[0, 2, 2, 1], [3, 4, 3, 3], [1, 2, 3, 1]],
        ),
    ],
)
def test_minesweeper(
    matrix: list[list[bool]], expected: list[list[int]]
) -> None:
    actual = minesweeper(matrix)
    assert expected == actual
