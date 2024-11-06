use std::collections::VecDeque;

pub fn encode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    let mut rails = Vec::with_capacity(num_rails);
    rails.resize_with(num_rails, String::new);

    for (c, rail) in text.chars().zip(RailsIter::new(num_rails)) {
        rails[rail].push(c);
    }

    let mut encoded = String::with_capacity(text.len());
    for rail in rails {
        encoded.push_str(&rail);
    }

    encoded
}

pub fn decode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    let n = text.len();

    let sizes = RailsIter::new(num_rails)
        .take(n)
        .fold(vec![0; num_rails], |mut rails, rail| {
            rails[rail] += 1;
            rails
        });

    let (mut rails, _) = sizes.into_iter().fold(
        (Vec::with_capacity(num_rails), 0),
        |(mut rails, start), cnt| {
            let end = start + cnt;
            rails.push(text[start..end].chars().collect::<VecDeque<_>>());
            (rails, end)
        },
    );

    let mut decoded = String::with_capacity(n);

    for rail in RailsIter::new(num_rails).take(n) {
        if let Some(c) = rails[rail].pop_front() {
            decoded.push(c);
        }
    }

    decoded
}

struct RailsIter {
    rail: usize,
    inc: bool,
    max: usize,
}

impl RailsIter {
    #[inline]
    fn new(num_rails: usize) -> Self {
        Self {
            rail: 0,
            inc: true,
            max: num_rails - 1,
        }
    }
}

impl Iterator for RailsIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.rail;

        match (self.rail, self.inc) {
            (r, true) if r == self.max => {
                self.rail -= 1;
                self.inc = false;
            }
            (_, true) => self.rail += 1,
            (r, false) if r == 0 => {
                self.rail += 1;
                self.inc = true;
            }
            (_, false) => self.rail -= 1,
        }

        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("WEAREDISCOVEREDFLEEATONCE", "WECRLTEERDSOEEFEAOCAIVDEN")]
    #[case("Hello, World!", "Hoo!el,Wrdl l")]
    #[trace]
    fn three_rails(#[case] decoded: &str, #[case] encoded: &str) {
        assert_eq!(encode_rail_fence_cipher(decoded, 3), encoded, "encode");
        assert_eq!(decode_rail_fence_cipher(encoded, 3), decoded, "decode");
    }
}
