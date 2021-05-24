use std::collections::HashSet;

/// Implements Integer Partitioning and collects statistics about unique
/// partition products.
///  - [Partition generation](https://bit.ly/34cxxi4)
pub fn part(n: i64) -> String {
    let mut prods = HashSet::new();

    // Initialize partition with n
    //  - Note that partition is kept in non-increasing order!
    let mut k = 0;
    let mut partition = vec![0; n as usize];
    partition[k] = n;

    // Generate all integer partitions of n and collect their unique products
    'collect: loop {
        // Record product of current partition
        let prod = partition.iter().take(k + 1).product();
        prods.insert(prod);

        // Generate new partition

        // Find the last value > 1 and stop after k reached 0
        //  - When `k < 0` all partition values are ones and we can stop
        let mut remaining_sum = 0;
        while partition[k] == 1 {
            remaining_sum += partition[k];
            k = match k.checked_sub(1) {
                Some(k) => k,
                None => break 'collect,
            }
        }

        // Transfer 1 from value at position k to the remaining sum of values
        partition[k] -= 1;
        remaining_sum += 1;

        // This condition checks for ordering violations
        //  - We then divide the remaining sum to different values of size
        //  `partition[k]` and copy these values at positions after k
        while remaining_sum > partition[k] {
            partition[k + 1] = partition[k];
            remaining_sum -= partition[k];
            k += 1;
        }

        // Insert the sum of remaining values at the next position
        partition[k + 1] = remaining_sum;
        k += 1;
    }

    // Collect and sort the products for median
    let mut prods: Vec<i64> = prods.into_iter().collect();
    prods.sort();

    let n_prods = prods.len();

    // Find minimum and maximum for range
    let min = prods.first().cloned().unwrap_or_default();
    let max = prods.last().cloned().unwrap_or_default();

    // Compute the median
    let median = if n_prods % 2 == 0 {
        (prods[n_prods / 2 - 1] + prods[n_prods / 2]) as f64 / 2.0
    } else {
        prods[n_prods / 2] as f64
    };

    // Compute mean and output the results
    format!(
        "Range: {} Average: {:.2} Median: {:.2}",
        max - min,
        prods.into_iter().sum::<i64>() as f64 / n_prods as f64,
        median
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testequal(ans: &str, sol: &str) {
        assert!(ans == sol, "Expected \"{}\", got \"{}\".", sol, ans);
    }

    #[test]
    fn returns_expected() {
        testequal(&part(1), "Range: 0 Average: 1.00 Median: 1.00");
        testequal(&part(2), "Range: 1 Average: 1.50 Median: 1.50");
        testequal(&part(3), "Range: 2 Average: 2.00 Median: 2.00");
        testequal(&part(4), "Range: 3 Average: 2.50 Median: 2.50");
        testequal(&part(5), "Range: 5 Average: 3.50 Median: 3.50");
    }
}
