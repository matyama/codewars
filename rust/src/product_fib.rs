pub fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut fib_prev = 0;
    let mut fib = 1;
    let mut fib_next = fib + fib_prev;

    while fib * fib_next < prod {
        fib_prev = fib;
        fib = fib_next;
        fib_next = fib + fib_prev;
    }

    (fib, fib_next, fib * fib_next == prod)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(4895, (55, 89, true))]
    #[case(5895, (89, 144, false))]
    fn basics_product_fib(#[case] prod: u64, #[case] expected: (u64, u64, bool)) {
        assert_eq!(product_fib(prod), expected);
    }
}
