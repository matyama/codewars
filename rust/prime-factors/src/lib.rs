use std::collections::HashMap;

/// From given list `l` collects a vector of pairs `(p, s)` where
///  1. `p` is a prime factor of at least one number `x` in `l`
///  2. `s` is the sum of all such `x` in `l` for which `p` is a prime factor
/// Implementation is based on https://bit.ly/3flId4d
pub fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
    let mut sums: Vec<(i64, i64)> = l
        .into_iter()
        .map(prime_factors)
        .reduce(|mut acc, factors| {
            for (p, s) in factors {
                *acc.entry(p).or_default() += s;
            }
            acc
        })
        .unwrap_or_default()
        .into_iter()
        .collect();

    sums.sort_by_key(|&(p, _)| p);
    sums
}

fn prime_factors(num: i64) -> HashMap<i64, i64> {
    let mut factors = HashMap::new();
    let mut n = num.abs();

    while n % 2 == 0 {
        *factors.entry(2).or_default() = num;
        n /= 2;
    }

    let sqrt_n = (n as f64).sqrt().floor() as i64;
    for i in (3..=sqrt_n).step_by(2) {
        while n % i == 0 {
            *factors.entry(i).or_default() = num;
            n /= i;
        }
    }

    if n > 2 {
        *factors.entry(n).or_default() = num;
    }

    factors
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(vec![12, 15], vec![(2, 12), (3, 27), (5, 15)])]
    #[case(vec![15, 21, 24, 30, 45], vec![(2, 54), (3, 135), (5, 90), (7, 21)])]
    #[case(vec![15, 21, 24, 30, -45], vec![(2, 54), (3, 45), (5, 0), (7, 21)])]
    fn basics_sum_of_divided(#[case] l: Vec<i64>, #[case] expected: Vec<(i64, i64)>) {
        assert_eq!(sum_of_divided(l), expected);
    }
}
