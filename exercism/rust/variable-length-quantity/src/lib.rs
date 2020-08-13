#![feature(bool_to_option)]

#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::iter::{once, repeat};

#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    unimplemented!()
}

pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    const MASK: u8 = 0b0111_1111;
    const FIRST: u8 = 0b1000_0000;
    const LAST: u8 =  0b0111_0000;

    if let Some(byte) = bytes.last() {
        if byte & FIRST > 0 {
            return Err(Error::IncompleteNumber);
        }
    }

    let mut res = vec![];
    let mut n = 0;
    let mut i = 0;

    for b in bytes.into_iter().rev() {
        if b & FIRST == 0 {
            res.push(n);
            n = 0;
            i = 0;
        } else if i == 4 && b & LAST > 0 {
            return Err(Error::Overflow);
        }

        n |= (b & MASK) as u32;
        n <<= 7;
        i += 1;
    }

    Ok(res)
}
