from codewars.closest_points import closest_pair


def test_description_example() -> None:
    points = (
        (2, 2),  # A
        (2, 8),  # B
        (5, 5),  # C
        (6, 3),  # D
        (6, 7),  # E
        (7, 4),  # F
        (7, 9),  # G
    )
    expected = ((6, 3), (7, 4))
    assert sorted(closest_pair(points)) == sorted(expected)


def test_two_points() -> None:
    points = ((2, 2), (6, 3))
    assert sorted(closest_pair(points)) == sorted(points)


def test_duplicated_points() -> None:
    points = (
        (2, 2),  # A
        (2, 8),  # B
        (5, 5),  # C
        (5, 5),  # C
        (6, 3),  # D
        (6, 7),  # E
        (7, 4),  # F
        (7, 9),  # G
    )
    expected = ((5, 5), (5, 5))
    assert sorted(closest_pair(points)) == sorted(expected)
