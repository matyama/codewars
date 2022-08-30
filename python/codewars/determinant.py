from typing import List

Num = int | float
Matrix = List[List[Num]]


def minor(matrix: Matrix, pos: int) -> Matrix:
    return [[x for i, x in enumerate(row) if i != pos] for row in matrix[1:]]


def determinant(matrix: Matrix) -> Num:
    match matrix:
        case [[x]]:
            return x
        case [[a, b], [c, d]]:
            return a * d - b * c
        case m:
            det: Num = sum(
                (-1) ** i * m[0][i] * determinant(minor(m, i))
                for i in range(len(m))
            )
            return det
