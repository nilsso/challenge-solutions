#![allow(unused_imports)]

use dot_dsl::graph::graph_items::edge::Edge;
use dot_dsl::graph::graph_items::node::Node;
use dot_dsl::graph::Graph;
use maplit::hashmap;

use dot_dsl::WithAttrs;

fn main() {
    println!("{:?}", Node::new("a").with_attrs(&[("color", "green")]));
//    "a b".split_whitespace().fold(String::new(), |s, w| s.concat(w));
//    let nodes = vec![Node::new("a")];
//    let graph = Graph::new().with_nodes(&nodes);
}
