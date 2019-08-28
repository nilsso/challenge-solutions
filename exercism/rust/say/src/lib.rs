fn encode_period(n: u64) -> String {
    assert!(n < 100);
    let s = match n {
        1 => Some("one"),
        2 => Some("two"),
        3 => Some("three"),
        4 => Some("four"),
        5 => Some("five"),
        6 => Some("six"),
        7 => Some("seven"),
        8 => Some("eight"),
        9 => Some("nine"),
        10 => Some("ten"),
        11 => Some("eleven"),
        12 => Some("twelve"),
        13 => Some("thirteen"),
        14 => Some("fourteen"),
        15 => Some("fifteen"),
        16 => Some("sixteen"),
        17 => Some("seventeen"),
        18 => Some("eighteen"),
        19 => Some("nineteen"),
        20 => Some("twenty"),
        30 => Some("thirty"),
        40 => Some("forty"),
        50 => Some("fifty"),
        60 => Some("sixty"),
        70 => Some("seventy"),
        80 => Some("eighty"),
        90 => Some("ninety"),
        _ => None,
    };
    if let Some(s) = s {
        s.to_string()
    } else {
        format!("{}-{}", encode_period(n / 10 * 10), encode_period(n % 10))
    }
}

struct Period {
    digits: u64,
    period: usize,
}

impl ToString for Period {
    fn to_string(&self) -> String {
        let (q, r) = (self.digits / 100, self.digits % 100);
        let p = match self.period {
            0 => "",
            1 => " thousand",
            2 => " million",
            3 => " billion",
            4 => " trillion",
            5 => " quadrillion",
            6 => " quintillion",
            _ => unreachable!(),
        };
        if q > 0 {
            if r > 0 {
                format!("{} hundred {}{}", encode_period(q), encode_period(r), p)
            } else {
                format!("{} hundred{}", encode_period(q), p)
            }
        } else {
            format!("{}{}", encode_period(r), p)
        }
    }
}

pub fn encode(mut n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }
    let mut periods: Vec<Period> = Vec::new();
    let mut p: usize = 0;
    while n > 0 {
        let d = n % 1000;
        if d != 0 {
            periods.push(Period {
                digits: d,
                period: p,
            });
        }
        n /= 1000;
        p += 1;
    }
    periods
        .iter()
        .rev()
        .map(|p| p.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}
