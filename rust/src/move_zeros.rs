pub fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let n = arr.len();

    let mut out = Vec::with_capacity(n);
    out.resize(n, 0);

    let mut pos = 0;
    for &value in arr {
        if value != 0 {
            out[pos] = value;
            pos += 1;
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::move_zeros;
    use rstest::*;

    #[rstest]
    #[case(&[1, 2, 0, 1, 0, 1, 0, 3, 0, 1], &[1, 2, 1, 1, 3, 1, 0, 0, 0, 0])]
    #[case(
        &[9, 0, 0, 9, 1, 2, 0, 1, 0, 1, 0, 3, 0, 1, 9, 0, 0, 0, 0, 9],
        &[9, 9, 1, 2, 1, 1, 3, 1, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    )]
    #[case(&[0, 0], &[0, 0])]
    #[case(&[0], &[0])]
    #[case(&[], &[])]
    fn sample_tests(#[case] arr: &[u8], #[case] expected: &[u8]) {
        let actual = move_zeros(arr);
        assert_eq!(expected, &actual);
    }
}
