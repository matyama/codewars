use std::cmp::min;

/// Resources on the Graph Realization Problem:
///  - https://en.wikipedia.org/wiki/Graph_realization_problem
///  - https://en.wikipedia.org/wiki/Erd%C5%91s%E2%80%93Gallai_theorem
pub fn solution(degrees: Vec<usize>) -> bool {
    // Sort the degrees in non-increasing order
    let mut degrees = degrees;
    degrees.sort_by(|x, y| y.cmp(x));

    let n = degrees.len();

    // Graph with an odd sum of degrees cannot be simple
    if degrees.iter().sum::<usize>() % 2 != 0 {
        return false;
    }

    // Validate n inequalities which are necessary for a graph to be simple
    for k in 0..n {
        let sum_k: usize = degrees[..=k].iter().sum();
        let sum_n: usize = degrees[(k + 1)..].iter().map(|&d| min(d, k + 1)).sum();
        if sum_k > k * (k + 1) + sum_n {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solution(vec![]), true);
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
