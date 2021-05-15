from codewars.tourney import tourney


def test_even_list_length() -> None:
    input1 = [9, 5, 4, 7, 6, 3, 8, 2]
    output1 = [[9, 5, 4, 7, 6, 3, 8, 2], [9, 7, 6, 8], [9, 8], [9]]
    assert tourney(input1) == output1


def test_odd_list_lenght() -> None:
    input2 = [9, 5, 4, 7, 6, 3, 8]
    output2 = [[9, 5, 4, 7, 6, 3, 8], [8, 9, 7, 6], [9, 7], [9]]
    assert tourney(input2) == output2
