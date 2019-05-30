pub fn update_sieve(s: &mut Vec<bool>, n: usize) {
    s.append(&mut vec![true;n-s.len()]);
    s[0] = false;
    s[1] = false;
    s[2] = true;
    for i in 2..s.len()-1 {
        if s[i] {
            for j in (2*i..s.len()-1).step_by(i) {
                s[j] = false;
            }
        }
    }
}

pub fn nth(n: u32) -> u32 {
    let mut s: Vec<bool> = Vec::new();
    let mut len: usize = 100;
    loop {
        update_sieve(&mut s, len);
        let opt = s
            .iter()
            .filter(|p| **p)
            .nth(n as usize);
        match opt {
            Some((p, _)) => return p as u32,
            None => len *= 10
        }
    }
}

/*
 *pub fn nth(n: u32) -> u32 {
 *    let mut primes = vec![2];
 *    let mut cur = 3;
 *    let mut i = 0;
 *    while i < n {
 *        if primes.iter().all(|&p| cur % p != 0) {
 *            primes.push(cur);
 *            i += 1;
 *        }
 *        cur += 2;
 *    }
 *    primes.pop().unwrap()
 *}
 */
