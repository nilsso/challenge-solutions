use lazy_static::lazy_static;
use regex::Regex;

pub fn encode(source: &str) -> String {
    if source == "" {
        return "".to_string();
    }
    let mut v: Vec<(usize, char)> = Vec::new();
    let mut p: (usize, char) = (1, source.chars().next().unwrap());
    for c in source.chars().skip(1) {
        if c == p.1 {
            p.0 += 1;
        } else {
            v.push(p.clone());
            p.0 = 1;
            p.1 = c;
        }
    }
    v.push(p.clone());
    v.iter()
        .map(|p| {
            if p.0 > 1 {
                format!("{}{}", p.0, p.1)
            } else {
                format!("{}", p.1)
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

pub fn decode(source: &str) -> String {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new("([0-9]*)([ a-zA-Z])").unwrap();
    }
    let mut s = String::new();
    for caps in PATTERN.captures_iter(source) {
        for _ in 0..(caps[1]).parse().unwrap_or(1) {
            s.push_str(&caps[2]);
        }
    }
    s.to_string()
}
