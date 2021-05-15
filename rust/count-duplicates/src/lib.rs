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

    #[test]
    fn test_abcde() {
        assert_eq!(count_duplicates("abcde"), 0);
    }

    #[test]
    fn test_abcdea() {
        assert_eq!(count_duplicates("abcdea"), 1);
    }

    #[test]
    fn test_indivisibility() {
        assert_eq!(count_duplicates("indivisibility"), 1);
    }
}
