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

    fn dotest(p0: i32, percent: f64, aug: i32, p: i32, exp: i32) {
        println!("p0: {:?};", p0);
        println!("percent: {:?};", percent);
        println!("aug: {:?};", aug);
        println!("p: {:?};", p);
        let ans = nb_year(p0, percent, aug, p);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(1000, 2.0, 50, 1200, 3);
        dotest(1500, 5.0, 100, 5000, 15);
        dotest(1500000, 2.5, 10000, 2000000, 10);
        dotest(368083, 0.28, 1840, 1393843, 249);
        dotest(886903, 0.72, 4434, 1957847, 76);
    }
}
