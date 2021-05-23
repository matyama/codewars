use std::cmp::max;

/// Solves the Graph Realization Problem:
///  - Implements the Erdős-Gallai approach in `O(n*log(n))` time:
///  http://compalg.inf.elte.hu/~tony/Kutatas/EGHH/Comb-IvanyiLucz-23Nov.pdf
///  - Note that with better sorting procedure (e.g. *Radix Sort*) the time
///  complexity can be reduced to linear
pub fn solution(degrees: Vec<usize>) -> bool {
    let mut degrees = degrees;

    let n = degrees.len();

    // Sort the degrees in non-increasing order
    // TODO: Use a Radix Sort to reduce time complexity to linear
    degrees.sort_by(|x, y| y.cmp(x));

    // Compute cumulative sum of vertex degrees
    let deg_sums: Vec<usize> = degrees
        .iter()
        .scan(0, |cumsum, &d| {
            *cumsum += d;
            Some(*cumsum)
        })
        .collect();

    // Total sum of vertex degrees
    let deg_sum = deg_sums.last().cloned().unwrap_or_default();

    // Parity test: Graph with an odd sum of degrees cannot be simple
    if deg_sum % 2 != 0 {
        return false;
    }

    // Validate n inequalities which are necessary for a graph to be simple
    let mut w = n.saturating_sub(1);
    for i in 0..n {
        // Find current weight point
        while degrees[w] <= i {
            if w == 0 {
                break;
            }
            w -= 1;
        }
        // Find current cutting point
        let y = max(i, w);
        // Degree test
        if deg_sums[i] > (i + 1) * y + deg_sum - deg_sums[y] {
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
