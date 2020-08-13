#![feature(bool_to_option)]

use itertools::iproduct;

fn shift(v: usize, lim: usize) -> std::ops::RangeInclusive<isize> {
    match (v > 0, v < lim - 1) {
        (true, true) => -1..=1,
        (true, false) => -1..=0,
        (false, true) => 0..=1,
        _ => 0..=0,
    }
}

fn increment(b: char) -> char {
    match b {
        '*' => '*',
        ' ' => '1',
        _ => (b as u8 + 1) as char,
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut lines: Vec<Vec<char>> = minefield
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let rows = minefield.len();
    let cols = if rows > 0 { lines[0].len() } else { 0 };

    let adjacents = |r, c| {
        let r_shifts = shift(r, rows);
        let c_shifts = shift(c, cols);
        iproduct!(r_shifts, c_shifts)
            .map(move |(u, v)| (u + r as isize, v + c as isize))
            .map(move |(r, c)| (r as usize, c as usize))
    };

    for (r, c) in iproduct!(0..rows, 0..cols) {
        if lines[r][c] == '*' {
            for (r, c) in adjacents(r, c) {
                lines[r][c] = increment(lines[r][c]);
            }
        }
    }

    lines.iter().map(|chars| chars.iter().collect()).collect()
}
