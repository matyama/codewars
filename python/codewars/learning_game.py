import math
from abc import abstractmethod
from collections import defaultdict
from collections.abc import Sequence
from dataclasses import dataclass
from functools import partial
from typing import Callable, DefaultDict, NamedTuple, Protocol, TypeAlias, cast

Action: TypeAlias = Callable[[int], int]


class MachineState(NamedTuple):
    cmd: int
    action: int


class Machine:
    """
    Machine which learns how to respond to a set of commands by selecting
    correct action to handle them, and receiving a correctness signal.

    The set of actions is common to all commands. Whether there can be multiple
    correct actions for a command is determined by the `stochastic` flag:
      - `stochastic=False` (default) => environment is fully deterministic,
        each command should be handled by single action, moreover, actions are
        deterministic
      - `stochastic=True` => environment and/or actions might be stochastic

    Actions are assumed to be stateless and commands independent.
    """

    k: int
    actions: Sequence[Action]
    _actors: DefaultDict[int, "Actor"]
    _state: MachineState | None

    def __init__(
        self, actions: list[Action] | None = None, stochastic: bool = False
    ) -> None:
        if actions is not None:
            self.actions = actions
        else:
            assert "ACTIONS" in globals(), "ACTIONS() undefined"
            get_actions = globals()["ACTIONS"]
            self.actions = cast(list[Action], get_actions())

        assert self.actions, "no actions"
        self.k = len(self.actions)

        make_actor: Callable[[], "Actor"]
        if stochastic:
            make_actor = partial(Bandit.new, k=self.k)
        else:
            make_actor = partial(GreedyActor.new, k=self.k)

        self._actors = defaultdict(make_actor)
        self._state = None

    def command(self, cmd: int, num: int) -> int:
        """
        Given command `cmd`, select appropriate action and return its result
        when applied to given number `num`.
        """
        a = self._actors[cmd]()
        self._state = MachineState(cmd=cmd, action=a)
        return self.actions[a](num)

    def response(self, res: bool) -> None:
        """Adapt to the result `res` of the last handled command."""
        assert self._state is not None, "response to no command"
        cmd, a = self._state
        self._actors[cmd] += Experience(action=a, reward=res)


class Experience(NamedTuple):
    action: int
    reward: bool


class Actor(Protocol):
    @classmethod
    @abstractmethod
    def new(cls, k: int) -> "Actor": ...

    @abstractmethod
    def __call__(self) -> int: ...

    @abstractmethod
    def __iadd__(self, exp: Experience) -> "Actor": ...


@dataclass
class GreedyActor(Actor):
    """
    Actor that assumes fully deterministic environment.

    This actor sequentially tests each action twice (boolean rewards) until
    finds one which succeeds on both attempts and then pays it forewer.
    """

    k: int
    action: int
    successes: int
    attempts: int
    stop: bool

    @classmethod
    def new(cls, k: int) -> "GreedyActor":
        return cls(k=k, action=0, successes=0, attempts=0, stop=False)

    def __call__(self) -> int:
        return self.action

    def __iadd__(self, exp: Experience) -> "GreedyActor":
        assert self.action == exp.action, "illegal action state"
        self.successes += int(exp.reward)
        self.attempts += 1
        self.stop |= self.successes > 2
        if not self.stop and self.attempts > 2:
            # reset & continue with next action
            self.successes = 0
            self.attempts = 0
            self.action += 1
            assert self.action < self.k, "failed to find correct action"
        return self


@dataclass
class Bandit(Actor):
    """UCB-1 k-armed bandit (Bernoulli)"""

    t: int
    q: list[float]
    n: list[int]
    c: float

    @classmethod
    def new(cls, k: int, c: float = 0.5) -> "Bandit":
        return cls(t=0, q=[0.0] * k, n=[0] * k, c=c)

    def value(self, a: int) -> float:
        log2_t = 0.0 if self.t == 0 else math.log2(self.t)
        return self.q[a] + self.c * math.sqrt(log2_t / self.n[a])

    def __call__(self) -> int:
        k = len(self.q)
        # first try each action at least once
        a = next((a for a in range(k) if self.n[a] == 0), None)
        # select best action according to its current value
        return max(range(k), key=self.value) if a is None else a

    def __iadd__(self, exp: Experience) -> "Bandit":
        a, r = exp
        self.t += 1
        self.q[a] += (float(r) - self.q[a]) / self.t
        self.n[a] += 1
        return self
