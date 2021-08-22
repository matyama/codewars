pub fn spiralize(size: usize) -> Vec<Vec<i8>> {
    let mut spiral = vec![vec![0; size]; size];

    for start in (0..=(size / 2)).step_by(2) {
        let end = size - start - 1;

        // Walk and put marks along current border
        for i in start..=end {
            spiral[start][i] = 1;
            spiral[i][end] = 1;
            spiral[end][i] = 1;
            spiral[i][start] = 1;
        }

        let bridge = start.saturating_sub(1);

        if bridge < end {
            // Connect to previous
            spiral[start][bridge] = 1;
            // Disconnect current
            spiral[start + 1][start] = 0;
        }
    }

    spiral
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test5() {
        assert_eq!(
            spiralize(5),
            [
                [1, 1, 1, 1, 1],
                [0, 0, 0, 0, 1],
                [1, 1, 1, 0, 1],
                [1, 0, 0, 0, 1],
                [1, 1, 1, 1, 1],
            ],
        );
    }

    #[test]
    fn test6() {
        assert_eq!(
            spiralize(6),
            [
                [1, 1, 1, 1, 1, 1],
                [0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 0, 1],
                [1, 0, 0, 1, 0, 1],
                [1, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1],
            ],
        );
    }

    #[test]
    fn test7() {
        assert_eq!(
            spiralize(7),
            [
                [1, 1, 1, 1, 1, 1, 1],
                [0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 1, 0, 1],
                [1, 0, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 1],
            ],
        );
    }

    #[test]
    fn test8() {
        assert_eq!(
            spiralize(8),
            [
                [1, 1, 1, 1, 1, 1, 1, 1],
                [0, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 1, 0, 1],
                [1, 0, 1, 0, 0, 1, 0, 1],
                [1, 0, 1, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 1, 1],
            ],
        );
    }

    #[test]
    fn test10() {
        assert_eq!(
            spiralize(10),
            [
                [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 1, 0, 1],
                [1, 0, 1, 1, 1, 1, 0, 1, 0, 1],
                [1, 0, 1, 0, 0, 1, 0, 1, 0, 1],
                [1, 0, 1, 0, 0, 0, 0, 1, 0, 1],
                [1, 0, 1, 1, 1, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            ],
        );
    }
}
