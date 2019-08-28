fn push_word(word: &str, abbr: &mut String) {
    for word in word.split('-') {
        let mut chars = word.chars();
        abbr.push(chars.next().unwrap().to_uppercase().next().unwrap());
        if !word.chars().all(|c| c.is_uppercase()) {
            chars.filter(|c| c.is_uppercase()).for_each(|c| abbr.push(c));
        }
    }
}

pub fn abbreviate(phrase: &str) -> String {
    let mut abbr = String::new();
    let mut words = phrase.trim().split_whitespace();
    if let Some(first) = words.next() {
        if !first.chars().last().unwrap().is_alphabetic() {
            first.chars().filter(|c| c.is_uppercase()).for_each(|c| abbr.push(c));
        } else {
            push_word(first, &mut abbr);
            for word in words {
                push_word(word, &mut abbr);
            }
        }
    }
    abbr
}
