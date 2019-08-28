#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        Err(Error::InvalidInputBase)
    } else if let Some(n)  = number.iter().find(|digit| *digit >= &from_base) {
        Err(Error::InvalidDigit(*n))
    } else if to_base < 2 {
        Err(Error::InvalidOutputBase)
    } else {
        let mut val = number
            .iter()
            .fold(0, |acc, n| acc * from_base + n);
        let mut out_digits: Vec<u32> = Vec::new();
        while val > 0 {
            out_digits.push(val % to_base);
            val /= to_base;
        }
        out_digits.reverse();
        Ok(out_digits)
    }
}
