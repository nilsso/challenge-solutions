use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2, p)
}

pub fn mod_pow(b: u64, mut e: u64, m: u64) -> u64 {
    let mut res = 1;
    let mut b = b % m;
    while e > 0 {
        if e & 1 == 1 {
            res = (res * b) % m;
        }
        e >>= 1;
        b = (b * b) % m;
    }
    res
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_pow(g, a, p)
}

pub fn secret(p: u64, pub_key: u64, a: u64) -> u64 {
    mod_pow(pub_key, a, p)
}
