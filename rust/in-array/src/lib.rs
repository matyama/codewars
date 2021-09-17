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
    use rstest::*;

    #[rstest]
    #[case(
        &["xyz", "live", "strong"],
        &["lively", "alive", "harp", "sharp", "armstrong"],
        &["live", "strong"],
    )]
    #[case(
        &["live", "strong", "arp"],
        &["lively", "alive", "harp", "sharp", "armstrong"],
        &["arp", "live", "strong"],
    )]
    #[case(
        &["tarp", "mice", "bull"],
        &["lively", "alive", "harp", "sharp", "armstrong"],
        &[],
    )]
    #[case(
        &["live", "strong", "arp", "arp"],
        &["lively", "alive", "harp", "sharp", "armstrong"],
        &["arp", "live", "strong"],
    )]
    fn it_works(#[case] arr_a: &[&str], #[case] arr_b: &[&str], #[case] expected: &[&str]) {
        assert_eq!(in_array(arr_a, arr_b), expected);
    }
}
