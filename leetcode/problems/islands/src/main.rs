pub struct Grid {
    pub h: isize,
    pub w: isize,
    pub cells: Vec<Vec<char>>,
}

impl Grid {
    fn is_land(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.w && y >= 0 && y < self.h && self.cells[y as usize][x as usize] == '1'
    }

    fn sink_island(&mut self, x: isize, y: isize) {
        if self.is_land(x, y) {
            self.cells[y as usize][x as usize] = '0';
            self.sink_island(x - 1, y);
            self.sink_island(x + 1, y);
            self.sink_island(x, y - 1);
            self.sink_island(x, y + 1);
        }
    }

    fn num_islands(&mut self) -> i32 {
        let mut n = 0;
        for y in 0..self.h {
            for x in 0..self.w {
                if self.is_land(x, y) {
                    self.sink_island(x, y);
                    n += 1;
                }
            }
        }
        n
    }
}

impl From<Vec<Vec<char>>> for Grid {
    fn from(other: Vec<Vec<char>>) -> Self {
        let h = other.len() as isize;
        Self {
            h,
            w: if h > 0 { other[0].len() as isize } else { 0 },
            cells: other,
        }
    }
}

fn main() {
    let grid = vec![
        vec!['1', '1', '1'],
        vec!['0', '1', '0'],
        vec!['1', '1', '1'],
    ];
    println!("{}", Grid::from(grid).num_islands());
}
