#[allow(clippy::needless_range_loop)]
pub fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    let n = matrix.first().map(Vec::len).unwrap_or_default();

    if n == 0 {
        return Vec::new();
    }

    let mut path = Vec::with_capacity(n.pow(2));

    for start in 0..=(n / 2) {
        let stop = n - start - 1;

        // Move RIGHT
        for col in start..=stop {
            path.push(matrix[start][col]);
        }

        // Move DOWN
        for row in start + 1..=stop {
            path.push(matrix[row][stop]);
        }

        // Move LEFT
        for col in (start..stop).rev() {
            path.push(matrix[stop][col]);
        }

        // Move UP
        for row in (start + 1..stop).rev() {
            path.push(matrix[row][start]);
        }
    }

    path
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(&[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], vec![1, 2, 3, 6, 9, 8, 7, 4, 5])]
    #[case(&[vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]], vec![1, 2, 3, 4, 5, 6, 7, 8, 9])]
    #[case(&[vec![1]], vec![1])]
    #[case(&[vec![]], vec![])]
    fn snail_path(#[case] square: &[Vec<i32>], #[case] expected: Vec<i32>) {
        assert_eq!(snail(square), expected);
    }
}
