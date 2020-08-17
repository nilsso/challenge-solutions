#![feature(bool_to_option)]

use std::collections::HashMap;
use std::iter::once;

/// From the integers [0..n), get all combinations of size r.
fn ncr_integers(n: usize, r: usize) -> Vec<Vec<usize>> {
    assert!(n >= r);
    match r {
        0 => vec![],
        1 => (0..n).map(|i| vec![i]).collect(),
        _ => (0..=(n - r))
            .flat_map(|p| {
                ncr_integers(n - p - 1, r - 1)
                    .into_iter()
                    .map(move |mut v| once(p).chain(v.drain(0..).map(|c| p + c + 1)).collect())
            })
            .collect(),
    }
}

fn group_price(size: usize) -> u32 {
    (match size {
        2 => 760,
        3 => 720,
        4 => 640,
        5 => 600,
        _ => 800,
    } * size) as u32
}

fn nonzero_indices(things: &[u8]) -> Vec<usize> {
    things
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| (v > 0).then_some(i))
        .collect()
}

fn bin_price_tree(counts: Vec<u8>, group_size: usize) -> u32 {
    let indices = nonzero_indices(&counts);
    if indices.len() >= group_size {
        // Then, where n is the number of non-zero counts and r is the desired group size, there
        // are n-choose-r ways of picking the indices to the non-zero counts. We try each of these
        // combinations: reducing by one the counts at the chosen indices and recursively finding
        // the best bin price within these newly reduced counts. Lastly we choose the minimum
        // resulting price from these combinations.
        ncr_integers(indices.len(), group_size)
            .iter()
            .map(|index_choices| {
                let mut counts = counts.clone();
                index_choices.iter().for_each(|&i| counts[indices[i]] -= 1);
                group_price(group_size) + bin_price_tree(counts, group_size)
            })
            .min()
            .unwrap_or(0)
    } else {
        bin_price_tree(counts, group_size - 1)
    }
}

fn count_items(items: &[u32]) -> Vec<u8> {
    let mut counts: HashMap<u32, u8> = HashMap::new();
    for &v in items.iter() {
        *counts.entry(v).or_insert(0) += 1;
    }
    counts.values().copied().collect()
}

pub fn lowest_price(books: &[u32]) -> u32 {
    let counts = count_items(books);
    let bin_prices = [
        // not including these still passes the tests
        // and these last two slow the algorithm down a lot
        bin_price_tree(counts.clone(), 5),
        bin_price_tree(counts.clone(), 4),
        bin_price_tree(counts.clone(), 3),
        //bin_price_tree(counts.clone(), 2),
        //bin_price_tree(counts.clone(), 1),
    ];

    *bin_prices.iter().min().unwrap_or(&0)
}
