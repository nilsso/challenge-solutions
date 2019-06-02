use maplit::hashmap;
use std::collections::HashMap;

use super::Attrs;
use macros::Attrs;

#[derive(Attrs, Clone, PartialEq, Debug)]
pub struct Edge {
    pub a: String,
    pub b: String,
    pub attrs: HashMap<String, String>,
}

impl Edge {
    pub fn new(a: &str, b: &str) -> Self {
        Self {
            a: a.to_string(),
            b: b.to_string(),
            attrs: hashmap! {},
        }
    }
}
