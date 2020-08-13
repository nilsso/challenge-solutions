#![feature(bool_to_option)]
#![allow(unused_variables, unused_imports, dead_code)]

use std::collections::HashSet;

use pythagorean_triplet::*;

fn main() {
    let n = 840;
    println!("{:?}", find(n));
}
