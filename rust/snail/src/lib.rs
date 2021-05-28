#[allow(clippy::clippy::needless_range_loop)]
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

    #[test]
    fn sample_test1() {
        let square = &[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test2() {
        let square = &[vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test3() {
        let square: &[Vec<i32>; 1] = &[Vec::new()];
        let expected = Vec::new();
        assert_eq!(snail(square), expected, "Failed with empty input");
    }

    #[test]
    fn sample_test4() {
        let square = &[vec![1]];
        let expected = vec![1];
        assert_eq!(snail(square), expected);
    }
}
