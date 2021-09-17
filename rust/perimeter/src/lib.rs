pub fn perimeter(n: u64) -> u64 {
    let (fib_sum, _, _) = (0..n).fold((1, 1, 0), |(fib_sum, fib, fib_prev), _| {
        (fib_sum + fib + fib_prev, fib + fib_prev, fib)
    });
    4 * fib_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(4, 48)]
    #[case(5, 80)]
    #[case(7, 216)]
    #[case(20, 114624)]
    #[case(30, 14098308)]
    fn basics_perimeter(#[case] n: u64, #[case] expected: u64) {
        assert_eq!(perimeter(n), expected);
    }
}
