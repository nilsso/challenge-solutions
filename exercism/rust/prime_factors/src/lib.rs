pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut d = 2;
    while n > 1 {
        match n % d {
            0 => {
                factors.push(d);
                n /= d;
            },
            _ => d += 1
        }
    }
    factors
}
