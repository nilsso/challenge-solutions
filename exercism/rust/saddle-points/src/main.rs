use saddle_points::find_saddle_points;

fn main() {
    let input = vec![vec![9, 8, 7], vec![5, 3, 2], vec![6, 6, 7]];
    println!("{:?}", find_saddle_points(&input));
}
