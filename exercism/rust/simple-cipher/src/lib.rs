use std::iter::FromIterator;

const RANDOM_KEY_LEN: usize = 100;

fn is_not_lowercase_alphabetic(c: char) -> bool {
    !(c.is_lowercase() && c.is_alphabetic())
}

macro_rules! shift_cipher {
    ($src:ident, $key:ident, $op:tt) => {
        if $key.len() == 0 || $key.chars().any(is_not_lowercase_alphabetic) {
            None
        } else {
            let t = |(s, k)| {
                let s = (s - b'a') as i8;
                let k = (k - b'a') as i8;
                let x = (s $op k).rem_euclid(26) as u8 + b'a';

                x as char
            };

            let s = $src.as_bytes().iter();
            let k = $key.as_bytes().iter().cycle();
            Some(String::from_iter(s.zip(k).map(t)))
        }
    };
}

pub fn encode(key: &str, src: &str) -> Option<String> {
    shift_cipher!(src, key, +)
}

pub fn decode(key: &str, src: &str) -> Option<String> {
    shift_cipher!(src, key, -)
}

pub fn encode_random(src: &str) -> (String, String) {
    use rand::Rng;

    let mut rng = rand::thread_rng();

    let i = (0..RANDOM_KEY_LEN).map(|_| rng.gen_range(b'a', b'z') as char);
    let key = String::from_iter(i);
    let encoded = shift_cipher!(src, key, +).unwrap();

    (key, encoded)
}
