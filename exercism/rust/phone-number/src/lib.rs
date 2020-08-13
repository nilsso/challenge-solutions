fn number_regular(cleaned: &[char]) -> bool {
    ('2'..='9').contains(&cleaned[0]) && ('2'..='9').contains(&cleaned[3])
}

fn number_country(cleaned: &[char]) -> bool {
    cleaned[0] == '1' && number_regular(&cleaned[1..])
}

pub fn number(number: &str) -> Option<String> {
    let cleaned: Vec<char> = number.chars().filter(|c| c.is_numeric()).collect();

    match cleaned.len() {
        10 if number_regular(&cleaned) => Some(cleaned.iter().collect()),
        11 if number_country(&cleaned) => Some(cleaned[1..].iter().collect()),
        _ => None,
    }
}
