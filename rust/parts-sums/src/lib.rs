pub fn parts_sums(ls: &[u64]) -> Vec<u64> {
    let n = ls.len();

    let mut sums = vec![0u64; n + 1];
    let mut s = 0;

    for (i, &item) in ls.iter().rev().enumerate() {
        s += item;
        sums[n - i - 1] = s;
    }

    sums
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(&[], &[0])]
    #[case(&[0, 1, 3, 6, 10], &[20, 20, 19, 16, 10, 0])]
    #[case(&[1, 2, 3, 4, 5, 6], &[21, 20, 18, 15, 11, 6, 0])]
    #[case(
        &[744125, 935, 407, 454, 430, 90, 144, 6710213, 889, 810, 2579358],
        &[
            10037855, 9293730, 9292795, 9292388, 9291934, 9291504, 9291414, 9291270, 2581057,
            2580168, 2579358, 0,
        ],
    )]
    fn it_works(#[case] ls: &[u64], #[case] expected: &[u64]) {
        assert_eq!(parts_sums(&ls), expected);
    }
}
