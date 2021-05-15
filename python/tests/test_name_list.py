from codewars.name_list import namelist


def test_namelist() -> None:
    assert (
        namelist(
            [
                {'name': 'Bart'},
                {'name': 'Lisa'},
                {'name': 'Maggie'},
                {'name': 'Homer'},
                {'name': 'Marge'},
            ]
        )
        == 'Bart, Lisa, Maggie, Homer & Marge'
    )
    assert (
        namelist([{'name': 'Bart'}, {'name': 'Lisa'}, {'name': 'Maggie'}])
        == 'Bart, Lisa & Maggie'
    )
    assert namelist([{'name': 'Bart'}, {'name': 'Lisa'}]) == 'Bart & Lisa'
    assert namelist([{'name': 'Bart'}]) == 'Bart'
    assert namelist([]) == ''
