pub struct Triangle([u64; 3]);

impl Triangle {
    pub fn build(mut sides: [u64; 3]) -> Option<Triangle> {
        sides.sort();
        let (a, b, c) = (sides[0], sides[1], sides[2]);
        if sides.contains(&0) || a + b < c || b + c < a || a + c < b {
            return None;
        }
        Some(Self(sides))
    }

    pub fn is_equilateral(&self) -> bool {
        let (a, c) = (self.0[0], self.0[2]);
        a == c
    }

    pub fn is_scalene(&self) -> bool {
        let (a, b, c) = (self.0[0], self.0[1], self.0[2]);
        a != b && b != c
    }

    pub fn is_isosceles(&self) -> bool {
        let (a, b, c) = (self.0[0], self.0[1], self.0[2]);
        a == b || b == c
    }
}
