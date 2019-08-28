#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    pub moves: u8,
    pub goal_bucket: Bucket,
    pub other_bucket: u8,
}

pub fn solve(c1: u8, c2: u8, goal: u8, start_bucket: &Bucket) -> BucketStats {
    if start_bucket == &Bucket::Two {
        let bs = solve(c2, c1, goal, &Bucket::One);
        return BucketStats {
            goal_bucket: {
                if bs.goal_bucket == Bucket::One {
                    Bucket::Two
                } else {
                    Bucket::One
                }
            },
            ..bs
        };
    }

    let mut moves: u8 = 0;
    let mut b1: u8 = 0;
    let mut b2: u8 = 0;
    while b1 != goal && b2 != goal {
        match (b1, b2) {
            (0, _) => {
                // left empty
                b1 = c1; // fill left
            }
            (_, r) => {
                if c2 == goal {
                    b2 = c2;
                } else if r < c2 {
                    // right less than full
                    b2 += b1; // pour left into right
                    if b2 > c2 {
                        // if right overflowing
                        b1 = b2 - c2; // put remainder back into left
                        b2 = c2; // set right to capacity
                    } else {
                        b1 = 0; // left now empty
                    }
                } else {
                    // right is full
                    b2 = 0; // empty right
                }
            }
        }
        moves += 1;
    }
    BucketStats {
        moves,
        goal_bucket: {
            if b1 == goal {
                Bucket::One
            } else {
                Bucket::Two
            }
        },
        other_bucket: {
            if b1 == goal {
                b2
            } else {
                b1
            }
        },
    }
}
