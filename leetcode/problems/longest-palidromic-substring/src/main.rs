//mod brute_force;
mod center_expand;

//use brute_force::brute_force;
use center_expand::center_expand;

fn main() {
    let s = "ababbaba";
    println!("{}", center_expand(s.to_string()));
}
