use std::collections::HashMap;

trait WordCount {
    fn is_word_delim(self) -> bool;
    fn isnt_alphanumeric(self) -> bool;
}

#[rustfmt::skip]
impl WordCount for char {
    fn is_word_delim(self) -> bool { self.is_whitespace() || self == ',' }
    fn isnt_alphanumeric(self) -> bool { !self.is_alphanumeric() }
}

pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();

    for word in words
        .split(<char>::is_word_delim)
        .filter(|w| w.find(<char>::is_alphanumeric).is_some())
        .map(|w| w.trim_matches(<char>::isnt_alphanumeric))
    {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }

    counts
}
