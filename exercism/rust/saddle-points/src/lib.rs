use num_traits::ToPrimitive;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

type Coord = (usize, usize);
type Neighbors = (Option<Coord>, Option<Coord>, Option<Coord>, Option<Coord>);

trait Cell {
    fn neighbor(&self, dir: Direction, w: usize, h: usize) -> Option<Coord>;
    fn neighbors(&self, w: usize, h: usize) -> Neighbors;
}

impl<T: ToPrimitive> Cell for (T, T) {
    fn neighbor(&self, dir: Direction, w: usize, h: usize) -> Option<Coord> {
        use Direction::*;

        let y = self.0.to_usize().unwrap();
        let x = self.1.to_usize().unwrap();
        match dir {
            Left  => if x == 0     { None } else { Some((y, x - 1)) }
            Right => if x >= w - 1 { None } else { Some((y, x + 1)) }
            Up    => if y == 0     { None } else { Some((y - 1, x)) }
            Down  => if y >= h - 1 { None } else { Some((y + 1, x)) }
        }
    }

    fn neighbors(&self, w: usize, h: usize) -> Neighbors {
        use Direction::*;

        let n = |dir| self.neighbor(dir, w, h);
        (n(Left), n(Right), n(Up), n(Down))
    }
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<Coord> {
    let h = input.len();
    let w = input.get(0).unwrap().len();
    let val = |p: Coord| &input[p.0][p.1];
    (0..w * h)
        .map(|i| (i / w, i % w))
        .filter(|p| {
            let v = val(*p);
            let (l, r, u, d) = p.neighbors(w, h);
            (l.is_none() || v >= val(l.unwrap()))
                && (r.is_none() || v >= val(r.unwrap()))
                && (u.is_none() || v <= val(u.unwrap()))
                && (d.is_none() || v <= val(d.unwrap()))
        })
        .collect()
}
