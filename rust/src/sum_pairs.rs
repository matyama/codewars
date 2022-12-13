use std::{collections::BTreeSet, ops::Bound};

const I8_SIZE: usize = (i8::MAX as i16 - i8::MIN as i16) as usize + 1;

/// Find pair of `ints` which sums up to `s` (tie-breaker: 2nd index).
pub fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    // construct value lookup table in O(n) time, assuming that the number of duplicates is O(1)
    let mut values: Vec<Option<BTreeSet<usize>>> = vec![Default::default(); I8_SIZE];
    for (i, x) in ints.iter().cloned().enumerate() {
        values[key(x)].get_or_insert_with(BTreeSet::new).insert(i);
    }

    // O(n) iterations
    ints.iter()
        .cloned()
        .enumerate()
        .filter_map(|(i, x)| {
            // O(1) lookup: find y such that `s = x + y`, then find min between O(1) indices
            let j = *values[key(s - x)]
                .as_ref()?
                .range((Bound::Excluded(i), Bound::Unbounded))
                .min()?;
            Some((i, j))
        })
        .min_by_key(|&(_, j)| j)
        .map(|(i, j)| (ints[i], ints[j]))
}

#[inline]
fn key(x: i8) -> usize {
    (x as isize - i8::MIN as isize) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(&[1, 4, 8, 7, 3, 15], 8, Some((1, 7)))]
    #[case(&[1, -2, 3, 0, -6, 1], -6, Some((0, -6)))]
    #[case(&[20, -13, 40], -7, None)]
    #[case(&[1, 2, 3, 4, 1, 0], 2, Some((1, 1)))]
    #[case(&[10, 5, 2, 3, 7, 5], 10, Some((3, 7)))]
    #[case(&[4, -2, 3, 3, 4], 8, Some((4, 4)))]
    #[case(&[0, 2, 0], 0, Some((0, 0)))]
    #[case(&[5, 9, 13, -3], 10, Some((13, -3)))]
    fn returns_expected(#[case] ints: &[i8], #[case] s: i8, #[case] expected: Option<(i8, i8)>) {
        assert_eq!(sum_pairs(ints, s), expected);
    }
}
