use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::iter::{Copied, Cycle};

/// Returns an infinite stream of prime numbers
///
/// Implementation based on [_The Genuine Sieve of Eratosthenes_][doi] paper.
///
/// [doi]: https://doi.org/10.1017/S0956796808007004
pub fn stream() -> impl Iterator<Item = u32> {
    let Wheel { primes, spin } = Wheel::<4, _>::default();
    primes.into_iter().chain(Sieve::new(spin)).map(|p| p as u32)
}

/// The [`Wheel`] optimizes the input to the [`Sieve`] as follows:
///  - [`primes`](Self::primes) is the initial list of `N` of known primes that define the wheel
///  - [`spin`](Self::spin) is the input stream of numbers to the [`Sieve`]
///
/// The [`Spin`] is a cyclic iterator that filters out known composites of the initial `primes`.
///
/// For instance, a `Wheel<1>` could be imagined as an initial prime `[2]` and a filter of all the
/// _even_ numbers `(3..).filter(|n| n % 2 != 0)`. With larger `N`, this filter is implemented as
/// an  iterator called [`Spin`].
struct Wheel<const N: usize, I: Iterator<Item = u64>> {
    primes: [u64; N],
    spin: Spin<I>,
}

type SpinIter<T> = Cycle<Copied<std::slice::Iter<'static, T>>>;

impl Default for Wheel<4, SpinIter<u64>> {
    fn default() -> Self {
        use once_cell::sync::Lazy;

        static STEPS: &[u64] = &[
            2, 4, 2, 4, 6, 2, 6, 4, 2, 4, 6, 6, 2, 6, 4, 2, 6, 4, 6, 8, 4, 2, 4, 2, 4, 8, 6, 4, 6,
            2, 4, 6, 2, 6, 6, 4, 2, 4, 6, 2, 6, 4, 2, 4, 2, 10, 2, 10,
        ];

        static SPIN: Lazy<Spin<SpinIter<u64>>> = Lazy::new(|| Spin {
            steps: STEPS.iter().copied().cycle(),
            n: 11,
        });

        Self {
            primes: [2, 3, 5, 7],
            spin: SPIN.clone(),
        }
    }
}

#[derive(Clone)]
struct Spin<I> {
    steps: I,
    n: u64,
}

impl<I> Iterator for Spin<I>
where
    I: Iterator<Item = u64>,
{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.n;
        self.n += self.steps.next()?;
        Some(n)
    }
}

/// An [`Iterator`] that given an element `p: T` and iterator `I`, yields for each `x: I::Item` a
/// multiple `x * p`.
struct MulBy<I, T>(I, T);

impl<T, I: Iterator<Item = T> + Clone> MulBy<I, T> {
    #[inline]
    fn new(xs: &I, p: T) -> Self {
        Self(xs.clone(), p)
    }
}

impl<I, T> Iterator for MulBy<I, T>
where
    I: Iterator<Item = T>,
    <I as Iterator>::Item: std::ops::Mul<T, Output = T>,
    T: Copy,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| x * self.1)
    }
}

struct Sieve<I> {
    x: Option<u64>,
    xs: I,
    // NOTE: this is a min-heap, not a max-heap of reversed lists
    table: BinaryHeap<Reverse<List<u64, MulBy<I, u64>>>>,
}

impl<I> Sieve<I>
where
    I: Iterator<Item = u64> + Clone,
{
    fn new(mut xs: I) -> Self {
        let Some(x) = xs.next() else {
            return Self {
                x: None,
                xs,
                table: BinaryHeap::new(),
            };
        };

        let mut this = Self {
            x: Some(x),
            xs,
            table: BinaryHeap::new(),
        };

        this.insert_prime(x);

        this
    }

    /// Registers new lazy [`List`] of multiples of a prime `p` in the [`table`](Self::table)
    fn insert_prime(&mut self, p: u64) {
        self.table.push(Reverse(List {
            head: p * p,
            tail: MulBy::new(&self.xs, p),
        }));
    }

    /// Remove all numbers `n` stored in the [`table`](Self::table) such that `n <= x`
    fn adjust(&mut self, x: u64) {
        loop {
            // take out the first list, advance it, and re-insert the rest back
            let mut list = self.table.peek_mut();

            let Some(Reverse(List { head, tail })) = list.as_deref_mut() else {
                break;
            };

            // actually, stop when we've reached x
            if *head > x {
                break;
            }

            // advance the list (we should never run out given an infinite input stream)
            match tail.next() {
                Some(h) => {
                    *head = h;
                }
                None => {
                    drop(list);
                    let _ = self.table.pop();
                }
            }
        }
    }
}

impl<I> Iterator for Sieve<I>
where
    I: Iterator<Item = u64> + Clone,
{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if let x @ Some(_) = self.x.take() {
            return x;
        }

        let x = loop {
            let x = self.xs.next()?;

            match self.table.peek() {
                Some(Reverse(List { head, .. })) if *head <= x => {
                    // x is not a prime, so adjust the table and continue searching
                    self.adjust(x);
                }
                _ => break x,
            }
        };

        // register new iterator for prime x in the table
        self.insert_prime(x);

        Some(x)
    }
}

/// Lazy list represented as a materialized head and an [`Iterator`] as the tail
///
/// Since this type is used to form a heap-backed table, it has an [`Eq`] and [`Ord`] instance
/// based on the [`head`](Self::head).
struct List<T, I> {
    head: T,
    tail: I,
}

impl<T: PartialEq, I> PartialEq for List<T, I> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head
    }
}

impl<T: Eq, I> Eq for List<T, I> {}

impl<T: Ord, I> PartialOrd for List<T, I> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord, I> Ord for List<T, I> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.head.cmp(&other.head)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    use std::time::Duration;

    #[rstest]
    #[case(0, &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29])]
    #[case(10, &[31, 37, 41, 43, 47, 53, 59, 61, 67, 71])]
    #[case(100, &[547, 557, 563, 569, 571, 577, 587, 593, 599, 601])]
    #[case(1000, &[7927, 7933, 7937, 7949, 7951, 7963, 7993, 8009, 8011, 8017])]
    #[trace]
    fn test_segment(#[case] start: usize, #[case] expected: &[u32]) {
        let actual = stream()
            .skip(start)
            .take(expected.len())
            .collect::<Vec<_>>();

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(1_000)]
    #[case(10_000)]
    #[case(100_000)]
    #[case(1_000_000)]
    #[timeout(Duration::from_secs(12))]
    #[trace]
    fn bench(#[case] n: usize) {
        let _ = stream().take(n).count();
    }
}
