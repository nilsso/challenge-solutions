const RANK_FILE_MIN: i32 = 0;
const RANK_FILE_MAX: i32 = 7;

fn is_rank_file<T>(v: T) -> Option<T>
where
    T: Copy + Into<i32>,
{
    if RANK_FILE_MIN <= v.into() && v.into() <= RANK_FILE_MAX {
        Some(v)
    } else {
        None
    }
}

#[derive(Debug)]
pub struct ChessPosition(i32, i32);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        Some(Self(is_rank_file(rank)?, is_rank_file(file)?))
    }
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let (dr, df) = (
            ((self.0).0 - (other.0).0).abs(),
            ((self.0).1 - (other.0).1).abs(),
        );
        dr == 0 || df == 0 || df == dr
    }
}
