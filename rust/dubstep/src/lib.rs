pub fn song_decoder(song: &str) -> String {
    song.split("WUB")
        .filter(|part| !part.is_empty())
        .collect::<Vec<&str>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::song_decoder;

    #[test]
    fn returns_expected() {
        assert_eq!(song_decoder("WUBAWUBWUBC"), "A C");
        assert_eq!(song_decoder("AWUBWUBWUBBWUBWUBWUBC"), "A B C");
        assert_eq!(song_decoder("WUBAWUBBWUBCWUB"), "A B C");
        assert_eq!(song_decoder("AWUBBWUBC"), "A B C");
    }
}
