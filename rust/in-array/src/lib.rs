pub fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut r = arr_a
        .iter()
        .filter(|&a| arr_b.iter().any(|&b| b.contains(a)))
        .map(|&a| a.into())
        .collect::<Vec<String>>();

    r.sort_unstable();
    r.dedup();
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            in_array(
                &["xyz", "live", "strong"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            ["live", "strong"]
        );

        assert_eq!(
            in_array(
                &["live", "strong", "arp"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            ["arp", "live", "strong"]
        );

        assert_eq!(
            in_array(
                &["tarp", "mice", "bull"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            [] as [&str; 0]
        );

        assert_eq!(
            in_array(
                &["live", "strong", "arp", "arp"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            ["arp", "live", "strong"]
        );
    }
}
