use itertools::Itertools;

pub fn doubles(maxk: i32, maxn: i32) -> f64 {
    (1..=maxk)
        .cartesian_product(1..=maxn)
        .map(|(k, n)| ((k as f64) * ((n + 1) as f64).powi(2 * k)).recip())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::float_eq;

    #[allow(non_fmt_panic)]
    fn assert_float_equals(actual: f64, expected: f64) {
        let merr = 1.0e-12;
        let res =
            float_eq!(actual, expected, abs <= merr) || float_eq!(actual, expected, rmax <= merr);
        assert!(
            res,
            format!(
                "Expected value must be near: {:e} but was:{:e}",
                expected, actual
            )
        );
    }

    fn dotest(maxk: i32, maxn: i32, exp: f64) -> () {
        assert_float_equals(doubles(maxk, maxn), exp);
    }

    #[test]
    fn basic_tests_doubles() {
        dotest(1, 10, 0.5580321939764581);
        dotest(10, 1000, 0.6921486500921933);
        dotest(10, 10000, 0.6930471674194457);
    }
}
