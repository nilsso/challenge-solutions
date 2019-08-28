use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    let mut letters: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    for c in sentence.to_lowercase().chars() {
        letters.remove(&c);
        if letters.len() == 0 {
            return true;
        }
    }
    false
}
