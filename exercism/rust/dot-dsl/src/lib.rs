#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::cmp::PartialEq;

pub mod graph {
    pub mod graph_items {
        pub mod edge {

            #[derive(PartialEq, Debug)]
            pub struct Edge {}

            impl Edge {
                pub fn new() -> Self {
                    Self {}
                }
            }
        }
        pub mod node {
            #[derive(PartialEq, Debug)]
            pub struct Node {
                pub name: String,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_string(),
                    }
                }

                pub fn with_attrs(&mut self, attrs: ()) -> &Self {
                    self
                }
            }

            impl From<&Node> for Node {
                fn from(other: &Node) -> Self {
                    Self::new(other.name.as_str())
                }
            }
        }
    }

    use graph_items::edge::Edge;
    use graph_items::node::Node;

    #[derive(PartialEq, Debug)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: Vec<String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: vec![],
                edges: vec![],
                attrs: vec![],
            }
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            nodes
                .iter()
                .for_each(|node| self.nodes.push(Node::from(node)));
            self
        }
    }
}
