def valid_parentheses(string: str) -> bool:
    n_open = 0

    for c in string:
        if c == "(":
            n_open += 1
        elif c == ")":
            n_open -= 1
            if n_open < 0:
                return False

    return n_open == 0
