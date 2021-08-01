use lazy_static::lazy_static;
use regex::Regex;
use std::{
    borrow::Borrow,
    fmt::{Display, Write},
    ops::{Add, BitXor, Deref, Div, Mul, Neg, Shr, Sub},
    rc::Rc,
    str::FromStr,
};

// TODO: get rid of this
const BINARY_OPS: &str = &"+-*/^";

pub fn diff(expr: &str) -> String {
    expr.parse::<Expr>()
        .expect("Invalid expression")
        .diff()
        .to_string()
}

// Basic algebraic data structures
//  - Function and binary operation enumerations
//  - Expression representations
//  - An `Rc` wrapper for expressions

#[derive(Clone, Copy, Debug)]
enum Func {
    Sin,
    Cos,
    Tan,
    Exp,
    Ln,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Clone, Debug)]
struct FuncExpr {
    f: Func,
    arg: Rc<Expr>,
}

// TODO: get rid of this
impl FuncExpr {
    #[inline]
    fn with(&self, f: Func) -> ExprRc {
        Self {
            f,
            arg: self.arg.clone(),
        }
        .into()
    }
}

#[derive(Clone, Debug)]
struct OpExpr {
    lhs: Rc<Expr>,
    op: Op,
    rhs: Rc<Expr>,
}

#[derive(Clone, Debug)]
enum Expr {
    Const(f64),
    Var(String),
    Unary(FuncExpr),
    Binary(OpExpr),
}

#[derive(Clone, Debug)]
struct ExprRc(Rc<Expr>);

// Expression parsing implementations

// TODO: generalize (do not enumerate -> macro?)
impl FromStr for Func {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sin" => Ok(Self::Sin),
            "cos" => Ok(Self::Cos),
            "tan" => Ok(Self::Tan),
            "exp" => Ok(Self::Exp),
            "ln" => Ok(Self::Ln),
            _ => Err(()),
        }
    }
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Sub),
            "*" => Ok(Self::Mul),
            "/" => Ok(Self::Div),
            "^" => Ok(Self::Pow),
            _ => Err(()),
        }
    }
}

fn split_operands(args: &str) -> Option<(&str, &str)> {
    if !args.starts_with('(') {
        args.split_once(' ')
    } else if !args.ends_with(')') {
        args.rsplit_once(' ')
    } else {
        lazy_static! {
            static ref ARGS_RE: Regex = Regex::new(r"^(?P<lhs>\(.*\)) (?P<rhs>\(.*\))$")
                .expect("ARGS_RE failed to compile");
        }
        ARGS_RE
            .captures(args)
            .and_then(|cap| match (cap.name("lhs"), cap.name("rhs")) {
                (Some(lhs), Some(rhs)) => Some((lhs.as_str(), rhs.as_str())),
                _ => None,
            })
    }
}

impl FromStr for Expr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Strip parentheses
        let expr = if s.starts_with('(') {
            &s[1..s.len() - 1]
        } else {
            &s[..]
        };

        // Split expressions like `+ 1 2` or `sin x` to (operator, arguments)
        let expr = if let Some((op, args)) = expr.split_once(' ') {
            // TODO: replace this check by chaining `parse`
            if BINARY_OPS.contains(op) {
                // Binary operation
                let (lhs, rhs) = split_operands(args).ok_or(())?;
                Self::Binary(OpExpr {
                    lhs: lhs.parse().map(Rc::new)?,
                    op: op.parse()?,
                    rhs: rhs.parse().map(Rc::new)?,
                })
            } else {
                // Unary function
                Self::Unary(FuncExpr {
                    f: op.parse()?,
                    arg: args.parse().map(Rc::new)?,
                })
            }
        } else {
            // Constant or a variable
            expr.trim_start_matches('-')
                .parse()
                .map_or_else(|_| Self::Var(expr.to_owned()), Self::Const)
        };

        Ok(expr)
    }
}

// Differentiation implementations

trait Diff {
    type OutExpr;

    fn diff(&self) -> Self::OutExpr;
}

impl Diff for FuncExpr {
    type OutExpr = ExprRc;

    fn diff(&self) -> Self::OutExpr {
        use Expr::*;
        use Func::*;

        if let Const(_) = *self.arg {
            return 0.into();
        }

        // TODO: clean up all these `.clone().into()`
        //
        //  - maybe `impl Borrow<ExprRc> for FuncExpr` => probably not without copying
        let df = match self.f {
            Sin => self.with(Cos),
            Cos => -self.with(Sin),
            Tan => Self::OutExpr::from(1) / (self.with(Cos) ^ 2.into()).into(),
            Exp => self.into(),
            Ln => Self::OutExpr::from(1) / self.arg.clone().into(),
        };

        // Apply the chain rule
        df >> self.arg.clone().into()
    }
}

impl Diff for OpExpr {
    type OutExpr = ExprRc;

    fn diff(&self) -> Self::OutExpr {
        use Expr::*;
        use Func::*;
        use Op::*;

        // TODO: do i need all these clones?
        // FIXME
        let lhs: &ExprRc = &self.lhs.clone().into();
        let rhs: &ExprRc = &self.rhs.clone().into();

        match self.op {
            Add => lhs.diff() + rhs.diff(),
            Sub => lhs.diff() - rhs.diff(),
            Mul => (lhs.diff() * rhs.clone()) + (lhs.clone() * rhs.diff()),
            Div => {
                ((lhs.diff() * rhs.clone()) - (lhs.clone() * rhs.diff())) / (rhs.clone() ^ 2.into())
            }
            Pow => {
                let df: ExprRc = match (lhs.borrow(), rhs.borrow()) {
                    (Const(_), _) => {
                        // TODO: make it nicer
                        Self::OutExpr::from((Pow, lhs.0.clone(), rhs.0.clone())) * (Ln, lhs).into()
                    }
                    (_, Const(a)) => Self::OutExpr::from(*a) * (lhs.clone() ^ (a - 1.0).into()),
                    _ => panic!("Unsupported operatation: only a^x and x^a are allowed"),
                };
                df >> lhs.clone()
            }
        }
    }
}

impl Diff for Expr {
    type OutExpr = Self;

    fn diff(&self) -> Self::OutExpr {
        match self {
            Self::Const(_) => 0.into(),
            Self::Var(_) => 1.into(),
            Self::Unary(f) => f.diff().into(),
            Self::Binary(op) => op.diff().into(),
        }
    }
}

impl Diff for ExprRc {
    type OutExpr = Self;

    fn diff(&self) -> Self::OutExpr {
        self.0.diff().into()
    }
}

// Display implementations

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).to_lowercase().as_str())
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            Self::Add => '+',
            Self::Sub => '-',
            Self::Mul => '*',
            Self::Div => '/',
            Self::Pow => '^',
        };
        f.write_char(op)
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Const(c) => write!(f, "{}", c),
            Self::Var(v) => write!(f, "{}", v),
            Self::Unary(FuncExpr { f: func, arg }) => write!(f, "({} {})", func, arg),
            Self::Binary(OpExpr { lhs, op, rhs }) => write!(f, "({} {} {})", op, lhs, rhs),
        }
    }
}

// Expression arithmetics
//  - Operations on expression refs additionally simplify (reduce) resulting expressions

impl Neg for ExprRc {
    type Output = Self;

    fn neg(self) -> Self::Output {
        use Expr::*;
        match self.borrow() {
            Const(c) => (-c).into(),
            _ => (Op::Mul, Const(-1.0).into(), self.0).into(),
        }
    }
}

impl Add for ExprRc {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        use Expr::*;
        match (self.0.borrow(), rhs.0.borrow()) {
            (_, Const(y)) if *y == 0.0 => self,
            (Const(x), _) if *x == 0.0 => rhs,
            (Const(x), Const(y)) => (x + y).into(),
            _ => (Op::Add, self.0, rhs.0).into(),
        }
    }
}

impl Sub for ExprRc {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        use Expr::*;
        match (self.0.borrow(), rhs.0.borrow()) {
            (_, Const(y)) if *y == 0.0 => self,
            (Const(x), Const(y)) => (x - y).into(),
            _ => (Op::Sub, self.0, rhs.0).into(),
        }
    }
}

impl Mul for ExprRc {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        use Expr::*;
        // TODO: can we do without shellow copy of y and x?
        match (self.0.borrow(), rhs.0.borrow()) {
            (Const(x), _) if *x == 1.0 => rhs,
            (Const(x), _) if *x == 0.0 => 0.into(),
            (_, Const(y)) if *y == 1.0 => self,
            (_, Const(y)) if *y == 0.0 => 0.into(),
            // TODO: is `div.rhs.clone()` necessary when we take ownership of `self`/`rhs`
            (Const(x), Binary(div)) if div.op == Op::Div => {
                // `box` and `if let` guards are still unstable in pattern matching
                //  - https://bit.ly/3f8ENRP
                if let Const(v) = *div.lhs {
                    (Op::Div, Const(x * v).into(), div.rhs.clone()).into()
                } else {
                    (Op::Mul, Const(*x).into(), Binary(div.clone()).into()).into()
                }
            }
            // TODO: make `(lhs @ Binary(...), rhs @ ...)` working
            (Binary(div), Const(y)) if div.op == Op::Div => {
                // `box` and `if let` guards are still unstable in pattern matching
                //  - https://bit.ly/3f8ENRP
                if let Const(v) = *div.lhs {
                    (Op::Div, Const(v * y).into(), div.rhs.clone()).into()
                } else {
                    (Op::Mul, Binary(div.clone()).into(), Const(*y).into()).into()
                }
            }
            (Const(x), Const(y)) => (x * y).into(),
            _ => (Op::Mul, self.0, rhs.0).into(),
        }
    }
}

impl Div for ExprRc {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        use Expr::*;
        match (self.0.borrow(), rhs.0.borrow()) {
            (Const(c), _) if *c == 0.0 => 0.into(),
            (_, Const(c)) if *c == 0.0 => panic!("TODO: division by zero"),
            (Const(x), Const(y)) => (x / y).into(),
            _ => (Op::Div, self.0, rhs.0).into(),
        }
    }
}

// Note that here we interpret x^a as taking x to the a-th power.
impl BitXor for ExprRc {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        use Expr::*;
        match (self.0.borrow(), rhs.0.borrow()) {
            (_, Const(c)) if *c == 0.0 => 1.into(),
            (_, Const(c)) if *c == 1.0 => self,
            (Const(x), Const(y)) => x.powf(*y).into(),
            _ => (Op::Pow, self.0, rhs.0).into(),
        }
    }
}

// Note that we interpret `self >> rhs` as the chain rule:
// `d(rhs)/dx * self` where `self` is assumed to be the derivative of an outer function
impl Shr for ExprRc {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        use Expr::*;
        match (self.borrow(), rhs.borrow()) {
            (Const(_) | Var(_), _) => self,
            (_, Const(_) | Var(_)) => self,
            _ => rhs.diff() * self,
        }
    }
}

// Data conversions

impl From<f64> for Expr {
    fn from(v: f64) -> Self {
        Self::Const(v)
    }
}

impl From<i8> for Expr {
    fn from(v: i8) -> Self {
        Self::Const(v as f64)
    }
}

impl From<ExprRc> for Expr {
    fn from(e: ExprRc) -> Self {
        (*e).clone()
    }
}

impl From<FuncExpr> for ExprRc {
    fn from(e: FuncExpr) -> Self {
        Expr::Unary(e).into()
    }
}

impl From<&FuncExpr> for ExprRc {
    fn from(e: &FuncExpr) -> Self {
        e.clone().into()
    }
}

impl From<OpExpr> for ExprRc {
    fn from(e: OpExpr) -> Self {
        Expr::Binary(e).into()
    }
}

impl From<ExprRc> for Rc<Expr> {
    fn from(e: ExprRc) -> Self {
        e.0
    }
}

impl From<Rc<Expr>> for ExprRc {
    fn from(e: Rc<Expr>) -> Self {
        Self(e)
    }
}

impl From<&Rc<Expr>> for ExprRc {
    fn from(e: &Rc<Expr>) -> Self {
        Self((*e).clone())
    }
}

impl From<(Func, &ExprRc)> for ExprRc {
    fn from(e: (Func, &ExprRc)) -> Self {
        let (f, arg) = e;
        FuncExpr {
            f,
            arg: arg.clone().into(),
        }
        .into()
    }
}

impl From<(Op, Rc<Expr>, Rc<Expr>)> for ExprRc {
    fn from(e: (Op, Rc<Expr>, Rc<Expr>)) -> Self {
        let (op, lhs, rhs) = e;
        OpExpr { lhs, op, rhs }.into()
    }
}

impl From<Expr> for ExprRc {
    fn from(e: Expr) -> Self {
        Self(e.into())
    }
}

impl From<f64> for ExprRc {
    fn from(v: f64) -> Self {
        let v: Expr = v.into();
        v.into()
    }
}

impl From<i8> for ExprRc {
    fn from(v: i8) -> Self {
        let v: Expr = v.into();
        v.into()
    }
}

impl Borrow<Expr> for ExprRc {
    fn borrow(&self) -> &Expr {
        self.0.as_ref()
    }
}

impl Deref for ExprRc {
    type Target = Expr;

    fn deref(&self) -> &Self::Target {
        self.borrow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants_and_variables() {
        assert_eq!(diff("5"), "0");
        assert_eq!(diff("x"), "1");
        assert_eq!(diff("5"), "0");
    }

    #[test]
    fn unary_functions() {
        assert_eq!(diff("(cos x)"), "(* -1 (sin x))");
        assert_eq!(diff("(sin x)"), "(cos x)");

        let result = diff("(tan x)");
        assert!(
            result == "(+ 1 (^ (tan x) 2))" || result == "(/ 1 (^ (cos x) 2))",
            "expected (+ 1 (^ (tan x) 2)) or (/ 1 (^ (cos x) 2))"
        );

        assert_eq!(diff("(exp x)"), "(exp x)");
        assert_eq!(diff("(ln x)"), "(/ 1 x)");
    }

    #[test]
    fn binary_operations() {
        assert_eq!(diff("(+ x x)"), "2");
        assert_eq!(diff("(- x x)"), "0");
        assert_eq!(diff("(* x 2)"), "2");
        assert_eq!(diff("(/ x 2)"), "0.5");
        assert_eq!(diff("(^ x 2)"), "(* 2 x)");
    }

    #[test]
    fn composite_functions() {
        assert_eq!(diff("(+ x (+ x x))"), "3");
        assert_eq!(diff("(- (+ x x) x)"), "1");
        assert_eq!(diff("(* 2 (+ x 2))"), "2");
        assert_eq!(diff("(/ 2 (+ 1 x))"), "(/ -2 (^ (+ 1 x) 2))");
        assert_eq!(diff("(cos (+ x 1))"), "(* -1 (sin (+ x 1)))");

        let result = diff("(cos (* 2 x))");
        assert!(
        result == "(* 2 (* -1 (sin (* 2 x))))"
            || result == "(* -2 (sin (* 2 x)))"
            || result == "(* (* -1 (sin (* 2 x))) 2)",
            "expected (* 2 (* -1 (sin (* 2 x)))) or (* -2 (sin (* 2 x))) or (* (* -1 (sin (* 2 x))) 2)"
        );

        assert_eq!(diff("(sin (+ x 1))"), "(cos (+ x 1))");
        assert_eq!(diff("(sin (* 2 x))"), "(* 2 (cos (* 2 x)))");

        let result = diff("(tan (* 2 x))");
        assert!(
            result == "(* 2 (+ 1 (^ (tan (* 2 x)) 2)))"
                || result == "(* 2 (/ 1 (^ (cos (* 2 x)) 2)))"
                || result == "(/ 2 (^ (cos (* 2 x)) 2))",
            "expected (* 2 (+ 1 (^ (tan (* 2 x)) 2))) or (* 2 (/ 1 (^ (cos (* 2 x)) 2)))"
        );

        assert_eq!(diff("(exp (* 2 x))"), "(* 2 (exp (* 2 x)))");
    }

    #[test]
    fn second_derivative() {
        assert_eq!(diff(&diff("(sin x)")), "(* -1 (sin x))");
        assert_eq!(diff(&diff("(exp x)")), "(exp x)");

        let result = diff(&diff("(^ x 3)"));
        assert!(
            result == "(* 3 (* 2 x))" || result == "(* 6 x)",
            "expected (* 3 (* 2 x)) or (* 6 x)"
        );
    }
}
