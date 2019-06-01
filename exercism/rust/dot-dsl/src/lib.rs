#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::collections::HashMap;

pub trait WithAttrs: Sized {
    fn attrs(&mut self) -> &mut HashMap<String, String>;

    fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        for (k, v) in attrs {
            self.attrs().insert(k.to_string(), v.to_string());
        }
        self
    }
}

pub mod graph {
    use maplit::hashmap;
    use std::collections::HashMap;

    pub mod graph_items {
        pub mod node {
            use maplit::hashmap;
            use std::collections::HashMap;

            use crate::WithAttrs;

            #[derive(PartialEq, Debug)]
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

            impl From<&Node> for Node {
                fn from(other: &Node) -> Self {
                    Self {
                        name: other.name.clone(),
                        attrs: other.attrs.clone(),
                    }
                }
            }

//            pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
//                for (k, v) in attrs {
//                    self.attrs.insert(k.to_string(), v.to_string());
//                }
//                self
//            }

            impl WithAttrs for Node {
                fn attrs(&mut self) -> &mut HashMap<String, String> {
                    &mut self.attrs
                }
            }
        }

        pub mod edge {
            #[derive(PartialEq, Debug)]
            pub struct Edge {
                a: String,
                b: String,
            }

            impl Edge {
                pub fn new(a: &str, b: &str) -> Self {
                    Self {
                        a: a.to_string(),
                        b: b.to_string(),
                    }
                }
            }

            impl From<&Edge> for Edge {
                fn from(other: &Edge) -> Self {
                    Self {
                        a: other.a.clone(),
                        b: other.b.clone(),
                    }
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
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: vec![],
                edges: vec![],
                attrs: hashmap!{},
            }
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (k, v) in attrs {
                self.attrs.insert(k.to_string(), v.to_string());
            }
            self
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            for node in nodes {
                self.nodes.push(Node::from(node));
            }
            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            for edge in edges {
                self.edges.push(Edge::from(edge));
            }
            self
        }
    }
}
