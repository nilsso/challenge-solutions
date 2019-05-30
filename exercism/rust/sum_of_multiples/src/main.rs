use sum_of_multiples::sum_of_multiples;
//use sum_of_multiples::NumericalSemigroup;

fn main() {
    //let gens = vec![6, 9, 20];
    //let nsg = NumericalSemigroup::new(gens);
    //println!("{:?}", nsg.sparse.iter().sum::<u32>());
    println!("{}", sum_of_multiples(7, &[3]))
}
