pub fn nb_year(p0: i32, percent: f64, aug: i32, p: i32) -> i32 {
    let growth = 1. + percent / 100.;

    let mut pop = p0;
    let mut n = 0;

    while pop < p {
        pop = (growth * (pop as f64)).floor() as i32;
        pop += aug;
        n += 1;
    }

    n
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(1000, 2.0, 50, 1200, 3)]
    #[case(1500, 5.0, 100, 5000, 15)]
    #[case(1500000, 2.5, 10000, 2000000, 10)]
    #[case(368083, 0.28, 1840, 1393843, 249)]
    #[case(886903, 0.72, 4434, 1957847, 76)]
    fn it_works(
        #[case] p0: i32,
        #[case] percent: f64,
        #[case] aug: i32,
        #[case] p: i32,
        #[case] expected: i32,
    ) {
        assert_eq!(nb_year(p0, percent, aug, p), expected);
    }
}
