use lazy_static::lazy_static;
use regex::Regex;
use std::{
    borrow::Borrow,
    convert::{TryFrom, TryInto},
    fmt::{Display, Write},
    ops::{Add, BitXor, Deref, Div, Mul, Neg, Shr, Sub},
    rc::Rc,
    str::FromStr,
};

pub fn diff(expr: &str) -> String {
    expr.parse::<Expr>()
        .and_then(|expr| expr.diff())
        .unwrap()
        .to_string()
}

// Basic algebraic data structures
//  - Function and binary operation enumerations
//  - Expression representations
//  - An `Rc` wrapper for expressions
//  - A wrapper that helps to implement `TryFrom` for binary operations which might simplify to
//  generic expressions

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

impl FuncExpr {
    #[inline]
    fn with<E>(&self, f: Func) -> E
    where
        E: From<Self>,
    {
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

struct SimplifiedBinary(Expr);

// Expression parsing implementations
// TODO: Automatically derive string patterns from enum variants.

impl FromStr for Func {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sin" => Ok(Self::Sin),
            "cos" => Ok(Self::Cos),
            "tan" => Ok(Self::Tan),
            "exp" => Ok(Self::Exp),
            "ln" => Ok(Self::Ln),
            _ => Err(format!("Failed to parse a function name from '{}'", s)),
        }
    }
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Sub),
            "*" => Ok(Self::Mul),
            "/" => Ok(Self::Div),
            "^" => Ok(Self::Pow),
            _ => Err(format!("Failed to parse a binary operator from '{}'", s)),
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

impl TryFrom<(&str, &str)> for SimplifiedBinary {
    type Error = String;

    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        let (lhs, rhs) = split_operands(value.1)
            .ok_or(format!("Failed to separate operands from '{}", value.1))?;
        let op = value.0.parse()?;
        let lhs = lhs.parse::<Expr>()?;
        let rhs = rhs.parse::<Expr>()?;
        // Note: Simplify here so that we don't have to deal with `Rc`s inside `OpExpr` later
        let expr = (op, lhs, rhs).valid()?.simplify();
        Ok(SimplifiedBinary(expr))
    }
}

impl TryFrom<(&str, &str)> for FuncExpr {
    type Error = String;

    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        Ok(Self {
            f: value.0.parse()?,
            arg: value.1.parse().map(Rc::new)?,
        })
    }
}

impl FromStr for Expr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Strip parentheses
        let expr = if s.starts_with('(') {
            // Prefix expressions statring with opening '(' must end with ')'
            if !s.ends_with(')') {
                return Err(format!("'{}' does not have matching parentheses", s));
            }

            // Prefix expressions must contain at least one symbol between parentheses
            if s.len() < 3 {
                return Err(format!("'{}' is too short to be valid", s));
            }

            &s[1..s.len() - 1]
        } else {
            s
        };

        // Split expressions like `+ 1 2` or `sin x` to (operator, arguments)
        let expr = if let Some(split) = expr.split_once(' ') {
            split
                .try_into()
                .map(|SimplifiedBinary(expr)| expr)
                .or_else(|e1| {
                    split
                        .try_into()
                        .map(Self::Unary)
                        .map_err(|e2| format!("{} & {}", e1, e2))
                })?
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

    fn diff(&self) -> Result<Self::OutExpr, String>;
}

impl Diff for FuncExpr {
    type OutExpr = ExprRc;

    fn diff(&self) -> Result<Self::OutExpr, String> {
        use Expr::*;
        use Func::*;

        if let Const(_) = *self.arg {
            return Ok(0.into());
        }

        let df = match self.f {
            Sin => Ok(self.with(Cos)),
            Cos => Ok(-self.with::<Self::OutExpr>(Sin)),
            Tan => Self::OutExpr::from(1) / (self.with::<Self::OutExpr>(Cos) ^ 2.into()),
            Exp => Ok(self.into()),
            Ln => Self::OutExpr::from(1) / self.arg.clone().into(),
        }?;

        // Apply the chain rule
        df >> self.arg.clone().into()
    }
}

impl Diff for OpExpr {
    type OutExpr = ExprRc;

    fn diff(&self) -> Result<Self::OutExpr, String> {
        use Expr::*;
        use Func::*;
        use Op::*;

        let f: ExprRc = self.lhs.clone().into();
        let g: ExprRc = self.rhs.clone().into();

        match self.op {
            Add => {
                let df = (&f).diff()?;
                let dg = (&g).diff()?;
                Ok(df + dg)
            }
            Sub => {
                let df = (&f).diff()?;
                let dg = (&g).diff()?;
                Ok(df - dg)
            }
            Mul => {
                let df = (&f).diff()?;
                let dg = (&g).diff()?;
                Ok((df * g) + (f * dg))
            }
            Div => {
                let df = (&f).diff()?;
                let dg = (&g).diff()?;
                let g2 = g.clone() ^ 2.into();
                ((df * g) - (f * dg)) / g2
            }
            Pow => {
                let df = match (f.borrow(), g.borrow()) {
                    (Const(_), _) => Self::OutExpr::from((Pow, f.0.clone(), g.0)) * (Ln, &f).into(),
                    (_, Const(a)) => Self::OutExpr::from(*a) * (f.clone() ^ (a - 1.0).into()),
                    (f, g) => {
                        return Err(format!(
                            "Can't diff '(^ {} {})', only forms supported are a^x and x^a",
                            f, g
                        ));
                    }
                };
                df >> f
            }
        }
    }
}

impl Diff for Expr {
    type OutExpr = Self;

    fn diff(&self) -> Result<Self::OutExpr, String> {
        match self {
            Self::Const(_) => Ok(0.into()),
            Self::Var(_) => Ok(1.into()),
            Self::Unary(f) => f.diff().map(Self::OutExpr::from),
            Self::Binary(op) => op.diff().map(Self::OutExpr::from),
        }
    }
}

impl Diff for ExprRc {
    type OutExpr = Self;

    fn diff(&self) -> Result<Self::OutExpr, String> {
        self.0.diff().map(Self::OutExpr::from)
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

// Expression validation

trait Validated: Sized {
    fn valid(self) -> Result<Self, String>;
}

impl<E> Validated for (Op, E, E)
where
    E: Borrow<Expr> + Display,
{
    fn valid(self) -> Result<Self, String> {
        use Expr::*;
        use Op::*;

        let (op, lhs, rhs) = self;

        match (op, lhs.borrow(), rhs.borrow()) {
            (Div, x, Const(c)) if approx!(c, 0) => {
                Err(format!("Division by zero in '({} {} {})'", Div, x, c))
            }
            _ => Ok((op, lhs, rhs)),
        }
    }
}

// Expression simplification
//
// Unfortunately, GATs have not yet been stabilized, so we have to define `F` as a generic type
// parameter on `Simplify` trait and not on an associated type such as
// ```
// type Simplified<F> where F: From<i8> + From<f64> + From<E> + From<Self> = F;
// ```
//
// One also can't just define it on the `impl<E, F> Simplify for (Op, E, E)` becaues it would not
// be constrained by the implemented trait.

trait Simplify<F> {
    fn simplify(self) -> F;
}

impl<E, F> Simplify<F> for (Op, E, E)
where
    E: Borrow<Expr> + Display,
    F: From<i8> + From<f64> + From<E> + From<Self> + From<(Op, Rc<Expr>, Rc<Expr>)>,
{
    fn simplify(self) -> F {
        use Expr::*;
        use Op::*;

        let (op, lhs, rhs) = self;

        // Binary expression reduction rules
        match (op, lhs.borrow(), rhs.borrow()) {
            (Sub, Var(x), Var(y)) if x == y => 0.into(),
            (Add, Const(x), Const(y)) => (x + y).into(),
            (Sub, Const(x), Const(y)) => (x - y).into(),
            (Mul, Const(x), Const(y)) => (x * y).into(),
            (Div, Const(x), Const(y)) => (x / y).into(),
            (Pow, Const(x), Const(y)) => x.powf(*y).into(),
            (Add | Sub, _, Const(c)) if approx!(c, 0) => lhs.into(),
            (Add, Const(c), _) if approx!(c, 0) => rhs.into(),
            (Mul, Const(c), _) if approx!(c, 1) => rhs.into(),
            (Mul, _, Const(c)) | (Div, _, Const(c)) | (Pow, _, Const(c)) if approx!(c, 1) => {
                lhs.into()
            }
            (Mul, Const(c), _) | (Mul, _, Const(c)) | (Div, Const(c), _) if approx!(c, 0) => {
                0.into()
            }
            (Pow, _, Const(c)) if approx!(c, 0) => 1.into(),
            // Unfortunately `if let` guards are not stable yet and one can't match on `Rc`
            (Mul, Const(a), Binary(div)) | (Mul, Binary(div), Const(a)) if div.op == Div => {
                if let Const(b) = *div.lhs {
                    // Note: This clone is cheap as it only temporarily increments `Rc` counter
                    (Div, Const(a * b).into(), div.rhs.clone()).into()
                } else {
                    (op, lhs, rhs).into()
                }
            }
            _ => (op, lhs, rhs).into(),
        }
    }
}

// Expression algebra
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
        (Op::Add, self.0, rhs.0).simplify()
    }
}

impl Sub for ExprRc {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        (Op::Sub, self.0, rhs.0).simplify()
    }
}

impl Mul for ExprRc {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        (Op::Mul, self.0, rhs.0).simplify()
    }
}

impl Div for ExprRc {
    type Output = Result<Self, String>;

    fn div(self, rhs: Self) -> Self::Output {
        Ok((Op::Div, self.0, rhs.0).valid()?.simplify())
    }
}

// Note that here we interpret x^a as taking x to the a-th power.
impl BitXor for ExprRc {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        (Op::Pow, self.0, rhs.0).simplify()
    }
}

// Note that we interpret `self >> rhs` as the chain rule:
// `d(rhs)/dx * self` where `self` is assumed to be the derivative of an outer function
impl Shr for ExprRc {
    type Output = Result<Self, String>;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn shr(self, rhs: Self) -> Self::Output {
        use Expr::*;
        let expr = match (self.borrow(), rhs.borrow()) {
            (Const(_) | Var(_), _) => self,
            (_, Const(_) | Var(_)) => self,
            _ => rhs.diff()? * self,
        };
        Ok(expr)
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

impl From<(Op, Expr, Expr)> for Expr {
    fn from(e: (Op, Expr, Expr)) -> Self {
        let (op, lhs, rhs) = e;
        Self::Binary(OpExpr {
            lhs: lhs.into(),
            op,
            rhs: rhs.into(),
        })
    }
}

impl From<(Op, Rc<Expr>, Rc<Expr>)> for Expr {
    fn from(e: (Op, Rc<Expr>, Rc<Expr>)) -> Self {
        let (op, lhs, rhs) = e;
        Self::Binary(OpExpr { lhs, op, rhs })
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

// Utilities

#[macro_export]
macro_rules! approx {
    ($x:ident, $y:expr, $eps:expr) => {
        ($x - $y as f64).abs() < $eps
    };
    ($x:ident, $y:expr) => {
        approx!($x, $y, f64::EPSILON)
    };
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

    #[test]
    fn simplification() {
        assert_eq!(diff("(exp (* 1 x))"), "(exp x)");
        assert_eq!(diff("(/ (exp (* 1 x)) (+ (- x x) 1))"), "(exp x)");
    }

    #[test]
    #[should_panic(expected = "'(+ x 1' does not have matching parentheses")]
    fn missing_parenthesis() {
        diff("(exp (+ x 1)");
    }

    #[test]
    #[should_panic(expected = "'()' is too short to be valid")]
    fn empty_expression() {
        diff("(exp ())");
    }

    #[test]
    #[should_panic(expected = "Failed to parse a function name from 'fn'")]
    fn unsupported_function() {
        diff("(fn x)");
    }

    #[test]
    #[should_panic(expected = "Failed to parse a binary operator from '$'")]
    fn unsupported_operator() {
        diff("($ 1 2)");
    }

    #[test]
    #[should_panic(expected = "Division by zero in '(/ (exp x) 0)'")]
    fn division_by_zero() {
        diff("(/ (exp (* 1 x)) (- x x))");
    }

    #[test]
    #[should_panic(expected = "Can't diff '(^ x x)', only forms supported are a^x and x^a")]
    fn unsupported_power() {
        diff("(^ x x)");
    }
}
