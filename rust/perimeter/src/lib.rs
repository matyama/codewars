pub fn perimeter(n: u64) -> u64 {
    let (fib_sum, _, _) = (0..n).fold((1, 1, 0), |(fib_sum, fib, fib_prev), _| {
        (fib_sum + fib + fib_prev, fib + fib_prev, fib)
    });
    4 * fib_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: u64, exp: u64) -> () {
        assert_eq!(perimeter(n), exp)
    }

    #[test]
    fn basics_perimeter() {
        dotest(4, 48);
        dotest(5, 80);
        dotest(7, 216);
        dotest(20, 114624);
        dotest(30, 14098308);
    }
}
