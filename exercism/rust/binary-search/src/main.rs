use binary_search::find;
use std::env;

fn main() {
    let mut args: Vec<i32> = env::args().skip(1).map(|n| n.parse().unwrap()).collect();
    let key = args.pop().unwrap();
    println!("{:?}", find(&args, key));
}
