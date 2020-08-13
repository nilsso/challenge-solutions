fn main() {
    let input = "  a   b   ";

    for w in input.split_whitespace() {
        println!("'{}'", w);
    }
}
