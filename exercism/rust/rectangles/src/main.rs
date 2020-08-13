#![allow(unused_imports)]

use rectangles::*;

use std::iter::repeat;

fn main() {
    let n = 5;

    let combs = (0..n - 1).flat_map(|i| repeat(i).zip(i + 1..n));

    for (i, j) in combs {
        println!("{} {}", i, j);
    }
}
