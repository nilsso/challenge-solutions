pub struct PascalsTriangle {
    rows: usize,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self {
            rows: row_count as usize,
        }
    }

    fn new_row(i: usize, prev: Option<&Vec<u32>>) -> Vec<u32> {
        let mut cur = Vec::with_capacity(i + 1);
        cur.push(1);
        if let Some(prev) = prev {
            for pair in prev.windows(2) {
                cur.push(pair.iter().sum());
            }
            cur.push(1);
        }
        return cur;
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows = Vec::with_capacity(self.rows);
        for i in 0..self.rows {
            rows.push(Self::new_row(i, rows.last()));
        }
        rows
    }
}
