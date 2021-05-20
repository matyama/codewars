pub fn solution(degrees: Vec<usize>) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solution(vec![]), true,);
        assert_eq!(solution(vec![0]), true);
        assert_eq!(solution(vec![2]), false);
        assert_eq!(solution(vec![1, 1]), true);
        assert_eq!(solution(vec![1, 1, 1]), false);
        assert_eq!(solution(vec![0, 0, 0]), true);
        assert_eq!(solution(vec![2, 2, 2]), true);
        assert_eq!(solution(vec![1, 2, 0]), false);
        assert_eq!(solution(vec![1, 2, 1]), true);
        assert_eq!(solution(vec![5, 5, 4, 3, 2, 2, 2, 1]), true);
        assert_eq!(solution(vec![5, 3, 5, 5, 2, 2, 1, 1]), true);
        assert_eq!(solution(vec![5, 5, 5, 4, 2, 1, 1, 1]), false);
        assert_eq!(solution(vec![1, 1, 1, 4, 2, 3, 1, 3, 1, 1]), true);
        assert_eq!(solution(vec![1, 1, 10, 4, 2, 3, 1, 3, 1, 1]), false);
    }
}
