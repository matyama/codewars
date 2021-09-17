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
    use float_eq::assert_float_eq;
    use rstest::*;

    const TOL: f64 = 1.0e-12;

    #[rstest]
    #[case(1, 10, 0.5580321939764581)]
    #[case(10, 1000, 0.6921486500921933)]
    #[case(10, 10000, 0.6930471674194457)]
    fn it_works(#[case] maxk: i32, #[case] maxn: i32, #[case] expected: f64) {
        let actual = doubles(maxk, maxn);
        assert_float_eq!(
            actual,
            expected,
            abs <= TOL,
            rmax <= TOL,
            "Expected value must be near: {:e} but was: {:e}",
            expected,
            actual
        );
    }
}
