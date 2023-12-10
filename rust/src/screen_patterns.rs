use once_cell::sync::Lazy;

const N: usize = 9;

static SKIPS: Lazy<[[Option<usize>; N]; N]> = Lazy::new(|| {
    let mut skips: [[Option<usize>; N]; N] = [[None; N]; N];

    for i in 0..N / 2 {
        // A -- I, B -- H, C -- G and D -- F skips E
        skips[i][N - i - 1] = Some(N / 2);
        skips[N - i - 1][i] = Some(N / 2);

        if i % 2 == 1 {
            // A -- C skips B and A -- G skips D
            skips[0][2 * i] = Some(i);
            skips[2 * i][0] = Some(i);

            // C -- I skips F and G -- I skips H
            skips[N - 1 - 2 * i][N - 1] = Some(N - i - 1);
            skips[N - 1][N - 1 - 2 * i] = Some(N - i - 1);
        }
    }

    skips
});

struct Pattern {
    last: usize,
    length: usize,
    max_length: usize,
    pts: [bool; N],
}

impl Pattern {
    fn new(last: usize, max_length: usize) -> Self {
        let mut pts = [false; N];
        pts[last] = true;
        Self {
            last,
            length: 1,
            max_length,
            pts,
        }
    }

    fn complete(&self) -> bool {
        self.length == self.max_length
    }

    fn push(&mut self, next: usize) -> usize {
        let prev = self.last;
        self.pts[next] = true;
        self.length += 1;
        self.last = next;
        prev
    }

    fn rollback(&mut self, prev: usize) {
        self.pts[self.last] = false;
        if self.last != prev {
            self.length -= 1;
        }
        self.last = prev
    }

    ///  Validity conditions:
    ///    1. `next` must not yet be in the pattern
    ///    2. `last` -> `next` must either be a skip over a point `p` in the pattern
    ///    3. or be a direct line (i.e. not over a skippable point)
    fn valid_with(&self, next: usize) -> bool {
        !self.pts[next] && SKIPS[self.last][next].map_or(true, |p| self.pts[p])
    }
}

fn count(pattern: &mut Pattern) -> u64 {
    if pattern.complete() {
        return 1;
    }

    let mut cnt = 0;

    for next in 0..N {
        if pattern.valid_with(next) {
            let prev = pattern.push(next);
            cnt += count(pattern);
            pattern.rollback(prev);
        }
    }

    cnt
}

pub fn count_patterns(from: char, length: u8) -> u64 {
    if length < 2 {
        return length.into();
    }

    let from = (from as u8 - b'A') as usize;
    let length = length as usize;

    if length > N {
        return 0;
    }

    let mut pattern = Pattern::new(from, length);
    count(&mut pattern)
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use rstest::*;

    #[rstest]
    #[case::c0('A', 0, 0)]
    #[case::a10('A', 10, 0)]
    #[case::b1('B', 1, 1)]
    #[case::c2('C', 2, 5)]
    #[case::d3('D', 3, 37)]
    #[case::e4('E', 4, 256)]
    #[case::e8('E', 8, 23280)]
    fn pattern_count(#[case] from: char, #[case] length: u8, #[case] expected: u64) {
        assert_eq!(count_patterns(from, length), expected);
    }

    #[rstest]
    fn android_screen() {
        assert_eq!(
            "ABCDEFGHI"
                .chars()
                .cartesian_product(4..=N as u8)
                .map(|(from, length)| count_patterns(from, length))
                .sum::<u64>(),
            389112
        );
    }
}
