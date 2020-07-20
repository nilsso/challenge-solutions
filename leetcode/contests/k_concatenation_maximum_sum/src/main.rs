pub fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let n = arr.len();
    let mut rep = vec![0; n * k];
    for (i, v) in arr.iter().enumerate() {
        for j in 0..k {
            rep[i + j * n] = *v;
        }
    }
    0.max((0..n*k).flat_map(|w| rep.windows(w+1)).map(|w| w.iter().sum()).max().unwrap_or(0))
}

fn main() {
    let arr = vec![-1, -2];
    let k = 7;

    println!("{}", k_concatenation_max_sum(arr, k));
}
