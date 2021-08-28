import pytest

from codewars.fibonacci import fib


@pytest.mark.parametrize(
    "n,expected",
    [
        (-5, 5),
        (-4, -3),
        (-3, 2),
        (-2, -1),
        (-1, 1),
        (0, 0),
        (1, 1),
        (2, 1),
        (3, 2),
        (4, 3),
        (5, 5),
        (
            1000,
            43466557686937456435688527675040625802564660517371780402481729089536555417949051890403879840079255169295922593080322634775209689623239873322471161642996440906533187938298969649928516003704476137795166849228875,  # noqa: E501  pylint: disable=line-too-long
        ),
    ],
)
@pytest.mark.execution_timeout(12)
def test_fib(n: int, expected: int) -> None:
    assert fib(n) == expected
