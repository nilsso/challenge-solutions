#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot(i32, i32, Direction);

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self(x, y, d)
    }

    pub fn turn_right(mut self) -> Self {
        self.2 = {
            let dir = self.2;
            match dir {
                Direction::North => Direction::East,
                Direction::East  => Direction::South,
                Direction::South => Direction::West,
                Direction::West  => Direction::North,
            }
        };
        self
    }

    pub fn turn_left(mut self) -> Self {
        self.2 = {
            let dir = self.2;
            match dir {
                Direction::North => Direction::West,
                Direction::West  => Direction::South,
                Direction::South => Direction::East,
                Direction::East  => Direction::North,
            }
        };
        self
    }

    pub fn advance(mut self) -> Self {
        match self.2 {
            Direction::North => self.1 += 1,
            Direction::East  => self.0 += 1,
            Direction::South => self.1 -= 1,
            Direction::West  => self.0 -= 1,
        }
        self
    }

    pub fn instructions(mut self, instructions: &str) -> Self {
        for i in instructions.chars() {
            self = match i {
                'R' => self.turn_right(),
                'L' => self.turn_left(),
                'A' => self.advance(),
                _   => {
                    eprintln!("Invalid instruction {}", i);
                    self
                }
            };
        }
        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.0, self.1)
    }

    pub fn direction(&self) -> &Direction {
        &self.2
    }
}
