use std::collections::HashMap;

pub fn max_number_of_balloons(text: String) -> i32 {
    let mut counts = HashMap::new();
    for c in text.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    let mut count = 0;
    loop {
        let b = counts.get(&'b');
        let a = counts.get(&'a');
        let l = counts.get(&'l');
        let o = counts.get(&'o');
        let n = counts.get(&'n');
        match (b, a, l, o, n) {
            (Some(b), Some(a), Some(l), Some(o), Some(n))
                if *b >= 1 && *a >= 1 && *l >= 2 && *o >= 2 && *n >= 1 =>
                {
                    *counts.get_mut(&'b').unwrap() -= 1;
                    *counts.get_mut(&'a').unwrap() -= 1;
                    *counts.get_mut(&'l').unwrap() -= 2;
                    *counts.get_mut(&'o').unwrap() -= 2;
                    *counts.get_mut(&'n').unwrap() -= 1;
                    count += 1;
                },
            _ => break,
        }
    }
    count
}

fn main() {
    println!("{}", max_number_of_balloons(String::from("balloonnollab")));
}
