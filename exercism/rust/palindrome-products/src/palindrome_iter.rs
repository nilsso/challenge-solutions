pub struct PalindromeIter {
    cur: Vec<u8>,
}

impl PalindromeIter {
    pub fn new(min: u64) -> Self {
        let mut cur = min.to_string().into_bytes();
        let n = cur.len();
        for i in 0..(n + 1) / 2 {
            cur[i] -= 48;
            cur[n - i - 1] = cur[i];
        }
        Self { cur }
    }
}

impl Iterator for PalindromeIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self
            .cur
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, &d)| acc + 10_u64.pow(i as u32) * d as u64);

        let n = self.cur.len();
        let mut carry: u8 = 0;
        for i in (0..(n + 1) / 2).rev() {
            self.cur[i] += 1;
            carry = self.cur[i] / 10;
            self.cur[i] = self.cur[i] % 10;
            self.cur[n - i - 1] = self.cur[i];
            if carry == 0 {
                break;
            }
        }
        if carry != 0 {
            self.cur.push(1);
            self.cur[0] = 1;
            for i in 1..n {
                self.cur[i] = 0;
            }
        }
        Some(next)
    }
}
