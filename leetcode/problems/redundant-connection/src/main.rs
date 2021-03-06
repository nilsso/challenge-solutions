use std::collections::HashMap;

#[derive(Debug)]
struct DSU {
    parents: HashMap<usize, usize>,
    ranks: HashMap<usize, usize>,
}

impl DSU {
    fn new() -> Self {
        DSU {
            parents: HashMap::new(),
            ranks: HashMap::new(),
        }
    }

    fn parent(&mut self, x: usize) -> &mut usize {
        self.parents.entry(x).or_insert(x)
    }

    fn rank(&mut self, x: usize) -> &mut usize {
        self.ranks.entry(x).or_insert(0)
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent(x) != &x {
            let p = *self.parent(x);
            *self.parent(x) = self.find(p);
        }
        *self.parent(x)
    }

    fn union(&mut self, edge: &Vec<i32>) -> bool {
        let (x, y) = (edge[0] as usize, edge[1] as usize);
        let (x_rank, y_rank) = (self.find(x), self.find(y));
        if x_rank == y_rank {
            false
        } else {
            use std::cmp::Ordering::*;
            let (x_parent, y_parent) = (*self.parent(x_rank), *self.parent(y_rank));
            match x_parent.cmp(&y_parent) {
                Less    => *self.parent(x_rank) = y_rank,
                Greater => *self.parent(y_rank) = x_rank,
                Equal   => {
                    *self.parent(y_rank) = x_rank;
                    *self.rank(x_rank) += 1;
                },
            }
            true
        }
    }
}

fn redundant_edge(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut dsu = DSU::new();
    edges.into_iter().find(|edge| !dsu.union(edge)).unwrap()
}

fn to_edges(edges: &[(i32, i32)]) -> Vec<Vec<i32>> {
    edges.iter().map(|t| vec![t.0, t.1]).collect()
}

fn main() {
    println!("{:?}", DSU::new());
}

fn test(edges: &[(i32, i32)], expected: Vec<i32>) {
    assert_eq!(redundant_edge(to_edges(edges)), expected);
}

#[test]
fn test_1() {
    test(&[(1, 2), (1, 3), (2, 3)], vec![2, 3]);
}
