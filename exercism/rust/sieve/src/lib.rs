pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let n = 1 + upper_bound as usize;
    let mut primes = vec![];
    let mut flags = vec![true; n];
    for i in 2..n {
        if flags[i] {
            primes.push(i as u64);
            flags.iter_mut().step_by(i).skip(2).for_each(|f| *f = false);
        }
    }
    primes
}
