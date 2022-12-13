import random

import pytest

from codewars.learning_game import Action, Machine


@pytest.fixture(name="actions", scope="module")
def make_actions() -> list[Action]:
    actions: list[Action] = [
        lambda x: x + 1,
        lambda _: 0,
        lambda x: x // 2,
        lambda x: x * 100,
        lambda x: x % 2,
    ]
    return actions


@pytest.fixture(name="machine")
def make_machine(actions: list[Action]) -> Machine:
    return Machine(actions)


def test_single_cmd(machine: Machine) -> None:
    assert machine.k == 5

    cmd = 0
    target = 0

    random.seed()
    for _ in range(0, 20):
        num = random.randint(0, 100)
        r = machine.command(cmd, num)
        machine.response(r == target)

    test_num = random.randint(0, 100)
    assert machine.command(cmd, test_num) == target


@pytest.fixture(name="trained_machine", scope="module")
def train_machine(actions: list[Action]) -> Machine:
    machine = Machine(actions)

    for i in range(0, 200):
        num = random.randint(0, 100)
        y_hat = machine.command(i % 5, num)
        y = actions[i % 5](num)
        machine.response(y_hat == y)

    return machine


@pytest.mark.parametrize(
    "test_cmd,test_num,expected",
    [
        pytest.param(0, 100, 101, id="cmd=0 a={num + 1}"),
        pytest.param(1, 100, 0, id="cmd=1 a={num * 0}"),
        pytest.param(2, 100, 50, id="cmd=2 a={num / 2}"),
        pytest.param(3, 1, 100, id="cmd=3 a={num * 100}"),
        pytest.param(4, 100, 0, id="cmd=4 a={num % 2}"),
    ],
)
def test_multi_cmds(
    test_cmd: int,
    test_num: int,
    expected: int,
    trained_machine: Machine,
) -> None:
    assert trained_machine.command(test_cmd, test_num) == expected
