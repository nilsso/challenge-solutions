struct Solution;

use std::collections::HashMap;

fn check_word(word: &String, mut alph: HashMap<char, usize>) -> bool {
    for c in word.chars() {
        if let Some(count) = alph.get_mut(&c) {
            if count == &0 {
                return false;
            } else {
                *count -= 1;
            }
        } else {
            return false;
        }
    }
    true
}

impl Solution {
    pub fn count_characters(words: Vec<String>, alph: String) -> i32 {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for c in alph.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
        words
            .iter()
            .filter(|word| check_word(word, counts.clone()))
            .map(|word| word.len())
            .sum::<usize>() as i32
    }
}

fn main() {
    let words: Vec<String> = vec![
        "hello".to_string(),
        "world".to_string(),
        "leetcode".to_string(),
    ];
    let chars: String = "welldonehoneyr".to_string();

    println!("{}", Solution::count_characters(words, chars));
}
