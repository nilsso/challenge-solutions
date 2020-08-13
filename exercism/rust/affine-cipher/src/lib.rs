#![feature(bool_to_option)]
#![allow(non_upper_case_globals, non_snake_case)]

use std::iter::once;
use std::iter::FromIterator;

#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Alphabet size
const m: i32 = 26;

pub fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn inverse_mod_n(a: i32, n: i32) -> Result<i32, AffineCipherError> {
    if gcd(a, n) != 1 {
        Err(AffineCipherError::NotCoprime(a))
    } else {
        for x in 1..n {
            if a * x % n == 1 {
                return Ok(x);
            }
        }
        unreachable!() // ;^)
    }
}

fn transformer(t: impl Fn(i32) -> i32) -> impl Fn(char) -> Option<char> {
    move |c| match c {
        '0'..='9' => Some(c),
        'A'..='Z' => Some((t(c as i32 - b'A' as i32) as u8 + b'a') as char),
        'a'..='z' => Some((t(c as i32 - b'a' as i32) as u8 + b'a') as char),
        _ => None,
    }
}

fn encoder(a: i32, b: i32) -> Result<impl Fn(char) -> Option<char>, AffineCipherError> {
    inverse_mod_n(a, m)?;

    let t = move |x: i32| (a * x + b) % m;
    Ok(transformer(t))
}

fn decoder(a: i32, b: i32) -> Result<impl Fn(char) -> Option<char>, AffineCipherError> {
    let a_inv = inverse_mod_n(a, m)?;

    let t = move |y: i32| (a_inv * (y - b)).rem_euclid(m);
    Ok(transformer(t))
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`).
pub fn encode(plain: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let Ex = encoder(a, b)?;

    Ok(String::from_iter(
        plain.chars().filter_map(Ex).enumerate().flat_map(|(i, c)| {
            (i > 0 && i % 5 == 0)
                .then_some(' ')
                .into_iter()
                .chain(once(c))
        }),
    ))
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`).
pub fn decode(cipher: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let Dy = decoder(a, b)?;

    Ok(String::from_iter(cipher.chars().filter_map(Dy)))
}
