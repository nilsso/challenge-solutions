use clock::Clock;

fn main() {
    let c = Clock::new(1, -40);
    println!("{0:?} {0}", c);
    println!("{0:?} {0}", c.add_minutes(30));
}
