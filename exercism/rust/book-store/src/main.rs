#![allow(unused_imports, unused_variables, dead_code)]

use std::iter::once;

//struct Combinations<'a> {
//items: &'a [u8],
//indices: Vec<usize>,
//}

//impl<'a> Iterator for Combinations<'a> {
//type Item = &'a [u8];

//fn next(&mut self) -> Option<Self::Item> {
//let Self { items, indices } = self;

//if !indices.iter().enumerate().all(|(i, &j)| i + 2 == j) {
//None
//} else {
//None
//}
//}
//}

//fn combinations(n: usize, r: usize) -> Vec<Vec<usize>> {
//let mut res = vec![];
//if r > 0 {
//for i in 0..=(n - r) {
//println!("{}", i);
//for mut v in combinations(n - 1, r - 1) {
//let mut m = vec![i];
//m.extend(v.drain(0..));
//res.push(m);
//}
//}
//}
//res
//}

fn ncr_indices(n: u32, r: u32) -> Vec<Vec<u32>> {
    assert!(n >= r);
    match r {
        0 => vec![],
        1 => (0..n).map(|i| vec![i]).collect(),
        _ => (0..=(n - r))
            .flat_map(|p| {
                ncr_indices(n - p - 1, r - 1)
                    .into_iter()
                    .map(move |mut v| once(p).chain(v.drain(0..).map(|c| p + c + 1)).collect())
            })
            .collect(),
    }
}

fn main() {
    for v in ncr_indices(5, 3) {
        println!("{:?}", v);
    }

    // 3 choose 3 [1, 2, 4]
    // 1 + 2 + 4

    // 4 choose 3 [1, 2, 4, 8]
    // 1 + 2 + 4 [3 c 2]
    // 1 + 2 + 8
    // 1 + 4 + 8
    // 2 + 4 + 8 [2 c 2]

    // 5 choose 3 [1, 2, 4, 8, 16]
    // 1 + 2 + 4, [4 c 2]
    // 1 + 2 + 8,
    // 1 + 2 + 16,
    // 1 + 4 + 8,
    // 1 + 4 + 16,
    // 1 + 8 + 16,
    // 2 + 4 + 8, [3 c 2]
    // 2 + 4 + 16,
    // 2 + 8 + 16,
    // 4 + 8 + 16, [2 c 2]
}
