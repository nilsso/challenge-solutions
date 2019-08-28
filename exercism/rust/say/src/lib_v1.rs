pub fn encode(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }
    n
        .to_string()
        .as_bytes()
        .rchunks(3)
        .map(|c| encode_chunk(c))
        .enumerate()
        .map(|(i, mut e)| {
            if e != "" {
                match i {
                    0 => {},
                    1 => e.push_str(" thousand"),
                    2 => e.push_str(" million"),
                    3 => e.push_str(" billion"),
                    4 => e.push_str(" trillion"),
                    5 => e.push_str(" quadrillion"),
                    6 => e.push_str(" quintillion"),
                    _ => unreachable!(),
                }
            }
            e
        })
        .filter(|c| c != "")
        .rev()
        .collect::<Vec<String>>()
        .join(" ")
        .to_string()
}

fn encode_chunk(chunk: &[u8]) -> String {
    let len = chunk.len();
    let mut encoded = String::new();
    if len == 3 {
        let s = &encode_two_digit(None, Some(chunk[0] - 48));
        if s != "" {
            encoded.push_str(s);
            encoded.push_str(" hundred");
        }
    }
    if len >= 2 {
        let s = encode_two_digit(Some(chunk[len - 2] - 48), Some(chunk[len - 1] - 48));
        if s != "" {
            if len == 3 && chunk[0] - 48 != 0 {
                encoded.push(' ');
            }
            encoded.push_str(&s);
        }
    } else {
        encoded.push_str(&encode_two_digit(None, Some(chunk[len - 1] - 48)));
    }
    encoded
}

fn encode_two_digit(a: Option<u8>, b: Option<u8>) -> String {
    let s = match (a, b) {
        (None, Some(b)) => {
            match b {
                1 => "one",
                2 => "two",
                3 => "three",
                4 => "four",
                5 => "five",
                6 => "six",
                7 => "seven",
                8 => "eight",
                9 => "nine",
                _ => "",
            }
        },
        (Some(a), Some(b)) => {
            match (a, b) {
                (1, 0) => "ten",
                (1, 1) => "eleven",
                (1, 2) => "twelve",
                (1, 3) => "thirteen",
                (1, 4) => "fourteen",
                (1, 5) => "fifteen",
                (1, 6) => "sixteen",
                (1, 7) => "seventeen",
                (1, 8) => "eighteen",
                (1, 9) => "nineteen",
                (2, 0) => "twenty",
                (3, 0) => "thirty",
                (4, 0) => "forty",
                (5, 0) => "fifty",
                (6, 0) => "sixty",
                (7, 0) => "seventy",
                (8, 0) => "eighty",
                (9, 0) => "ninety",
                (0, b) => {
                    return encode_two_digit(None, Some(b))
                }
                (a, b) => {
                    let mut s = String::new();
                    s.push_str(&encode_two_digit(Some(a), Some(0)));
                    s.push('-');
                    s.push_str(&encode_two_digit(None, Some(b)));
                    return s
                }
            }
        }
        _ => unreachable!(),
    };
    s.to_string()
}
