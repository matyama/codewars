from typing import List, Union

Num = Union[int, float]
Matrix = List[List[Num]]


def minor(matrix: Matrix, pos: int) -> Matrix:
    return [[x for i, x in enumerate(row) if i != pos] for row in matrix[1:]]


def determinant(matrix: Matrix) -> Num:

    n = len(matrix)

    if n == 1:
        return matrix[0][0]

    if n == 2:
        (a, b), (c, d) = matrix
        return a * d - b * c

    return sum(
        (-1) ** i * matrix[0][i] * determinant(minor(matrix, i))
        for i in range(n)
    )
