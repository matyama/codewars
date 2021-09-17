fn digits_product(n: u64) -> (u64, u32) {
    if n >= 10 {
        let (prod, digits) = digits_product(n / 10);
        (prod * (n % 10), digits + 1)
    } else {
        (n, 1)
    }
}

pub fn persistence(num: u64) -> u64 {
    // Sadly, there's no tail call optimization in Rust yet.
    match digits_product(num) {
        (_, 1) => 0,
        (num, _) => 1 + persistence(num),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(39, 3)]
    #[case(4, 0)]
    #[case(25, 2)]
    #[case(999, 4)]
    fn sample_tests(#[case] num: u64, #[case] exprected: u64) {
        assert_eq!(persistence(num), exprected);
    }
}
