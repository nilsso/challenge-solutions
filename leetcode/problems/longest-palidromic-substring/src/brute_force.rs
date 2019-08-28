pub fn brute_force(s: String) -> String {
    fn palindromic(s: &[u8]) -> bool {
        let n = s.len();
        let h = (n + 1) / 2;
        let mut i_iter = 0..h;
        let mut j_iter = (h..n).rev();
        for (i, j) in i_iter.zip(j_iter) {
            if s[i] != s[j] {
                return false;
            }
        }
        true
    }
    let n = s.len();
    if n > 0 {
        let bytes = s.as_bytes();
        for len in (1..n + 1).rev() {
            for start in 0..(n - len + 1) {
                let end = start + len;
                if palindromic(&bytes[start..end]) {
                    return s[start..end].to_string();
                }
            }
        }
        unreachable!();
    } else {
        s
    }
}
