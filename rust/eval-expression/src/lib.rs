use anyhow::{anyhow, bail, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    convert::{TryFrom, TryInto},
    ops::{Add, Div, Mul, Neg, Sub},
    str::FromStr,
};

pub fn calc(expr: &str) -> f64 {
    expr.eval().unwrap()
}

#[derive(thiserror::Error, Debug, PartialEq)]
enum Error {
    #[error("Mismatched parentheses in expression '{0}'")]
    MismatchedParentheses(String),
    #[error("Not enough items to apply an operator to")]
    MissingArguments,
    #[error("Failed to parse '{0}'")]
    ParseFailure(String),
    #[error("Values must be separated by an operator at '{0} {0}'")]
    InvalidValueSeparation(f64, f64),
    #[error(transparent)]
    UnknownToken(#[from] strum::ParseError),
}

/// Typeclass for types which can be *evaluated*.
///
/// Evaluation here is generic in the sense that it produces `V` for any type for which
/// there's an implementation of [Apply] - i.e. any *algebraic* output types (not just numbers).
///
/// Note: It additionally requires [f64] to be [`Into<V>`](Into) which is due to [Token]
/// representation of numbers.
trait Eval {
    fn eval<V>(self) -> Result<V>
    where
        Operator: Apply<V>,
        f64: Into<V>;
}

/// This is an implementation of the
/// [*Shunting-yard algorithm*](https://en.wikipedia.org/wiki/Shunting-yard_algorithm) which parses
/// and immediately evaluates expression represented by [Self].
///
/// All errors are reported as variants of [Error].
impl Eval for &str {
    fn eval<V>(self) -> Result<V>
    where
        Operator: Apply<V>,
        f64: Into<V>,
    {
        // Operator stack
        let mut ops = Vec::new();

        // Output stack in Reverse Polish Notation (RPN)
        let mut out = Vec::new();

        // Parse the expression string slice into a stream of `Token`s
        for token in self.parse::<Tokens>()? {
            match token {
                // Handle numbers
                Token::Num(num) => out.push(num.into()),

                // Handle both types of operators: unary negation and binary operators
                Token::Operator(op) => {
                    // Apply all operators from the op stack that have higher precedence than `op`
                    //  - or the same but `op` is left-associative
                    while let Some(top) = ops.last() {
                        match top {
                            OpItem::Op(top_op) if top_op.has_precedence(&op) => {
                                // Actually remove and apply the top of the op stack
                                if let Some(OpItem::Op(top_op)) = ops.pop() {
                                    top_op.apply(&mut out)?
                                }
                            }
                            _ => break,
                        }
                    }

                    ops.push(OpItem::Op(op));
                }

                // Handle left parenthesis
                Token::Parenthesis(ParenKind::Left) => ops.push(OpItem::LeftParenthesis),

                // Handle right parenthesis
                Token::Parenthesis(ParenKind::Right) => {
                    // Apply all operators from the op stack until we hit a left parenthesis or it
                    // runs out (the latter means that parentheses are mismatched)
                    loop {
                        match ops.pop() {
                            Some(OpItem::Op(op)) => op.apply(&mut out)?,
                            Some(OpItem::LeftParenthesis) => break,
                            None => bail!(Error::MismatchedParentheses(self.to_string())),
                        }
                    }
                }
            }
        }

        // Process the rest of the operator stack, there should be no left parenthesis at this point
        while let Some(item) = ops.pop() {
            match item {
                OpItem::Op(op) => op.apply(&mut out)?,
                OpItem::LeftParenthesis => bail!(Error::MismatchedParentheses(self.to_string())),
            }
        }

        // There ought to be at least one item (and if sound then exactly one)
        out.pop()
            .ok_or_else(|| anyhow!(Error::ParseFailure(self.to_string())))
    }
}

#[derive(Clone, Copy, Debug, strum::Display, strum::EnumString)]
enum Op {
    #[strum(serialize = "+")]
    Add,
    #[strum(serialize = "-")]
    Sub,
    #[strum(serialize = "*")]
    Mul,
    #[strum(serialize = "/")]
    Div,
}

#[derive(Debug, strum::Display, strum::EnumString)]
enum ParenKind {
    #[strum(serialize = "(")]
    Left,
    #[strum(serialize = ")")]
    Right,
}

#[derive(Debug)]
enum Operator {
    /// Left-associative binary operator where the precedence of [Op::Mul] and [Op::Div] is higher
    /// than that of [Op::Add] and [Op::Sub]
    Binary(Op),
    /// Right-associative unary negation operator with higher precedence than any infix operator
    Neg,
}

impl Operator {
    fn prec(&self) -> u8 {
        match self {
            Self::Binary(Op::Add) => 1,
            Self::Binary(Op::Sub) => 1,
            Self::Binary(Op::Mul) => 2,
            Self::Binary(Op::Div) => 2,
            Self::Neg => 3,
        }
    }

    fn is_left_assoc(&self) -> bool {
        match self {
            Self::Binary(_) => true,
            Self::Neg => false,
        }
    }

    fn has_precedence(&self, other: &Self) -> bool {
        let self_prec = self.prec();
        let other_prec = other.prec();
        self_prec > other_prec || (self_prec == other_prec && other.is_left_assoc())
    }
}

trait Apply<V> {
    /// Applies [Self] to top n (*arity*) values on the `stack` and replaces these arguments on the
    /// stack by the result.
    fn apply(self, stack: &mut Vec<V>) -> Result<()>;
}

// Note: This impl forwards operator application to traits bounding `V` and therefore can work with
// any *algebraic* type (e.g. one could build an expression AST)
impl<V> Apply<V> for Operator
where
    V: Neg<Output = V> + Add<Output = V> + Sub<Output = V> + Mul<Output = V> + Div<Output = V>,
{
    fn apply(self, stack: &mut Vec<V>) -> Result<()> {
        match self {
            Self::Neg => {
                if let Some(value) = stack.pop() {
                    stack.push(value.neg())
                } else {
                    bail!(Error::MissingArguments)
                }
            }
            Self::Binary(op) => match (stack.pop(), stack.pop()) {
                (Some(rhs), Some(lhs)) => {
                    let value = match op {
                        Op::Add => lhs + rhs,
                        Op::Sub => lhs - rhs,
                        Op::Mul => lhs * rhs,
                        Op::Div => lhs / rhs,
                    };
                    stack.push(value);
                }
                _ => bail!(Error::MissingArguments),
            },
        }
        Ok(())
    }
}

/// [OpItem] represents tokens which can be placed on an operator stack
#[derive(Debug)]
enum OpItem {
    Op(Operator),
    LeftParenthesis,
}

#[derive(Debug)]
enum Token {
    Num(f64),
    Operator(Operator),
    Parenthesis(ParenKind),
}

/// Try to parse an [Operator] from a state represented by a reference to previously parsed [Token]
/// and current token - potentially the operator.
impl TryFrom<(Option<&Token>, &str)> for Operator {
    type Error = anyhow::Error;

    fn try_from(value: (Option<&Token>, &str)) -> Result<Self> {
        let (last, token) = value;

        let op = match (last, token.parse()?) {
            // Check for preceeding operator and parenthesis to determine the unary negation
            (
                None | Some(Token::Operator(_)) | Some(Token::Parenthesis(ParenKind::Left)),
                Op::Sub,
            ) => Operator::Neg,
            (_, op) => Operator::Binary(op),
        };

        Ok(op)
    }
}

/// Try to parse a [Token] from a state represented by a reference to previously parsed [Token]
/// and current token.
impl TryFrom<(Option<&Self>, &str)> for Token {
    type Error = anyhow::Error;

    fn try_from(value: (Option<&Token>, &str)) -> Result<Self> {
        let (last, token) = value;

        // Try to parse the token as a number
        if let Ok(num) = token.parse() {
            if let Some(Self::Num(prev)) = last {
                bail!(Error::InvalidValueSeparation(*prev, num))
            } else {
                return Ok(Self::Num(num));
            }
        }

        // Try to parse the token as a parenthesis
        if let Ok(mode) = token.parse() {
            return Ok(Self::Parenthesis(mode));
        }

        // Finally, try to parse the token as an operator
        if let Ok(op) = value.try_into() {
            return Ok(Self::Operator(op));
        }

        bail!(Error::ParseFailure(token.to_string()))
    }
}

struct Tokens(Vec<Token>);

impl FromStr for Tokens {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref TOKENS_RE: Regex =
                Regex::new(r"[+\-*/()]|\d+\.\d+|\d+").expect("TOKENS_RE failed to compile");
        }

        let mut tokens = Vec::new();

        for token in TOKENS_RE.find_iter(s).map(|m| m.as_str()) {
            tokens.push((tokens.last(), token).try_into()?);
        }

        Ok(Self(tokens))
    }
}

impl IntoIterator for Tokens {
    type Item = Token;
    type IntoIter = <Vec<Self::Item> as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {

    use crate::{Error, Eval};

    use super::calc;
    use rstest::*;

    // Wrap custom message to reduce repitition
    macro_rules! assert_expr_eq {
        ($expr: expr, $expect: expr) => {
            assert_eq!(
                calc($expr),
                $expect,
                "\nexpected expression \"{}\" to equal \"{:?}\", but got \"{:?}\"",
                $expr,
                $expect,
                calc($expr),
            );
        };
    }

    #[rstest]
    #[case("0", 0.0)]
    #[case("1", 1.0)]
    #[case("42", 42.0)]
    #[case("350", 350.0)]
    fn single_values(#[case] expr: &str, #[case] expected: f64) {
        assert_expr_eq!(expr, expected);
    }

    #[rstest]
    #[case("1 + 1", 2.0)]
    #[case("1 - 1", 0.0)]
    #[case("1 * 1", 1.0)]
    #[case("1 / 1", 1.0)]
    #[case("12 * 123", 1476.0)]
    fn basic_operations(#[case] expr: &str, #[case] expected: f64) {
        assert_expr_eq!(expr, expected);
    }

    #[rstest]
    #[case("1-1", 0.0)]
    #[case("1 -1", 0.0)]
    #[case("1- 1", 0.0)]
    #[case("1* 1", 1.0)]
    fn whitespace_between_operators_and_operands(#[case] expr: &str, #[case] expected: f64) {
        assert_expr_eq!(expr, expected);
    }

    #[rstest]
    #[case("1- -1", 2.0)]
    #[case("1--1", 2.0)]
    #[case("1 - -1", 2.0)]
    #[case("-42", -42.0)]
    fn unary_minuses(#[case] expr: &str, #[case] expected: f64) {
        assert_expr_eq!(expr, expected);
    }

    #[rstest]
    #[case("(1)", 1.0)]
    #[case("((1))", 1.0)]
    #[case("((80 - (19)))", 61.0)]
    fn parentheses(#[case] expr: &str, #[case] expected: f64) {
        assert_expr_eq!(expr, expected);
    }

    #[rstest]
    #[case("12* 123/(-5 + 2)", -492.0)]
    #[case("1 - -(-(-(-4)))", -3.0)]
    #[case("2 /2+3 * 4.75- -6", 21.25)]
    #[case("2 / (2 + 3) * 4.33 - -6", 7.732)]
    #[case("(1 - 2) + -(-(-(-4)))", 3.0)]
    #[case("((2.33 / (2.9+3.5)*4) - -6)", 7.45625)]
    fn multiple_operators(#[case] expr: &str, #[case] expected: f64) {
        assert_expr_eq!(expr, expected);
    }

    #[rstest]
    #[case::parenthesis_right("1 + (1 / 2", Error::MismatchedParentheses("1 + (1 / 2".to_string()))]
    #[case::parenthesis_left("1 + 1 / 2)", Error::MismatchedParentheses("1 + 1 / 2)".to_string()))]
    #[case::value_separateor("1 2", Error::InvalidValueSeparation(1.0, 2.0))]
    #[case::missing_neg_arg("-", Error::MissingArguments)]
    #[case::missing_op_arg("1 +", Error::MissingArguments)]
    fn failures(#[case] expr: &str, #[case] expected: Error) {
        let error = expr
            .eval::<f64>()
            .expect_err("eval should fail")
            .downcast::<Error>()
            .expect("custom error");

        assert_eq!(error, expected);
    }
}
