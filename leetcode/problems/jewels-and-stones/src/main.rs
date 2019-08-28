use std::{
    collections::HashSet,
    iter::FromIterator,
};

pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
    let j: HashSet<char> = HashSet::from_iter(j.chars());
    s.chars().filter(|c| j.contains(c)).count() as i32
}

fn test(j: &str, s: &str, answer: i32) {
    assert_eq!(num_jewels_in_stones(j.to_string(), s.to_string()), answer);
}

#[test]
fn test_1() { test("aA", "aaAbbbb", 3); }
