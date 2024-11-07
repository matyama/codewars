use std::cell::RefCell;
use std::cmp::{Ordering, Reverse};
use std::collections::binary_heap::{self, BinaryHeap};
use std::collections::hash_map::{Entry, HashMap};
use std::iter::{Copied, Cycle};

use num::integer::{gcd, sqrt};
use num::CheckedMul;

/// Returns an infinite stream of prime numbers
///
/// The implementation is based on the following papers:
///  - [_The Genuine Sieve of Eratosthenes_][sieve] for the sieve algorithm
///  - [_An Introduction to Prime Number Sieves_][wheel] for the generic wheel
///
/// [sieve]: https://doi.org/10.1017/S0956796808007004
/// [wheel]: https://research.cs.wisc.edu/techreports/1990/TR909.pdf
pub fn stream() -> impl Iterator<Item = u32> {
    let Wheel { primes, spin } = Wheel::<8, _>::new();
    primes.into_iter().chain(Sieve::new(spin))
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
struct Wheel<const N: usize, I: Iterator<Item = u32>> {
    primes: [u32; N],
    spin: Spin<I>,
}

impl<const N: usize> Wheel<N, SpinIter<u32>> {
    fn new() -> Self {
        assert!(N > 0, "N cannot be 0");

        // find the first N primes (using a naive algorithm)
        let primes = Self::primes();

        std::thread_local! {
            // NOTE: we're using a static cache of wheels so the returned spin iter does not have
            // to clone the whole wheel Vec, and to keep the Wheel 'static
            static WHEELS: RefCell<HashMap<usize, &'static [u32]>> = RefCell::new(HashMap::new());
        }

        let wheel = WHEELS.with(move |wheels| match wheels.borrow_mut().entry(N) {
            Entry::Vacant(e) => {
                let m = primes.iter().product::<u32>() as usize;

                // NOTE: u16 gaps are sufficient for u32 primes
                // https://en.wikipedia.org/wiki/Prime_gap
                let mut wheel = (0..m)
                    .map(|x| u16::from(gcd(x, m) == 1))
                    .collect::<Vec<_>>();

                let mut y = m - 1;
                wheel[y] = 2;

                for x in (0..(m - 1)).rev() {
                    if wheel[x] == 0 {
                        continue;
                    }

                    wheel[x] = (y - x) as u16;
                    y = x;
                }

                let wheel = wheel
                    .into_iter()
                    .filter_map(|x| if x != 0 { Some(x as u32) } else { None })
                    .collect::<Vec<_>>()
                    .into_boxed_slice();

                // NOTE: since N is const, leaking this should be ok
                e.insert(Box::leak(wheel))
            }

            Entry::Occupied(e) => *e.get(),
        });

        // the very first item is the next prime p - 1, so initialize n to 1 to offset this
        let spin = Spin {
            steps: wheel.iter().copied().cycle(),
            n: 1,
        };

        Self { primes, spin }
    }

    fn primes() -> [u32; N] {
        debug_assert!(N > 0, "N cannot be 0");

        let mut primes = [0; N];
        primes[0] = 2;
        let mut n = 3;
        let mut k = 1;

        while k < N {
            if (0..k).all(|i| n % primes[i] != 0) {
                primes[k] = n;
                k += 1;
            }
            n += 2;
        }

        primes
    }
}

type SpinIter<T> = Copied<std::slice::Iter<'static, T>>;

#[derive(Clone)]
struct Spin<I> {
    steps: Cycle<I>,
    n: u32,
}

impl<I> Iterator for Spin<I>
where
    I: Iterator<Item = u32> + Clone,
{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: self.steps is a Cycle
        self.n += unsafe { self.steps.next().unwrap_unchecked() };
        Some(self.n)
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
    <I as Iterator>::Item: CheckedMul<Output = T>,
    T: Copy,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().and_then(|x| x.checked_mul(&self.1))
    }
}

type TableEntry<I> = Reverse<List<u32, MulBy<I, u32>>>;

struct Sieve<I> {
    x: Option<u32>,
    xs: I,
    // NOTE: this is a min-heap, not a max-heap of reversed lists
    table: BinaryHeap<TableEntry<I>>,
    max: u32,
}

impl<I> Sieve<I>
where
    I: Iterator<Item = u32> + Clone,
{
    fn new(mut xs: I) -> Self {
        let mut this = Self {
            x: xs.next(),
            xs,
            table: BinaryHeap::new(),
            max: sqrt(u32::MAX),
        };

        if let Some(x) = this.x {
            this.insert_prime(x);
        }

        this
    }

    /// Registers new lazy [`List`] of multiples of a prime `p` in the [`table`](Self::table)
    ///
    /// Note that since we insert `[p^2, ...]` into the table, we ignore any `p > sqrt(u32::MAX)`.
    /// This is fine, because we can never get to them under the `u32` prime representation (see
    /// the [`is_composite`](Self::is_composite) check).
    fn insert_prime(&mut self, p: u32) {
        if p > self.max {
            return;
        }

        self.table.push(Reverse(List {
            head: p * p,
            tail: MulBy::new(&self.xs, p),
        }));
    }

    /// Returns `true` iff the first entry's head `n` satisfies `n <= x`
    #[inline]
    fn is_composite(&self, x: u32) -> bool {
        matches!(self.table.peek(), Some(Reverse(List { head, .. })) if *head <= x)
    }

    /// Remove all numbers `n` stored in the [`table`](Self::table) such that `n <= x`
    fn adjust(&mut self, x: u32) {
        // stop when we've reached x
        while self.is_composite(x) {
            // take out the first list, advance it, and re-insert the rest back
            let list = self.table.peek_mut();

            // SAFETY: peek_mut must return Some due to the peek check in is_composite
            let mut list = unsafe { list.unwrap_unchecked() };

            let Reverse(List { head, tail }) = &mut *list;

            // advance the list
            match tail.next() {
                Some(h) => *head = h,
                None => {
                    let _ = binary_heap::PeekMut::<'_, TableEntry<I>>::pop(list);
                }
            }
        }
    }
}

impl<I> Iterator for Sieve<I>
where
    I: Iterator<Item = u32> + Clone,
{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if let x @ Some(_) = self.x.take() {
            return x;
        }

        let x = loop {
            let x = self.xs.next()?;

            if self.is_composite(x) {
                // x is not a prime, so adjust the table and continue searching
                self.adjust(x);
            } else {
                break x;
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
    #[case([2])]
    #[case([2, 3])]
    #[case([2, 3, 5])]
    #[case([2, 3, 5, 7])]
    #[case([2, 3, 5, 7, 11])]
    #[timeout(Duration::from_millis(100))]
    #[trace]
    fn init_primes<const N: usize>(#[case] expected: [u32; N]) {
        assert_eq!(expected, Wheel::primes());
    }

    #[rstest]
    #[case([2], &[3, 5, 7, 9, 11, 13, 15])] // [2]
    #[case([2, 3], &[5, 7, 11, 13, 17, 19, 23, 25, 29])] // [4, 2]
    #[case([2, 3, 5], &[7, 11, 13, 17, 19, 23, 29, 31, 37, 41])] // [6, 4, 2, 4, 2, 4, 6, 2]
    #[trace]
    fn small_wheels<const N: usize>(#[case] expected_primes: [u32; N], #[case] expected: &[u32]) {
        let Wheel { primes, spin } = Wheel::<N, _>::new();
        let actual = spin.take(expected.len()).collect::<Vec<_>>();
        assert_eq!(expected_primes, primes);
        assert_eq!(expected, actual);
    }

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

    #[ignore]
    #[rstest]
    #[case(64_955_634, 1_294_268_491)]
    #[trace]
    fn large_prime(#[case] n: usize, #[case] expected: u32) {
        assert_eq!(Some(expected), stream().nth(n - 1));
    }
}
