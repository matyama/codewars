use std::cmp::{min_by_key, Ordering};

use itertools::Itertools;

pub type Point = (f64, f64);

const NAN_POINT: Point = (f64::NAN, f64::NAN);

pub fn closest_pair(points: &[Point]) -> (Point, Point) {
    fn find_closest(xs: &[Point], ys: &[Point]) -> Pair {
        // Brute-force search for closest pair if current set of points is small
        if xs.len() <= 3 {
            return xs
                .iter()
                .tuple_combinations()
                .map(|(&a, &b)| Pair::new(a, b))
                .min_by_key(Pair::dist)
                .unwrap_or_default();
        }

        // Divide
        //  - Split all the sets to "left" and "right" halves
        //  - Points in "left" set are those which are to the left of a
        //    vertical line that bisects the original set

        let (xs_left, xs_right) = xs.split_at(xs.len() / 2 + 1);

        // Vertical split that bisects input sets
        let Some((x_split, _)) = xs_left.last() else {
            unreachable!("xs has at least 3 points: (left, mid, right)");
        };

        let (ys_left, ys_right) = ys.iter().fold(
            (Vec::new(), Vec::new()),
            |(mut ys_left, mut ys_right), p @ (x, _)| {
                if x < x_split {
                    ys_left.push(*p);
                } else {
                    ys_right.push(*p);
                }
                (ys_left, ys_right)
            },
        );

        // Conquer
        //  - Recursive call for the left and right splits

        let pair_left = find_closest(xs_left, &ys_left);
        let pair_right = find_closest(xs_right, &ys_right);

        let pair_min = min_by_key(pair_left, pair_right, Pair::dist);
        let Float(dist_min) = pair_min.dist();

        // Combine

        // Find all points in ys which are within 2 delta of the vertical split
        let ys_close = ys
            .iter()
            .filter(|(x, _)| (x - x_split).abs() < dist_min)
            .collect_vec();

        let n_ys_close = ys_close.len();

        if n_ys_close <= 1 {
            return pair_min;
        }

        // For each point in ys_close, find points within dist_min
        //  - Note that it's been proven that only 7 points need to be checked
        let close_pair_min = (0..n_ys_close - 1)
            .flat_map(|i| ((i + 1)..n_ys_close.min(i + 8)).map(move |j| (i, j)))
            .map(|(i, j)| Pair::new(*ys_close[i], *ys_close[j]))
            .min_by_key(Pair::dist)
            .unwrap_or_default();

        // Final comparison of result of the recursive call and
        // the closest pair in ys_close.
        min_by_key(pair_min, close_pair_min, Pair::dist)
    }

    let mut xs = points.to_vec();
    xs.sort_by_key(|(x, _)| Float(*x));

    let mut ys = points.to_vec();
    ys.sort_by_key(|(_, y)| Float(*y));

    let Pair { a, b, .. } = find_closest(&xs, &ys);
    (a, b)
}

#[derive(Debug)]
struct Pair {
    a: Point,
    b: Point,
    dist: Float,
}

impl Pair {
    #[inline]
    fn new(a: Point, b: Point) -> Self {
        Self {
            a,
            b,
            dist: Float(((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()),
        }
    }

    #[inline]
    fn dist(&self) -> Float {
        self.dist
    }
}

impl Default for Pair {
    #[inline]
    fn default() -> Self {
        Self {
            a: NAN_POINT,
            b: NAN_POINT,
            dist: Float(f64::INFINITY),
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
struct Float(f64);

impl PartialOrd for Float {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        !self.ge(other)
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        other.ge(self)
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        !other.ge(self)
    }

    #[inline]
    fn ge(&self, other: &Self) -> bool {
        self.0.is_nan() | (self.0 >= other.0)
    }
}

impl Ord for Float {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        #[allow(clippy::comparison_chain)]
        if self < other {
            Ordering::Less
        } else if self > other {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialEq for Float {
    #[inline]
    fn eq(&self, other: &Float) -> bool {
        if self.0.is_nan() {
            other.0.is_nan()
        } else {
            self.0 == other.0
        }
    }
}

impl Eq for Float {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(&[(2.0, 2.0), (6.0, 3.0)], ((2.0, 2.0), (6.0, 3.0)))]
    #[case(
        &[
            (2.0, 2.0),
            (2.0, 8.0),
            (5.0, 5.0),
            (6.0, 3.0),
            (6.0, 7.0),
            (7.0, 4.0),
            (7.0, 9.0),
        ],
        ((6.0, 3.0), (7.0, 4.0))
    )]
    #[case(
        &[
            (2.0, 2.0),
            (2.0, 8.0),
            (5.0, 5.0),
            (5.0, 5.0),
            (6.0, 3.0),
            (6.0, 7.0),
            (7.0, 4.0),
            (7.0, 9.0),
        ],
        ((5.0, 5.0), (5.0, 5.0))
    )]
    fn sample_tests(#[case] points: &[(f64, f64)], #[case] expected: (Point, Point)) {
        let actual = closest_pair(points);
        assert_eq!(actual, expected);
    }
}
