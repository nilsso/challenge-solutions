use scale_generator::*;

fn main() {
    let res = Scale::new("C", "MMmMMMm").unwrap();

    for p in res.enumerate() {
        println!("{}", p);
    }
}
