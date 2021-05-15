from codewars.parentheses import valid_parentheses


def test_valid_parentheses() -> None:
    assert not valid_parentheses("  (")
    assert not valid_parentheses(")test")
    assert valid_parentheses("")
    assert not valid_parentheses("hi())(")
    assert valid_parentheses("hi(hi)()")
