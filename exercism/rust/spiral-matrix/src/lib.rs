const DIRECTIONS: &[(isize, isize); 4] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];

pub fn spiral_matrix(size: usize) -> Vec<Vec<u32>> {
    let mut spiral = vec![vec![0; size]; size];
    let mut dirs = DIRECTIONS.iter().cycle();
    let (mut x, mut y, mut i) = (-1, 0, 1..);
    for steps in (2..size * 2 + 1).map(|x| x / 2).rev() {
        let dir = dirs.next().unwrap();
        for _ in 0..steps {
            x += dir.0;
            y += dir.1;
            spiral[y as usize][x as usize] = i.next().unwrap();
        }
    }
    spiral
}
