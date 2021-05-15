from codewars.cube_pile import find_nb


def test_find_nb() -> None:
    assert find_nb(4183059834009) == 2022
    assert find_nb(24723578342962) == -1
    assert find_nb(135440716410000) == 4824
    assert find_nb(40539911473216) == 3568
    assert find_nb(26825883955641) == 3218
