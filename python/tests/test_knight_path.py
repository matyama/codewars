from codewars.knight_path import knight


def test_shortest_knight_path() -> None:
    cases = [
        ('a1', 'c1', 2),
        ('a1', 'f1', 3),
        ('a1', 'f3', 3),
        ('a1', 'f4', 4),
        ('a1', 'f7', 5),
        ('b2', 'a8', 3),
    ]

    for src, dst, expected in cases:
        assert knight(src, dst) == expected
