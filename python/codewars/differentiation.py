import re
from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import Any, Dict, Optional, Type, Union, cast

ARGS_EXPR = re.compile(r"^(?P<expr1>\(.*\)) (?P<expr2>\(.*\))$")


class Expr(ABC):
    @abstractmethod
    def diff(self) -> "Expr":
        raise NotImplementedError

    # pylint: disable=unused-argument
    def chain(self, inner: "Expr") -> "Expr":
        return self


@dataclass(frozen=True)
class Const(Expr):
    value: Union[int, float]

    def diff(self) -> Expr:
        return Const(0)

    def __str__(self) -> str:
        return str(self.value)


@dataclass(frozen=True)
class Var(Expr):
    name: str

    def diff(self) -> Expr:
        return Const(1)

    def __str__(self) -> str:
        return self.name


class Fn(Expr, ABC):
    def chain(self, inner: Expr) -> Expr:
        """Chains derivative of inner to self if inner's a funciton"""
        return (
            Mul.apply(arg1=inner.diff(), arg2=self)
            if isinstance(inner, Fn)
            else self
        )


# pylint: disable=too-few-public-methods
class UnaryMeta:
    _funcs: Dict[str, Type["Unary"]] = {}

    @classmethod
    def __init_subclass__(cls, **kwargs: Any) -> None:
        # https://github.com/python/mypy/issues/4660
        super().__init_subclass__(**kwargs)  # type: ignore
        # This method is called on subclasses and since `cls` is a subclass of
        # `UnaryMeta` which is only used as a base for `Unary`, this is safe.
        cls._funcs[cls.__name__.lower()] = cast("Type[Unary]", cls)


@dataclass(frozen=True)  # type: ignore
class Unary(Fn, ABC, UnaryMeta):
    arg: Expr

    @classmethod
    def new(cls, fn: str, arg: Expr) -> "Unary":
        if fn not in cls._funcs:
            raise ValueError(f"Unsupported function {fn}")

        return cls._funcs[fn](arg)

    @abstractmethod
    def _diff(self) -> Expr:
        raise NotImplementedError

    def diff(self) -> Expr:
        if isinstance(self.arg, Const):
            return Const(0)

        return self._diff().chain(inner=self.arg)

    def __str__(self) -> str:
        return f"({self.__class__.__name__.lower()} {self.arg})"


@dataclass(frozen=True)
class Sin(Unary):
    def _diff(self) -> Expr:
        return Cos(self.arg)


@dataclass(frozen=True)
class Cos(Unary):
    def _diff(self) -> Expr:
        return Mul.apply(arg1=Const(-1), arg2=Sin(self.arg))


@dataclass(frozen=True)
class Tan(Unary):
    def _diff(self) -> Expr:
        return Div(
            arg1=Const(1),
            arg2=Pow(arg1=Cos(self.arg), arg2=Const(2)),
        )


@dataclass(frozen=True)
class Exp(Unary):
    def _diff(self) -> Expr:
        return self


@dataclass(frozen=True)
class Ln(Unary):
    def _diff(self) -> Expr:
        return Div.apply(arg1=Const(1), arg2=self.arg)


# pylint: disable=too-few-public-methods
class BinaryMeta:
    _ops: Dict[str, Type["Binary"]] = {}
    _op: str

    @classmethod
    def __init_subclass__(
        cls, /, op: Optional[str] = None, **kwargs: Any
    ) -> None:
        # https://github.com/python/mypy/issues/4660
        super().__init_subclass__(**kwargs)  # type: ignore
        # This method is called on subclasses and since `cls` is a subclass of
        # `BinaryMeta` which is only used as a base for `Binary`, this is safe.
        if op is not None:
            cls._ops[op] = cast("Type[Binary]", cls)
            cls._op = op


@dataclass(frozen=True)  # type: ignore
class Binary(Fn, BinaryMeta):
    arg1: Expr
    arg2: Expr

    @classmethod
    @abstractmethod
    def _eval(cls, x: Const, y: Const) -> Const:
        raise NotImplementedError

    @classmethod
    @abstractmethod
    def _apply(cls, arg1: Expr, arg2: Expr) -> Expr:
        raise NotImplementedError

    @abstractmethod
    def _diff(self) -> Expr:
        raise NotImplementedError

    @classmethod
    def new(cls, op: str, arg1: Expr, arg2: Expr) -> "Binary":
        if op not in cls._ops:
            raise ValueError(f"Unsupported operation {op}")

        return cls._ops[op](arg1, arg2)

    @classmethod
    def apply(cls, arg1: Expr, arg2: Expr) -> Expr:
        if isinstance(arg1, Const) and isinstance(arg2, Const):
            return cls._eval(arg1, arg2)

        return cls._apply(arg1, arg2)

    def diff(self) -> Expr:
        if isinstance(self.arg1, Const) and isinstance(self.arg2, Const):
            return Const(0)

        return self._diff()

    def __str__(self) -> str:
        return f"({self._op} {self.arg1} {self.arg2})"


@dataclass(frozen=True)
class Add(Binary, op="+"):
    @classmethod
    def _eval(cls, x: Const, y: Const) -> Const:
        return Const(x.value + y.value)

    @classmethod
    def _apply(cls, arg1: Expr, arg2: Expr) -> Expr:
        if isinstance(arg1, Const) and arg1.value == 0:
            return arg2
        if isinstance(arg2, Const) and arg2.value == 0:
            return arg1
        return cls(arg1, arg2)

    def _diff(self) -> Expr:
        return self.apply(self.arg1.diff(), self.arg2.diff())


@dataclass(frozen=True)
class Sub(Add, op="-"):
    @classmethod
    def _eval(cls, x: Const, y: Const) -> Const:
        return Const(x.value - y.value)

    @classmethod
    def _apply(cls, arg1: Expr, arg2: Expr) -> Expr:
        if isinstance(arg2, Const) and arg2.value == 0:
            return arg1
        return cls(arg1, arg2)


@dataclass(frozen=True)
class Mul(Binary, op="*"):
    @classmethod
    def _eval(cls, x: Const, y: Const) -> Const:
        return Const(x.value * y.value)

    # pylint: disable=too-many-return-statements
    @classmethod
    def _apply(cls, arg1: Expr, arg2: Expr) -> Expr:
        if isinstance(arg1, Const):
            if arg1.value == 1:
                return arg2
            if arg1.value == 0:
                return Const(0)
            if isinstance(arg2, Div) and isinstance(arg2.arg1, Const):
                return Div(
                    arg1=Const(arg1.value * arg2.arg1.value),
                    arg2=arg2.arg2,
                )

        if isinstance(arg2, Const):
            if arg2.value == 1:
                return arg1
            if arg2.value == 0:
                return Const(0)
            if isinstance(arg1, Div) and isinstance(arg1.arg1, Const):
                return Div(
                    arg1=Const(arg1.arg1.value * arg2.value),
                    arg2=arg1.arg2,
                )

        return cls(arg1, arg2)

    def _diff(self) -> Expr:
        return Add.apply(
            arg1=Mul.apply(self.arg1.diff(), self.arg2),
            arg2=Mul.apply(self.arg1, self.arg2.diff()),
        )


@dataclass(frozen=True)
class Div(Binary, op="/"):
    @classmethod
    def _eval(cls, x: Const, y: Const) -> Const:
        return Const(x.value / y.value)

    @classmethod
    def _apply(cls, arg1: Expr, arg2: Expr) -> Expr:
        if isinstance(arg1, Const) and arg1.value == 0:
            return Const(0)
        if isinstance(arg2, Const) and arg2.value == 0:
            raise ZeroDivisionError(f"(/ {arg1} {arg2})")
        return cls(arg1, arg2)

    def _diff(self) -> Expr:
        return Div.apply(
            arg1=Sub.apply(
                arg1=Mul.apply(self.arg1.diff(), self.arg2),
                arg2=Mul.apply(self.arg1, self.arg2.diff()),
            ),
            arg2=Pow.apply(self.arg2, Const(2)),
        )


@dataclass(frozen=True)
class Pow(Binary, op="^"):
    @classmethod
    def _eval(cls, x: Const, y: Const) -> Const:
        return Const(x.value ** y.value)

    @classmethod
    def _apply(cls, arg1: Expr, arg2: Expr) -> Expr:
        if isinstance(arg2, Const):
            if arg2.value == 0:
                return Const(1)
            if arg2.value == 1:
                return arg1
        return cls(arg1, arg2)

    def _diff(self) -> Expr:

        # Case (exponential): a^f(x)
        if isinstance(self.arg1, Const):
            df = Mul.apply(
                arg1=self,
                arg2=Ln(self.arg1),
            )
            return df.chain(inner=self.arg2)

        # Case (power): f(x)^a
        if isinstance(self.arg2, Const):
            df = Mul.apply(
                arg1=self.arg2,
                arg2=Pow.apply(
                    arg1=self.arg1, arg2=Const(self.arg2.value - 1)
                ),
            )
            return df.chain(inner=self.arg1)

        # Default case (exponential): e^x
        return self


def parse(expression: str) -> Expr:

    if expression.startswith("("):
        expression = expression[1:-1]

    expr = expression.split(" ", maxsplit=1)

    if len(expr) == 1:
        v = expr[0]
        return Const(int(v)) if v.lstrip("-").isnumeric() else Var(v)

    op, args = expr[0], expr[1]

    if op in "+-*/^":

        if not args.startswith("("):
            expr1, _, expr2 = args.partition(" ")
        elif not args.endswith(")"):
            expr1, _, expr2 = args.rpartition(" ")
        else:
            match = ARGS_EXPR.match(args)
            assert match is not None
            expr1 = match.group("expr1")
            expr2 = match.group("expr2")

        return Binary.new(op, arg1=parse(expr1), arg2=parse(expr2))

    return Unary.new(fn=op, arg=parse(args))


def diff(expr: str) -> str:
    return str(parse(expr).diff())
