pub struct RangeModule {
    ranges: Vec<(i32, i32)>,
}

impl RangeModule {
    pub fn new() -> Self {
        Self {
            ranges: Vec::new(),
        }
    }

    pub fn add_range(&mut self, left: i32, right: i32) {
        let ranges = &mut self.ranges;
        if let Some(&first) = ranges.get(0) {
            if right < first.0 {
                ranges.insert(0, (left, right));
            } else {
                if let Some(i) = ranges
                    .iter()
                    .enumerate()
                    .find(|(i, r)| r.0 <= left)
                    .map(|(i, _)| i)
                {
                } else {
                }
            }
        } else {
            // empty
            ranges.push((left, right));
        }
    }

    pub fn query_range(&self, left: i32, right: i32) -> bool {
        for (l, r) in self.ranges.iter() {
            if l <= &left && &left < r {
                return &right <= r;
            }
        }
        false
    }

    pub fn remove_range(&self, left: i32, right: i32) {
    }
}

fn main() {
    let mut v = vec![0,1,2,3,4,5];
    println!("{:?}", v);
    v.drain(2..2);
    println!("{:?}", v);
}
