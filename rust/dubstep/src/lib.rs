pub fn song_decoder(song: &str) -> String {
    song.split("WUB")
        .filter(|part| !part.is_empty())
        .collect::<Vec<&str>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::song_decoder;
    use rstest::*;

    #[rstest]
    #[case("WUBAWUBWUBC", "A C")]
    #[case("AWUBWUBWUBBWUBWUBWUBC", "A B C")]
    #[case("WUBAWUBBWUBCWUB", "A B C")]
    #[case("AWUBBWUBC", "A B C")]
    fn returns_expected(#[case] song: &str, #[case] exprected: &str) {
        assert_eq!(song_decoder(song), exprected);
    }
}
