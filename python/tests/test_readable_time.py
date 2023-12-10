from codewars.readable_time import make_readable


def test_make_readable() -> None:
    cases = [
        (0, "00:00:00"),
        (5, "00:00:05"),
        (60, "00:01:00"),
        (86399, "23:59:59"),
        (359999, "99:59:59"),
    ]

    for seconds, expected in cases:
        assert make_readable(seconds) == expected
