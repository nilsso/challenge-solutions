use std::cmp::{Ord, Ordering::*};

pub fn find<A: AsRef<[K]>, K: Ord>(array: A, key: K) -> Option<usize> {
    let array = array.as_ref();
    let i: usize = array.len() / 2;
    match key.cmp(array.get(i)?) {
        Greater => match find(&array[i + 1..], key) {
            Some(j) => Some(i + j + 1),
            None => None,
        },
        Less => find(&array[..i], key),
        Equal => Some(i),
    }
}
