use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};

use lazy_static::*;
use maplit::*;

const WHITELIST: &[&str] = &["What", "is", "by", "to", "the"];

lazy_static! {
    static ref OPERATIONS: HashMap<&'static str, fn(i32, i32) -> i32> = hashmap! {
        "plus" => Add::add as fn(i32, i32) -> i32,
        "minus" => Sub::sub,
        "multiplied" => Mul::mul,
        "divided" => Div::div,
        "raised" => |a: i32, b: i32| a.pow(b as u32),
    };
}

pub fn answer(command: &str) -> Option<i32> {
    let mut a_opt: Option<i32> = None;
    let mut f_opt: Option<fn(i32, i32) -> i32> = None;

    for word in command
        .trim_end_matches('?')
        .split_whitespace()
        .filter(|word| !WHITELIST.contains(word))
    {
        if let Ok(n) = word.parse() {
            match (a_opt.is_some(), f_opt.is_some()) {
                (false, false) => a_opt = Some(n),
                (true, true) => {
                    let a = a_opt.take().unwrap();
                    let f = f_opt.take().unwrap();
                    a_opt = Some(f(a, n))
                }
                _ => return None,
            }
        } else if let Some(&f) = OPERATIONS.get(&word) {
            if a_opt.is_none() || f_opt.replace(f).is_some() {
                return None;
            }
        } else {
            return None;
        }
    }

    if a_opt.is_some() && f_opt.is_none() {
        a_opt
    } else {
        None
    }
}
