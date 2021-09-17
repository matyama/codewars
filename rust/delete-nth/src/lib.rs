pub fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    let mut counts = [0; u8::MAX as usize];
    lst.iter()
        .cloned()
        .filter(|&x| {
            let ix = x as usize;
            counts[ix] += 1;
            counts[ix] <= n
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(&[20, 37, 20, 21], 1, &[20, 37, 21])]
    #[case(&[1, 1, 3, 3, 7, 2, 2, 2, 2], 3, &[1, 1, 3, 3, 7, 2, 2, 2])]
    fn it_works(#[case] lst: &[u8], #[case] n: usize, #[case] expected: &[u8]) {
        assert_eq!(delete_nth(lst, n), expected);
    }
}
