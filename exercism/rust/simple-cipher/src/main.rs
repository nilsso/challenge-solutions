use simple_cipher::*;

fn main() {
    let input = "aaaaa";

    println!("{}", input);
    println!("{:?}", encode_random(input));
}
