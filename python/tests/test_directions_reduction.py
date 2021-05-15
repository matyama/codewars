from codewars.directions_reduction import dirReduc


def test_dir_reduc() -> None:
    assert dirReduc(["NORTH", "SOUTH", "SOUTH", "EAST", "WEST", "NORTH", "WEST"]) == ['WEST']
    assert dirReduc(["NORTH", "WEST", "SOUTH", "EAST"]) == ["NORTH", "WEST", "SOUTH", "EAST"]
