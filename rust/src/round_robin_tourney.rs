use std::collections::VecDeque;

pub fn build_matches_table(n: u32) -> Vec<Vec<(u32, u32)>> {
    // NOTE: assumption stated in the exercise description
    assert!(n > 0 && n % 2 == 0);

    let mut players = (1..=n).collect::<VecDeque<_>>();

    let n = n as usize;
    let m = n / 2;

    let mut rounds = Vec::with_capacity(n - 1);

    for _ in 0..(n - 1) {
        rounds.push((0..m).map(|i| (players[i], players[n - i - 1])).collect());

        // NOTE: these take O(1) with a VecDeque
        players.rotate_right(1);
        players.swap(0, 1);
    }

    rounds
}

#[cfg(test)]
mod sample_tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn with_2_teams() {
        let actual = build_matches_table(2);

        assert!(
            actual.len() == 1,
            "The table should have 1 round, but yours had {}\n",
            actual.len()
        );

        assert!(
            actual[0].len() == 1,
            "The round should have 1 match, but yours had {}\n",
            actual[0].len()
        );

        let m = actual[0][0];
        let (a, b) = if m.0 > m.1 { (m.1, m.0) } else { m };
        assert_eq!((a, b), (1, 2), "Match should be 1 vs 2")
    }

    #[test]
    fn with_4_teams() {
        let actual = build_matches_table(4);

        assert!(
            actual.len() == 3,
            "The table should have 3 rounds, yours had {}\n",
            actual.len()
        );

        let mut expected_matches: HashSet<(u32, u32)> =
            HashSet::from_iter([(1, 2), (3, 4), (1, 3), (2, 4), (1, 4), (2, 3)]);

        let expected_teams = [1, 2, 3, 4];

        let mut seen = HashSet::new();

        for (mut i, round) in actual.iter().enumerate() {
            i += 1;

            assert!(
                round.len() == 2,
                "Round {i}: each round should have 2 matches, yours had {}:\n{round:?}\n",
                round.len()
            );

            let mut participants = Vec::new();

            for (mut a, mut b) in round {
                if a > b {
                    (a, b) = (b, a);
                }

                assert!(
                    !seen.contains(&(a, b)),
                    "Round {i}: Match ({a} vs {b}) has already been played!\n"
                );

                seen.insert((a, b));

                assert!(
                    expected_matches.contains(&(a, b)),
                    "Round {i}: ({a} vs {b}) is not a valid match!\n"
                );

                participants.extend_from_slice(&[a, b]);
                expected_matches.remove(&(a, b));
            }

            participants.sort();

            assert!(
                participants == expected_teams,
                "Round {i}: Every team must participate in a round.\n
                {participants:?} should equal {expected_teams:?}\n",
            );
        }

        assert!(
            expected_matches.is_empty(),
            "{} matches were not played:\n{expected_matches:?}\n",
            expected_matches.len()
        );
    }
}
