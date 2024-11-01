use std::cmp::max;

/// Solves the Graph Realization Problem:
///  - Implements the Erdős-Gallai approach in `O(n*log(n))` time:
///  <http://compalg.inf.elte.hu/~tony/Kutatas/EGHH/Comb-IvanyiLucz-23Nov.pdf>
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
    use rstest::*;

    #[rstest]
    #[case(vec![])]
    #[case(vec![0])]
    #[case(vec![1, 1])]
    #[case(vec![0, 0, 0])]
    #[case(vec![2, 2, 2])]
    #[case(vec![1, 2, 1])]
    #[case(vec![5, 5, 4, 3, 2, 2, 2, 1])]
    #[case(vec![5, 3, 5, 5, 2, 2, 1, 1])]
    #[case(vec![1, 1, 1, 4, 2, 3, 1, 3, 1, 1])]
    fn positive_examples(#[case] degrees: Vec<usize>) {
        assert!(solution(degrees));
    }

    #[rstest]
    #[case(vec![2])]
    #[case(vec![1, 1, 1])]
    #[case(vec![1, 2, 0])]
    #[case(vec![5, 5, 5, 4, 2, 1, 1, 1])]
    #[case(vec![1, 1, 10, 4, 2, 3, 1, 3, 1, 1])]
    fn negative_examples(#[case] degrees: Vec<usize>) {
        assert!(!solution(degrees));
    }
}
