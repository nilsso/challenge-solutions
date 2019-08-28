pub fn check(candidate: &str) -> bool {
    let mut chars: [bool; 26] = Default::default();
    for c in candidate.chars().filter(|c| c.is_alphabetic()) {
        let i = c.to_ascii_lowercase() as usize - 'a' as usize;
        if chars[i] {
            return false;
        } else {
            chars[i] = true;
        }
    }
    true
}
