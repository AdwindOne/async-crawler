pub fn matches_keywords(text: &str) -> Option<String> {
    let keywords = ["Rust", "async", "blockchain", "AI"];

    for kw in &keywords {
        if text.contains(kw) {
            return Some(kw.to_string());
        }
    }

    None
}