#![feature(bool_to_option)]

use itertools::iproduct;
use std::iter::{once, repeat};

pub fn encrypt(input: &str) -> String {
    let iter = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(char::to_lowercase);

    let l = iter.clone().count();
    let (w, h) = {
        let n = (l as f32).sqrt();
        (n.ceil() as usize, n.floor() as usize)
    };

    let chars: Vec<char> = iter.chain(repeat(' ').take(w * h - l)).collect();

    iproduct!(0..w, 0..h)
        .flat_map(|(x, y)| {
            let c = chars[x + y * w] as char;
            let add_extra = (y == h - 1) && (x < w - 1);
            once(c).chain(add_extra.then_some(' '))
        })
        .collect()
}
