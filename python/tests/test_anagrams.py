from codewars.anagrams import anagrams


def test_anagrams() -> None:
    assert anagrams('abba', ['aabb', 'abcd', 'bbaa', 'dada']) == [
        'aabb',
        'bbaa',
    ]
    assert anagrams(
        'racer', ['crazer', 'carer', 'racar', 'caers', 'racer']
    ) == ['carer', 'racer']
