use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num < 1 {
        return None;
    }
    Some(
        match num.cmp(&(1..=num / 2).filter(|d| num % d == 0).sum()) {
            Ordering::Less => Classification::Abundant,
            Ordering::Equal => Classification::Perfect,
            Ordering::Greater => Classification::Deficient,
        },
    )
}
