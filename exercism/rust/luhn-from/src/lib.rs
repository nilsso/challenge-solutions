fn all_numeric(bytes: &[u8]) -> bool {
    bytes.iter().all(|&b| b'0' <= b && b <= b'9')
}

fn luhn_check(mut bytes: Vec<u8>) -> bool {
    bytes.iter_mut().for_each(|b| *b -= b'0');

    for b in bytes.iter_mut().rev().skip(1).step_by(2) {
        *b = match b {
            0..=4 => 2 * (*b),
            5..=9 => 2 * (*b) - 9,
            _ => unreachable!(),
        };
    }

    bytes.iter().sum::<u8>() % 10 == 0
}

pub fn is_valid(code: &str) -> bool {
    let bytes: Vec<u8> = code
        .split_whitespace()
        .flat_map(|chunk| chunk.as_bytes().iter())
        .copied()
        .collect();

    bytes.len() > 1 && all_numeric(&bytes) && luhn_check(bytes)
}

pub struct Luhn(bool);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.0
    }
}

impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Self(is_valid(&input.to_string()))
    }
}
