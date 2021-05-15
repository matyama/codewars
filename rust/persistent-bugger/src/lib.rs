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

    #[test]
    fn sample_tests() {
        assert_eq!(super::persistence(39), 3);
        assert_eq!(super::persistence(4), 0);
        assert_eq!(super::persistence(25), 2);
        assert_eq!(super::persistence(999), 4);
    }
}
