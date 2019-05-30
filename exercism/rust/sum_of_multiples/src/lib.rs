pub fn som_naive(limit: u32, factors: &[u32]) -> u32 {
    let factors: Vec<u32> = factors
        .iter()
        .filter(|f| **f > 0)
        .cloned()
        .collect();
    (1..limit)
        .filter(|i| factors
                .iter()
                .any(|f| i % f == 0))
        .sum()
}

pub use som_naive as sum_of_multiples;
