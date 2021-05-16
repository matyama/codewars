from codewars.game_of_life import get_generation


def test_game_of_life() -> None:
    start = [[1, 0, 0], [0, 1, 1], [1, 1, 0]]
    end = [[0, 1, 0], [0, 0, 1], [1, 1, 1]]
    assert get_generation(start, 1) == end
