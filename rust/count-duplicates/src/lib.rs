use std::collections::HashMap;

pub fn count_duplicates(text: &str) -> u32 {
    let mut counts = HashMap::new();
    for c in text.to_lowercase().chars() {
        let cnt = counts.entry(c).or_insert(0);
        *cnt += 1;
    }
    counts.values().filter(|&&c| c > 1).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case::abcde("abcde", 0)]
    #[case::abcdea("abcdea", 1)]
    #[case::indivisibility("indivisibility", 1)]
    fn it_works(#[case] text: &str, #[case] exprected: u32) {
        assert_eq!(count_duplicates(text), exprected);
    }
}
