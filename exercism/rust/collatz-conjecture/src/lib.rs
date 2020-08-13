#![feature(bool_to_option)]

struct Collatz(u64);

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let Self(ref mut n) = self;
        (*n > 1).then_some({
            *n = if (*n) % 2 == 0 {
                (*n) / 2
            } else {
                (*n) * 3 + 1
            };
            *n
        })
    }
}

pub fn collatz(n: u64) -> Option<u64> {
    (n > 0).then_some(Collatz(n).into_iter().count() as u64)
}
