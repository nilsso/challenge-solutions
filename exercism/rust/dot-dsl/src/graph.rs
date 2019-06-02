use maplit::hashmap;
use std::collections::HashMap;

use super::Attrs;
use macros::Attrs;

pub mod graph_items {
    pub mod node {
        pub use crate::node::Node;
    }
    pub mod edge {
        pub use crate::edge::Edge;
    }
}

use graph_items::edge::Edge;
use graph_items::node::Node;

#[derive(Attrs, Clone, PartialEq, Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            edges: vec![],
            attrs: hashmap! {},
        }
    }

    pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
        self.nodes.extend(nodes.iter().cloned());
        self
    }

    pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
        self.edges.extend(edges.iter().cloned());
        self
    }

    pub fn get_node(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|node| node.name.as_str() == name)
    }
}
