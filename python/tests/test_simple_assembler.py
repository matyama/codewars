import pytest

from codewars.simple_assembler import simple_assembler


@pytest.mark.parametrize(
    "program,expected",
    [
        (
            ["mov a 5", "inc a", "dec a", "dec a", "jnz a -1", "inc a"],
            {"a": 1},
        ),
        (
            [
                "mov c 12",
                "mov b 0",
                "mov a 200",
                "dec a",
                "inc b",
                "jnz a -2",
                "dec c",
                "mov a b",
                "jnz c -5",
                "jnz 0 1",
                "mov c a",
            ],
            {"a": 409600, "c": 409600, "b": 409600},
        ),
    ],
)
def test_simple_asm_interpreter(
    program: list[str], expected: dict[str, int]
) -> None:
    assert simple_assembler(program) == expected
