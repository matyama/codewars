use itertools::Itertools;
use std::collections::HashSet;

pub struct Sudoku {
    data: Vec<Vec<u32>>,
}

impl Sudoku {
    pub fn is_valid(&self) -> bool {
        // Check dimensions
        let n = self.data.len();
        let s = (n as f64).sqrt();

        if n == 0 || s.fract() != 0.0 {
            return false;
        }

        let s = s as usize;

        // Check rows and columns
        let domain = 1..=n as u32;
        let mut row_vals = HashSet::with_capacity(n);
        let mut col_vals = HashSet::with_capacity(n);

        for i in 0..n {
            // Check row structure
            if self.data[i].len() != n {
                return false;
            }

            for j in 0..n {
                // Check that values are between 1 and n (inclusive)
                if !domain.contains(&self.data[i][j]) {
                    return false;
                }

                row_vals.insert(self.data[i][j]);
                col_vals.insert(self.data[j][i]);
            }

            if row_vals.len() != n || col_vals.len() != n {
                return false;
            }

            row_vals.clear();
            col_vals.clear();
        }

        // Check blocks
        let row_steps = (0..n).step_by(s);
        let col_steps = (0..n).step_by(s);

        // For upper left position of each block
        row_steps.cartesian_product(col_steps).all(|(row, col)| {
            (0..s)
                .cartesian_product(0..s)
                .map(|(ro, co)| self.data[row + ro][col + co])
                .unique()
                .count()
                .eq(&n)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_sudoku() {
        let good_sudoku_1 = Sudoku {
            data: vec![
                vec![7, 8, 4, 1, 5, 9, 3, 2, 6],
                vec![5, 3, 9, 6, 7, 2, 8, 4, 1],
                vec![6, 1, 2, 4, 3, 8, 7, 5, 9],
                vec![9, 2, 8, 7, 1, 5, 4, 6, 3],
                vec![3, 5, 7, 8, 4, 6, 1, 9, 2],
                vec![4, 6, 1, 9, 2, 3, 5, 8, 7],
                vec![8, 7, 6, 3, 9, 4, 2, 1, 5],
                vec![2, 4, 3, 5, 6, 1, 9, 7, 8],
                vec![1, 9, 5, 2, 8, 7, 6, 3, 4],
            ],
        };

        let good_sudoku_2 = Sudoku {
            data: vec![
                vec![1, 4, 2, 3],
                vec![3, 2, 4, 1],
                vec![4, 1, 3, 2],
                vec![2, 3, 1, 4],
            ],
        };
        assert!(good_sudoku_1.is_valid());
        assert!(good_sudoku_2.is_valid());
    }

    #[test]
    fn bad_sudoku() {
        let bad_sudoku_1 = Sudoku {
            data: vec![
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            ],
        };

        let bad_sudoku_2 = Sudoku {
            data: vec![
                vec![1, 2, 3, 4, 5],
                vec![1, 2, 3, 4],
                vec![1, 2, 3, 4],
                vec![1],
            ],
        };

        let bad_sudoku_3 = Sudoku {
            data: vec![vec![2]],
        };

        let bad_sudoku_4 = Sudoku { data: vec![vec![]] };

        assert!(!bad_sudoku_1.is_valid());
        assert!(!bad_sudoku_2.is_valid());
        assert!(!bad_sudoku_3.is_valid());
        assert!(!bad_sudoku_4.is_valid());
    }
}
