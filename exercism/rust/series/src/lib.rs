pub fn series(digits: &str, len: usize) -> Vec<String> {
    let n = digits.len();
    let n = if len <= n { n - len + 1 } else { 0 };
    (0..n).map(|i| digits[i..i + len].into()).collect()
}
