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

    fn dotest(prod: u64, exp: (u64, u64, bool)) {
        assert_eq!(product_fib(prod), exp)
    }

    #[test]
    fn basics_product_fib() {
        dotest(4895, (55, 89, true));
        dotest(5895, (89, 144, false));
    }
}
