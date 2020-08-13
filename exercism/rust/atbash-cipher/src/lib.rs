#![feature(bool_to_option)]

use std::iter::{once, FromIterator};

fn atbash_transform(c: char) -> Option<char> {
    match c {
        '0'..='9' => Some(c),
        'A'..='Z' => Some((b'a' + b'Z' - c as u8) as char),
        'a'..='z' => Some((b'a' + b'z' - c as u8) as char),
        _ => None,
    }
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    String::from_iter(
        plain
            .chars()
            .filter_map(atbash_transform)
            .enumerate()
            .flat_map(|(i, c)| {
                (i > 0 && i % 5 == 0)
                    .then_some(' ')
                    .into_iter()
                    .chain(once(c))
            }),
    )
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    String::from_iter(cipher.chars().filter_map(atbash_transform))
}
