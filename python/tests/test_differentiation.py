from codewars.differentiation import diff


def test_simple_expr() -> None:
    cases = [
        ("0", "5"),
        ("1", "x"),
        ("2", "(+ x x)"),
        ("0", "(- x x)"),
        ("2", "(* x 2)"),
        ("0.5", "(/ x 2)"),
        ("(* 2 x)", "(^ x 2)"),
        ("(* -1 (sin x))", "(cos x)"),
        ("(cos x)", "(sin x)"),
        ("(exp x)", "(exp x)"),
        ("(/ 1 x)", "(ln x)"),
        ("3", "(+ x (+ x x))"),
        ("1", "(- (+ x x) x)"),
        ("2", "(* 2 (+ x 2))"),
        (
            "(/ -2 (^ (+ 1 x) 2))",
            "(/ 2 (+ 1 x))",
        ),
        (
            "(* -1 (sin (+ x 1)))",
            "(cos (+ x 1))",
        ),
        ("(cos (+ x 1))", "(sin (+ x 1))"),
        (
            "(* 2 (cos (* 2 x)))",
            "(sin (* 2 x))",
        ),
        (
            "(* 2 (exp (* 2 x)))",
            "(exp (* 2 x))",
        ),
    ]

    for expected, expr in cases:
        assert diff(expr) == expected


def test_multiple_option_expr() -> None:
    cases = [
        (
            "(tan x)",
            ("(+ 1 (^ (tan x) 2))", "(^ (cos x) -2)", "(/ 1 (^ (cos x) 2))"),
        ),
        (
            "(tan (* 2 x))",
            (
                "(* 2 (+ 1 (^ (tan (* 2 x)) 2)))",
                "(* 2 (^ (cos (* 2 x)) -2))",
                "(/ 2 (^ (cos (* 2 x)) 2))",
            ),
        ),
        (
            "(cos (* 2 x))",
            ("(* 2 (* -1 (sin (* 2 x))))", "(* -2 (sin (* 2 x)))"),
        ),
    ]

    for expr, options in cases:
        assert diff(expr) in options


def test_second_derivative() -> None:
    cases = [
        ("(sin x)", ("(* -1 (sin x))",)),
        ("(exp x)", ("(exp x)",)),
        ("(^ x 3)", ("(* 3 (* 2 x))", "(* 6 x)")),
    ]

    for expr, options in cases:
        assert diff(diff(expr)) in options
