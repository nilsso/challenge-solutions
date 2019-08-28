pub fn center_expand(s: String) -> String {
    fn helper(s: &[u8], l: usize, r: usize) -> usize {
        let (mut l, mut r) = (l as isize, r as isize);
        while l >= 0 && r < s.len() as isize && s[l as usize] == s[r as usize] {
            l -= 1;
            r += 1;
        }
        (r - l - 1) as usize
    }

    if s.len() == 0 {
        s
    } else {
        let bytes = s.as_bytes();
        let (mut start, mut end) = (0, 0);
        for i in 0..s.len() - 1 {
            let l1 = helper(&bytes, i, i);
            let l2 = helper(&bytes, i, i + 1);
            let len = l1.max(l2);
            if len > end - start {
                start = i - (len - 1) / 2;
                end = i + len / 2;
            }
        }
        s[start..end + 1].to_string()
    }
}
