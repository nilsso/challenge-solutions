use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

#[derive(PartialEq, PartialOrd, Eq, Ord)]
struct CharCoord {
    coord: (isize, isize),
    c: char,
}

impl CharCoord {
    pub fn swap(&mut self) {
        self.coord = (self.coord.1, self.coord.0);
    }
}

macro_rules! new_coord {
    ($i:expr, $rails:expr, $period:expr) => {{
        let x = $i;
        let y = $rails - 1 - ($rails - 1 - x % $period).abs();

        (y, x)
    }};
}

macro_rules! new_coords {
    ($rails:expr, $period:expr, $text:ident) => {{
        let len = $text.len() as isize;
        (0..len).map(|i| new_coord!(i, $rails, $period))
    }};
}

macro_rules! encode_coords {
    ($text:ident, $iter:expr) => {
        $iter
            .zip($text.chars())
            .map(|(coord, c)| CharCoord { coord, c })
    };
}

pub struct RailFence {
    rails: isize,
    period: isize,
}

impl RailFence {
    pub fn new(rails: u32) -> Self {
        let rails = rails as isize;
        let period = ((rails - 1) * 2).max(1);

        Self { rails, period }
    }

    pub fn encode(&self, plain: &str) -> String {
        // Combine characters and rail fence coordinates sequentially
        let iter = new_coords!(self.rails, self.period, plain);
        let mut char_coords: Vec<CharCoord> = encode_coords!(plain, iter).collect();

        // Sorting the coordinates gives the encode order
        char_coords.sort();
        char_coords.iter().map(|t| t.c).collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        // Get rail fence coordinates
        let iter = new_coords!(self.rails, self.period, cipher);
        let mut coords: Vec<(isize, isize)> = iter.collect();

        // Sorting the coordinates gives the encode order
        coords.sort();

        // Combine characters and coordinates sequentially
        let mut char_coords: Vec<CharCoord> = encode_coords!(cipher, coords.drain(0..)).collect();

        // Swap coordinate orders
        char_coords.iter_mut().for_each(CharCoord::swap);

        // Now, sorting gives the decode order
        char_coords.sort();
        char_coords.iter().map(|t| t.c).collect()
    }
}
