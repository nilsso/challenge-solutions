#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub struct DigitGroup<'a> {
    i: std::ops::Range<usize>,
    lines: [&'a str; 4],
}

impl<'a> DigitGroup<'a> {
    pub fn new(lines: [&'a str; 4]) -> Result<Self, Error> {
        let invalid = lines.iter().map(|l| l.len()).find(|&n| n % 3 != 0);

        if let Some(n) = invalid {
            Err(Error::InvalidColumnCount(n))
        } else {
            let i = 0..lines[0].len() / 3;
            Ok(Self { i, lines })
        }
    }
}

impl<'a> Iterator for DigitGroup<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(i) = self.i.next() {
            let digit_str = |s: &'a str| &s[i * 3..i * 3 + 3];

            let lines = [
                digit_str(self.lines[0]),
                digit_str(self.lines[1]),
                digit_str(self.lines[2]),
                digit_str(self.lines[3]),
            ];

            let c = match lines {
                [" _ ", "| |", "|_|", "   "] => '0',
                ["   ", "  |", "  |", "   "] => '1',
                [" _ ", " _|", "|_ ", "   "] => '2',
                [" _ ", " _|", " _|", "   "] => '3',
                ["   ", "|_|", "  |", "   "] => '4',
                [" _ ", "|_ ", " _|", "   "] => '5',
                [" _ ", "|_ ", "|_|", "   "] => '6',
                [" _ ", "  |", "  |", "   "] => '7',
                [" _ ", "|_|", "|_|", "   "] => '8',
                [" _ ", "|_|", " _|", "   "] => '9',
                _ => '?',
            };

            Some(c)
        } else {
            None
        }
    }
}

pub fn convert(input: &str) -> Result<String, Error> {
    let mut lines = input.split('\n').collect::<Vec<_>>();
    let num_lines = lines.len();
    let num_digit_lines = num_lines / 4;

    if num_lines % 4 != 0 {
        return Err(Error::InvalidRowCount(num_lines));
    }

    let mut res: Vec<String> = vec![];

    for _ in 0..num_digit_lines {
        let d = lines.pop().unwrap();
        let c = lines.pop().unwrap();
        let b = lines.pop().unwrap();
        let a = lines.pop().unwrap();

        let digit_line = DigitGroup::new([a, b, c, d])?;

        res.push(digit_line.into_iter().collect());
    }

    res.reverse();
    Ok(res.join(","))
}
