#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }

    let mut digits = vec![];
    for c in string_digits.chars() {
        if let Some(d) = c.to_digit(10) {
            digits.push(d as u64);
        } else {
            return Err(Error::InvalidDigit(c));
        }
    }

    if span > 0 {
        Ok(digits
            .windows(span)
            .map(|w| w.iter().fold(1, |acc, d| acc * d))
            .max()
            .unwrap())
    } else {
        Ok(1)
    }
}
