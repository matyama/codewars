use std::{
    cmp::{max, Reverse},
    collections::{BinaryHeap, HashSet},
};

const FUNCTONS: [fn(u32) -> u32; 2] = [|x| 2 * x + 1, |x| 3 * x + 1];

pub fn dbl_linear(n: u32) -> u32 {
    let mut nth = 1;

    let mut u = HashSet::new();
    let mut unevaluated = BinaryHeap::new();

    unevaluated.push(Reverse(1));

    for _ in 0..=n {
        let Reverse(x) = unevaluated.pop().unwrap();
        nth = max(nth, x);

        for f in FUNCTONS {
            let y = f(x);
            if u.insert(y) {
                unevaluated.push(Reverse(y));
            }
        }
    }

    nth
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case::n0(0, 1)]
    #[case::n10(10, 22)]
    #[case::n20(20, 57)]
    #[case::n30(30, 91)]
    #[case::n50(50, 175)]
    #[case::n100(100, 447)]
    #[case::n500(500, 3355)]
    #[case::n1k(1000, 8488)]
    fn basics_dbl_linear(#[case] n: u32, #[case] exp: u32) {
        assert_eq!(dbl_linear(n), exp);
    }
}
