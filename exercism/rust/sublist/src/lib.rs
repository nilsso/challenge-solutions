#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

impl Comparison {
    pub fn invert(self) -> Comparison {
        match self {
            Comparison::Sublist => Comparison::Superlist,
            Comparison::Superlist => Comparison::Sublist,
            other => other,
        }
    }
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if a.len() == b.len() {
        if a == b {
            Comparison::Equal
        } else {
            Comparison::Unequal
        }
    } else if b.len() < a.len() {
        sublist(b, a).invert()
    } else if a.is_empty() || b.windows(a.len()).find(|w| &a == w).is_some() {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}
