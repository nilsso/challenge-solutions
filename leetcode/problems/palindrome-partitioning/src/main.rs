fn is_palindromic(bytes: &[u8]) -> bool {
    let half = (bytes.len() + 1) / 2;
    (0..half)
        .zip((half..bytes.len()).rev())
        .all(|(l, r)| bytes[l] == bytes[r])
}

pub fn sized_palindromes(bytes: &[u8], n: usize) -> Vec<usize> {
    (0..bytes.len() - n + 1)
        .filter(|&i| is_palindromic(&bytes[i..i + n]))
        .collect()
}

pub fn partition(s: String) -> Vec<Vec<String>> {
    let mut partitionings = Vec::new();
    let bytes = s.as_bytes();
    if is_palindromic(bytes) {
        partitionings.push(vec![s.clone()]);
    }
    for size in (1..s.len()).rev() {
        if is_palindromic(&bytes[0..size]) {
            for mut subpartition in partition(s[size..].to_string()) {
                let mut partition = vec![s[0..size].to_string()];
                partition.append(&mut subpartition);
                partitionings.push(partition);
            }
        }
    }
    partitionings
}

fn main() {
    let s = "aabaa";
    for partition in partition(s.to_string()) {
        println!("{:?}", partition);
    }
}
