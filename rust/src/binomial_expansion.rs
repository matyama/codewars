use std::fmt::Write as _;

use num::integer::binomial;
use once_cell::sync::Lazy;
use regex::Regex;

static EXPR_PATTER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\((?P<m>-)?(?P<a>\d+)?(?P<x>[a-zA-z])(?P<o>[+-])(?P<b>\d+)\)\^(?P<n>\d+)").unwrap()
});

pub fn expand(expr: &str) -> String {
    let Ok(Expr { a, x, b, n }) = Expr::try_from(expr) else {
        panic!("invalid expression: {expr}");
    };

    if a == 0 {
        return format!("{}", b.pow(n));
    }

    if b == 0 {
        return format!("{}", Term::init(a, x, n));
    }

    let mut expansion = String::new();

    for term in (0..=n).map(|k| Term::new(k, a, x, b, n)) {
        write!(&mut expansion, "{term}").expect("write term");
    }

    expansion
}

struct Expr<'a> {
    a: i64,
    x: &'a str,
    b: i64,
    n: u32,
}

impl<'a> TryFrom<&'a str> for Expr<'a> {
    type Error = ();

    /// Parse an expression of the form: `(ax+b)^n`
    fn try_from(expr: &'a str) -> Result<Self, Self::Error> {
        let Some(caps) = EXPR_PATTER.captures(expr) else {
            return Err(());
        };

        let mut a = match caps.name("a") {
            Some(a) => a.as_str().parse().map_err(|_| ())?,
            None => 1,
        };

        if caps.name("m").is_some() {
            a = -a;
        }

        let Some(x) = caps.name("x").map(|x| x.as_str()) else {
            return Err(());
        };

        let mut b = match caps.name("b") {
            Some(b) => b.as_str().parse().map_err(|_| ())?,
            None => 0,
        };

        if let Some("-") = caps.name("o").map(|o| o.as_str()) {
            b = -b;
        }

        let n = match caps.name("n") {
            Some(n) => n.as_str().parse().map_err(|_| ())?,
            None => 1,
        };

        Ok(Expr { a, x, b, n })
    }
}

struct Term<'a> {
    k: u32,
    c: i64,
    x: &'a str,
    n: u32,
}

impl<'x> Term<'x> {
    fn init(a: i64, x: &'x str, n: u32) -> Self {
        Self {
            k: 0,
            c: a.pow(n),
            x,
            n,
        }
    }

    fn new(k: u32, a: i64, x: &'x str, b: i64, n: u32) -> Self {
        Self {
            k,
            c: binomial(n, k) as i64 * a.pow(n - k) * b.pow(k),
            x,
            n,
        }
    }
}

impl std::fmt::Display for Term<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.k > 0 && self.c > 0 {
            f.write_char('+')?;
        }

        let i = self.n - self.k;

        match (self.c, i) {
            (c @ 0, _) | (c, 0) => write!(f, "{c}")?,
            (1, _) => write!(f, "{}", self.x)?,
            (-1, _) => write!(f, "-{}", self.x)?,
            (c, _) => write!(f, "{c}{}", self.x)?,
        }

        if i > 1 {
            write!(f, "^{i}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("(x+1)^0", "1")]
    #[case("(x+1)^1", "x+1")]
    #[case("(x+1)^2", "x^2+2x+1")]
    #[case("(x-1)^0", "1")]
    #[case("(x-1)^1", "x-1")]
    #[case("(x-1)^2", "x^2-2x+1")]
    #[case("(5m+3)^4", "625m^4+1500m^3+1350m^2+540m+81")]
    #[case("(2x-3)^3", "8x^3-36x^2+54x-27")]
    #[case("(7x-7)^0", "1")]
    #[case("(-5m+3)^4", "625m^4-1500m^3+1350m^2-540m+81")]
    #[case("(-2k-3)^3", "-8k^3-36k^2-54k-27")]
    #[case("(-7x-7)^0", "1")]
    #[trace]
    fn fixed_tests(#[case] expr: &str, #[case] expected: &str) {
        let actual = expand(expr);
        assert_eq!(expected, actual);
    }
}
