from collections import defaultdict
from dataclasses import dataclass


@dataclass(frozen=True)
class Mov:
    """
    `mov x y` copies `y` (either a constant value or the content of a register)
    into register `x`
    """

    x: str
    y: int | str


@dataclass(frozen=True)
class Inc:
    """`inc x` increases the content of the register `x` by one"""

    x: str


@dataclass(frozen=True)
class Dec:
    """`dec x` decreases the content of the register `x` by one"""

    x: str


@dataclass(frozen=True)
class Jnz:
    """
    `jnz x y` jumps to an instruction `y` steps away (positive means forward,
    negative means backward, y can be a register or a constant), but only if
    `x` (a constant or a register) is not zero
    """

    x: int | str
    y: int | str


class Registry(defaultdict[str, int]):
    def __getitem__(self, index: str | int) -> int:
        match index:
            case str(reg):
                return super().__getitem__(reg)
            case int(val):
                return val

    def __setitem__(self, key: str, value: str | int) -> None:
        super().__setitem__(key, self[value])


def simple_assembler(program: list[str]) -> dict[str, int]:
    """
    Interprets given simplified assembler `program`.

    Supported instructions are: `mov x y`, `inc x`, `dec x`, `jnz x y`

    Register names are alphabetical (letters only). Constants are always
    integers (positive or negative).

    Returns a dictionary with the registers.
    """

    def try_parse(v: str) -> int | str:
        try:
            return int(v)
        except ValueError:
            return v

    def parse(i: str) -> Mov | Inc | Dec | Jnz:
        match i.split(maxsplit=2):
            case ["mov", x, y]:
                return Mov(x, y=try_parse(y))
            case ["inc", x]:
                return Inc(x)
            case ["dec", x]:
                return Dec(x)
            case ["jnz", x, y]:
                return Jnz(x=try_parse(x), y=try_parse(y))
            case _:
                raise ValueError(f"unsupported instruction '{i}'")

    prog = [parse(i) for i in program]
    regs = Registry()
    pc = 0

    while 0 <= pc < len(prog):
        match prog[pc]:
            case Mov(x, y):
                regs[x] = y
                pc += 1
            case Inc(x):
                regs[x] += 1
                pc += 1
            case Dec(x):
                regs[x] -= 1
                pc += 1
            case Jnz(x, y):
                pc += 1 if regs[x] == 0 else regs[y]

    return regs
