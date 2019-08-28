fn all_one_type(grid: &Vec<Vec<i32>>) -> bool {
    grid.iter().all(|row| row.iter().all(|&v| v == 0)) ||
        grid.iter().all(|row| row.iter().all(|&v| v == 1))
}

fn distance(a: &(i32, i32), b: &(i32, i32)) -> i32 {
    let dist = (a.0 - b.0).abs() + (a.1 - b.1).abs();
    println!("distance({:?}, {:?}) = {}", a, b, dist);
    dist
}

struct Solution;

impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        if all_one_type(&grid) {
            -1
        } else {
            let mut waters: Vec<(i32, i32)> = Vec::new();
            let mut lands: Vec<(i32, i32)> = Vec::new();
            for (y, row) in grid.iter().enumerate() {
                for (x, v) in row.iter().enumerate() {
                    let (x, y) = (x as i32, y as i32);
                    match v {
                        0 => waters.push((x, y)),
                        1 => lands.push((x, y)),
                        _ => unreachable!(),
                    }
                }
            }
            println!("waters:");
            for water in waters.iter() {
                println!("    {:?}", water);
            }
            println!("lands:");
            for land in lands.iter() {
                println!("    {:?}", land);
            }
            let mut max_dist = 0;
            for water in waters.iter() {
                let mut min_dist = 99999;
                for land in lands.iter() {
                    min_dist = min_dist.min(distance(water, land))
                }
                max_dist = max_dist.max(min_dist);
            }
            max_dist
        }
    }
}

fn main() {
    let grid = vec![
        vec![1,0,1],
        vec![0,0,0],
        vec![1,0,1],
    ];

    println!("{}", Solution::max_distance(grid));
}
