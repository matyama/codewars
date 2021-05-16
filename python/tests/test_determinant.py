from codewars.determinant import determinant


def test_determinant() -> None:
    assert determinant([[1]]) == 1
    assert determinant([[1, 3], [2, 5]]) == -1
    assert determinant([[2, 5, 3], [1, -2, -1], [1, 3, 4]]) == -20
