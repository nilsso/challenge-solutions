use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let re = Regex::new(r"[[:alpha:]]+").unwrap();
    let terms: Vec<&str> = re
        .captures_iter(input)
        .map(|c| c.get(0).unwrap().as_str())
        .collect();
    let m = terms.iter().max_by_key(|t| t.len()).unwrap().len();
    let cols: Vec<String> = {
        let mut cols = Vec::new();
        for i in 0..m {
            cols.push(terms.iter().filter_map(|t| t.chars().rev().nth(i)).collect());
        }
        cols
    };
    let chars: HashSet<char> = terms.iter().flat_map(|s| s.chars()).collect();
    let mut map: HashMap<char, u8> = chars.iter().map(|&c| (c, 0)).collect();

    let mut carry: u64;
    for mut combination in (0..10_u8).combinations(chars.len()) {
        for permutation in permutohedron::Heap::new(&mut combination) {
            map.values_mut()
                .zip(permutation.into_iter())
                .for_each(|(v, i)| *v = i);
            if terms.iter().any(|t| *map.get(&t.chars().next().unwrap()).unwrap() == 0) {
                continue;
            }
            carry = 0;
            for col in cols.iter() {
                let mut i = col.chars().rev();
                let r = *map.get(&i.next().unwrap()).unwrap() as u64;
                let l = i.fold(carry as u64, |acc, c| acc + *map.get(&c).unwrap() as u64);
                carry = l / 10;
                if l % 10 != r {
                    continue;
                }
            }
            return Some(map);
        }
    }
    None
}
