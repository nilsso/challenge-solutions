#![feature(bool_to_option)]

use std::iter::repeat;

fn cartesian_product(m: usize, n: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..m).flat_map(move |i| repeat(i).zip(0..n))
}

fn combinations(n: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..n.max(1) - 1).flat_map(move |i| repeat(i).zip(i + 1..n))
}

pub fn count(lines: &[&str]) -> u32 {
    let rows = lines.len();
    let cols = lines.get(0).map_or(0, |l| l.len());
    let chars: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let is_vertex = |&(i, j): &(usize, usize)| chars[i][j] == '+';
    let vertices: Vec<(usize, usize)> = cartesian_product(rows, cols).filter(is_vertex).collect();

    let is_rectangle = |&[t, l, b, r]: &[usize; 4]| {
        // Helpers to check that a char belongs on a rectangle side
        let is_hline_char = |(i, j): (usize, usize)| {
            let c = chars[i][j];
            c == '+' || c == '-'
        };
        let is_vline_char = |(i, j): (usize, usize)| {
            let c = chars[i][j];
            c == '+' || c == '|'
        };

        // Helpers to check rectangle sides for gaps
        let is_hline = |row| repeat(row).zip(l..r).all(is_hline_char);
        let is_vline = |col| (t..b).zip(repeat(col)).all(is_vline_char);

        is_hline(t) && is_hline(b) && is_vline(l) && is_vline(r)
    };

    combinations(vertices.len())
        .filter_map(|(i, j)| {
            let (t, l) = vertices[i];
            let (b, r) = vertices[j];
            // If two selected vertices could be top-left and bottom-right corners of a rectangle,
            // and if bottom-left and top-right are also vertices.
            (l < r && t < b && is_vertex(&(t, r)) && is_vertex(&(b, l))).then_some([t, l, b, r])
        })
        .filter(is_rectangle)
        .count() as u32
}
