use num::{BigUint, One};
use once_cell::sync::Lazy;
use regex::{Match, Regex};

use std::fmt::Write;

static INPUT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?P<prefix>.*\D+)?(?P<zeros>0+)?(?P<digits>\d+)?$")
        .expect("INPUT_RE failed to compile")
});

pub fn increment_string(s: &str) -> String {
    let Some(caps) = INPUT_RE.captures(s) else {
        unreachable!("INPUT_RE should match any input string");
    };

    let mut output = caps
        .name("prefix")
        .map(|m| m.as_str())
        .unwrap_or_default()
        .to_owned();

    output.reserve(s.len() - output.len() + 1);

    fn parse(m: Match) -> BigUint {
        m.as_str()
            .parse()
            .expect("failed to parse trailing number as a BigUint")
    }

    match (caps.name("zeros"), caps.name("digits")) {
        // trailing digits with leading zeros
        (Some(zeros), Some(digits)) => {
            let num = parse(digits) + BigUint::one();
            // NOTE: write would call this anyway, so the alloc seems unavoidable
            let num = num.to_str_radix(10);
            let k = zeros.len() - (num.len() - digits.len());
            let zeros = &zeros.as_str()[..k];
            write!(&mut output, "{zeros}{num}").expect("failed to write output");
        }

        // trailing zeros
        (Some(zeros), None) => {
            let zeros = &zeros.as_str()[..(zeros.len() - 1)];
            write!(&mut output, "{zeros}1").expect("failed to write output");
        }

        // trailing digits (without leading zeros)
        (None, Some(digits)) => {
            let num = parse(digits) + BigUint::one();
            write!(&mut output, "{num}").expect("failed to write output");
        }

        // no numeric suffix
        _ => output.push('1'),
    }

    output
}

#[cfg(test)]
mod tests {
    use super::increment_string;
    use rstest::*;

    #[rstest]
    #[trace]
    #[case("foo", "foo1")]
    #[trace]
    #[case("foobar001", "foobar002")]
    #[trace]
    #[case("foobar1", "foobar2")]
    #[trace]
    #[case("foobar00", "foobar01")]
    #[trace]
    #[case("foobar99", "foobar100")]
    #[trace]
    #[case("foobar099", "foobar100")]
    #[trace]
    #[case("", "1")]
    fn sample_tests(#[case] s: &str, #[case] expected: &str) {
        let actual = increment_string(s);
        assert_eq!(actual, expected);
    }

    #[rstest]
    fn long_input() {
        let s = "7GGBPSqCJ6JnGCD6dJBg1324yqlgcpnX3psq9RiMrfC4Yvuri4y0Atb7rJz0e4gChDSfN3cWht\
                 naCBb86zwf0FpmT63AkFXUVG90161399969907970803395914957463096259943980939506\
                 90969785976929235699147446999994";

        let expected = "7GGBPSqCJ6JnGCD6dJBg1324yqlgcpnX3psq9RiMrfC4Yvuri4y0Atb7rJz0e4gChDSfN3cWht\
                        naCBb86zwf0FpmT63AkFXUVG90161399969907970803395914957463096259943980939506\
                        90969785976929235699147446999995";

        let actual = increment_string(s);
        assert_eq!(actual, expected);
    }
}
