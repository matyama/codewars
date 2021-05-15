pub fn bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
    if h < 0. || bounce <= 0. || bounce >= 1. || window >= h {
        return -1;
    }

    let n = (window / h).log(bounce).ceil() as i32;
    2 * n - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testequal(h: f64, bounce: f64, window: f64, exp: i32) -> () {
        assert_eq!(bouncing_ball(h, bounce, window), exp)
    }

    #[test]
    fn tests_bouncing_ball() {
        testequal(3.0, 0.66, 1.5, 3);
        testequal(30.0, 0.66, 1.5, 15);
        testequal(40.0, 0.4, 10.0, 3);
        testequal(10.0, 0.6, 10.0, -1);
    }
}
