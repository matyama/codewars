import re
from abc import ABC, abstractmethod
from dataclasses import dataclass

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
    value: int

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
            Binary.new("*", arg1=inner.diff(), arg2=self)
            if isinstance(inner, Fn)
            else self
        )


@dataclass(frozen=True)
class Unary(Fn):
    fn: str
    arg: Expr

    def diff(self) -> Expr:

        if isinstance(self.arg, Const):
            return Const(0)

        if self.fn == "cos":
            df = Binary.new("*", arg1=Const(-1), arg2=Unary("sin", self.arg))

        elif self.fn == "sin":
            df = Unary("cos", self.arg)

        elif self.fn == "tan":
            df = Binary(
                op="/",
                arg1=Const(1),
                arg2=Binary("^", arg1=Unary("cos", self.arg), arg2=Const(2)),
            )

        elif self.fn == "exp":
            df = self

        elif self.fn == "ln":
            df = Binary.new("/", arg1=Const(1), arg2=self.arg)

        else:
            raise ValueError("Unsupported function {fn}".format(fn=self.fn))

        return df.chain(inner=self.arg)

    def __str__(self) -> str:
        return "({fn} {arg})".format(fn=self.fn, arg=self.arg)


@dataclass(frozen=True)
class Binary(Fn):
    op: str
    arg1: Expr
    arg2: Expr

    # pylint: disable=too-many-return-statements,too-many-branches
    @classmethod
    def new(cls, op: str, arg1: Expr, arg2: Expr) -> Expr:
        if isinstance(arg1, Const) and isinstance(arg2, Const):
            # pylint: disable=eval-used
            return Const(
                value=eval(
                    "{x} {op} {y}".format(
                        x=arg1.value,
                        op="**" if op == "^" else op,
                        y=arg2.value,
                    )
                )
            )

        if op in "+-":
            if isinstance(arg1, Const) and arg1.value == 0:
                return arg2
            if isinstance(arg2, Const) and arg2.value == 0:
                return arg1

        if op == "*":
            if isinstance(arg1, Const):
                if arg1.value == 1:
                    return arg2
                if arg1.value == 0:
                    return Const(0)
                if (
                    isinstance(arg2, Binary)
                    and arg2.op == "/"
                    and isinstance(arg2.arg1, Const)
                ):
                    return Binary(
                        op=arg2.op,
                        arg1=Const(arg1.value * arg2.arg1.value),
                        arg2=arg2.arg2,
                    )
            if isinstance(arg2, Const):
                if arg2.value == 1:
                    return arg1
                if arg2.value == 0:
                    return Const(0)
                if (
                    isinstance(arg1, Binary)
                    and arg1.op == "/"
                    and isinstance(arg1.arg1, Const)
                ):
                    return Binary(
                        op=arg1.op,
                        arg1=Const(arg1.arg1.value * arg2.value),
                        arg2=arg1.arg2,
                    )

        if op == "/":
            if isinstance(arg1, Const) and arg1.value == 0:
                return Const(0)
            if isinstance(arg2, Const) and arg2.value == 0:
                raise ZeroDivisionError(
                    "({op} {arg1} {arg2})".format(op=op, arg1=arg1, arg2=arg2)
                )

        if op == "^" and isinstance(arg2, Const):
            if arg2.value == 0:
                return Const(1)
            if arg2.value == 1:
                return arg1

        return cls(op, arg1, arg2)

    def diff(self) -> Expr:

        if isinstance(self.arg1, Const) and isinstance(self.arg2, Const):
            return Const(0)

        if self.op in "+-":
            return Binary.new(self.op, self.arg1.diff(), self.arg2.diff())

        if self.op == "*":
            return Binary.new(
                op="+",
                arg1=Binary.new("*", self.arg1.diff(), self.arg2),
                arg2=Binary.new("*", self.arg1, self.arg2.diff()),
            )

        if self.op == "/":
            return Binary.new(
                op="/",
                arg1=Binary.new(
                    op="-",
                    arg1=Binary.new("*", self.arg1.diff(), self.arg2),
                    arg2=Binary.new("*", self.arg1, self.arg2.diff()),
                ),
                arg2=Binary.new("^", self.arg2, Const(2)),
            )

        if self.op == "^":

            # Case: a^f(x)
            if isinstance(self.arg1, Const):
                df = Binary.new(
                    op="*",
                    arg1=self,
                    arg2=Unary(fn="ln", arg=self.arg1),
                )
                return df.chain(inner=self.arg2)

            # Case: f(x)^a
            if isinstance(self.arg2, Const):
                df = Binary.new(
                    op="*",
                    arg1=self.arg2,
                    arg2=Binary.new(
                        op="^", arg1=self.arg1, arg2=Const(self.arg2.value - 1)
                    ),
                )
                return df.chain(inner=self.arg1)

            return self

        raise ValueError("Unsupported operation {op}".format(op=self.op))

    def __str__(self) -> str:
        return "({op} {x} {y})".format(op=self.op, x=self.arg1, y=self.arg2)


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

        return Binary(op, arg1=parse(expr1), arg2=parse(expr2))

    return Unary(fn=op, arg=parse(args))


def diff(expr: str) -> str:
    return str(parse(expr).diff())
