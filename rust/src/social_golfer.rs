use std::{
    collections::{HashMap, HashSet},
    ops::ControlFlow,
};

use itertools::Itertools;

/// Validates that
///  1. Each golfer plays exactly once every day
///  2. The number and size of the groups is the same every day, and
///  3. Each player plays with every other player at most once
pub fn valid(solution: Vec<Vec<&str>>) -> bool {
    let Some(day1) = solution.first() else {
        return true;
    };

    let Some(group1) = day1.first() else {
        return true;
    };

    let days_played = solution.len();
    let group_count = day1.len();
    let group_size = group1.len();

    let mut matching: HashMap<_, Record> = HashMap::new();

    for (day, groups) in solution.into_iter().enumerate() {
        if groups.len() != group_count {
            return false;
        }

        for group in groups {
            if group.len() != group_size {
                return false;
            }

            for (p1, p2) in group.chars().combinations(2).map(|pair| (pair[0], pair[1])) {
                if matching.entry(p1).or_default().insert(day, p2).is_break() {
                    return false;
                }

                if matching.entry(p2).or_default().insert(day, p1).is_break() {
                    return false;
                }
            }
        }
    }

    matching
        .into_iter()
        .all(|(_, Record { pairs: _, days })| days.len() == days_played)
}

#[derive(Debug, Default)]
struct Record {
    pairs: HashSet<char>,
    days: HashSet<usize>,
}

impl Record {
    fn insert(&mut self, day: usize, other: char) -> ControlFlow<()> {
        if self.pairs.insert(other) {
            self.days.insert(day);
            ControlFlow::Continue(())
        } else {
            ControlFlow::Break(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    type Solution<'a> = Vec<Vec<&'a str>>;

    #[rstest]
    #[case::two_players(vec![vec!["AB"]], true)]
    #[case::four_players(vec![vec!["AB", "CD"], vec!["AD", "BC"], vec!["BD", "AC"]], true)]
    #[case::players_twice(vec![vec!["ABC", "DEF"], vec!["ADE", "CBF"]], false)]
    fn validation(#[case] s: Solution, #[case] expected: bool) {
        assert_eq!(valid(s), expected);
    }

    #[fixture]
    fn wolfram_example() -> Solution<'static> {
        vec![
            vec!["ABCD", "EFGH", "IJKL", "MNOP", "QRST"],
            vec!["AEIM", "BJOQ", "CHNT", "DGLS", "FKPR"],
            vec!["AGKO", "BIPT", "CFMS", "DHJR", "ELNQ"],
            vec!["AHLP", "BKNS", "CEOR", "DFIQ", "GJMT"],
            vec!["AFJN", "BLMR", "CGPQ", "DEKT", "HIOS"],
        ]
    }

    #[rstest]
    fn wolfram_math_world_test(wolfram_example: Solution) {
        assert!(valid(wolfram_example));
    }

    #[fixture]
    fn invalid_solution() -> Solution<'static> {
        vec![
            vec!["AB", "CD", "EF", "GH"],
            vec!["AC", "BD", "EG", "FH"],
            vec!["AD", "CE"],
            vec!["AE", "BG", "CH", "FD"],
        ]
    }

    #[rstest]
    fn groups_different_test(invalid_solution: Solution) {
        assert!(!valid(invalid_solution));
    }

    #[fixture]
    fn unknown_player() -> Solution<'static> {
        vec![
            vec!["ABCD", "EFGH", "IJKL", "MNOP", "QRST"],
            vec!["AEIM", "BJOQ", "CHNT", "DGLS", "FKPR"],
            vec!["AGKO", "BIPT", "CFMS", "DHJR", "ELNQ"],
            vec!["AHLP", "BKNS", "CEOR", "DFXQ", "GJMT"],
            vec!["AFJN", "BLMR", "CGPQ", "DEKT", "HIOS"],
        ]
    }

    #[rstest]
    fn unknown_player_test(unknown_player: Solution) {
        assert!(!valid(unknown_player));
    }
}
