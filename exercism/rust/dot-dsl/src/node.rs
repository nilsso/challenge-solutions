use maplit::hashmap;
use std::collections::HashMap;

use super::Attrs;
use macros::Attrs;

#[derive(Attrs, Clone, PartialEq, Debug)]
pub struct Node {
    pub name: String,
    pub attrs: HashMap<String, String>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            attrs: hashmap! {},
        }
    }
}
