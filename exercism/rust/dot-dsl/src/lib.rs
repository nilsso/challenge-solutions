mod edge;
pub mod graph;
mod node;

pub trait Attrs {
    fn with_attrs(self, attrs: &[(&str, &str)]) -> Self;
    fn get_attr(&self, key: &str) -> Option<&str>;
}
