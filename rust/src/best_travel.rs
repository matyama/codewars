use itertools::Itertools;

pub fn choose_best_sum(t: i32, k: i32, ls: &[i32]) -> i32 {
    ls.iter()
        .combinations(k as usize)
        .map(|dists| dists.iter().copied().sum())
        .filter(|&total| total <= t)
        .max()
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(163, 3, &[50, 55, 56, 57, 58], 163)]
    #[case(163, 3, &[50], -1)]
    #[case(230, 3, &[91, 74, 73, 85, 73, 81, 87], 228)]
    #[case(331, 2, &[91, 74, 73, 85, 73, 81, 87], 178)]
    fn it_works(#[case] t: i32, #[case] k: i32, #[case] ls: &[i32], #[case] expected: i32) {
        assert_eq!(choose_best_sum(t, k, ls), expected);
    }
}
