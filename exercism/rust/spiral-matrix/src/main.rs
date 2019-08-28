use spiral_matrix::*;
use std::env;

fn main() {
    for i in std::iter::once(4) {
        println!("{:?}", i);
    }
    //let dim = env::args().skip(1).next().unwrap().parse().unwrap();
    //for row in spiral_matrix(dim) {
        //println!("{:?}", row);
    //}
}
