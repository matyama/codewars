pub fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    let mut counts = [0; u8::MAX as usize];
    lst.into_iter()
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

    #[test]
    fn test_basic() {
        assert_eq!(delete_nth(&[20, 37, 20, 21], 1), vec![20, 37, 21]);
        assert_eq!(
            delete_nth(&[1, 1, 3, 3, 7, 2, 2, 2, 2], 3),
            vec![1, 1, 3, 3, 7, 2, 2, 2]
        );
    }
}
