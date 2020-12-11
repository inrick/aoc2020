use std::io::BufRead;
use std::io::BufReader;

type Grid = Vec<Vec<u8>>;

fn main() {
    // . floor
    // # occupied
    // L empty
    let grid: Grid = BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap().into_bytes())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    for l in &grid {
        assert_eq!(l.len(), cols);
    }

    let a = {
        let mut grid = grid.clone();
        let mut changed = true;
        while changed {
            changed = false;
            let prev = grid.clone();
            for i in 0..rows {
                for j in 0..cols {
                    let tile = prev[i][j];
                    match (tile, na(&prev, i as i32, j as i32)) {
                        (b'L', 0) => grid[i][j] = b'#',
                        (b'#', x) if x >= 4 => grid[i][j] = b'L',
                        _ => continue,
                    };
                    changed = true;
                }
            }
        }
        count_occupied(&grid)
    };

    let b = {
        let mut grid = grid.clone();
        let mut changed = true;
        while changed {
            changed = false;
            let prev = grid.clone();
            for i in 0..rows {
                for j in 0..cols {
                    let tile = prev[i][j];
                    match (tile, nb(&prev, i as i32, j as i32)) {
                        (b'L', 0) => grid[i][j] = b'#',
                        (b'#', x) if x >= 5 => grid[i][j] = b'L',
                        _ => continue,
                    };
                    changed = true;
                }
            }
        }
        count_occupied(&grid)
    };

    println!("a) {}", a);
    println!("b) {}", b);
}

fn count_occupied(grid: &Grid) -> usize {
    grid.iter().flatten().filter(|&&c| c == b'#').count()
}

// Neighbors a
fn na(grid: &Grid, i: i32, j: i32) -> i32 {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut sum = 0;
    for x in -1..=1 {
        for y in -1..=1 {
            let (a, b) = (i + x, j + y);
            if (x == 0 && y == 0) || a < 0 || b < 0 || a >= rows || b >= cols {
                continue;
            }
            sum += (grid[a as usize][b as usize] == b'#') as i32;
        }
    }
    sum
}

// Neighbors b
fn nb(grid: &Grid, i: i32, j: i32) -> i32 {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut sum = 0;
    for x in -1..=1 {
        for y in -1..=1 {
            if x == 0 && y == 0 {
                continue;
            }
            let (mut a, mut b) = (i, j);
            loop {
                a += x;
                b += y;
                if a < 0 || b < 0 || a >= rows || b >= cols {
                    break;
                }
                let tile = grid[a as usize][b as usize];
                sum += (tile == b'#') as i32;
                if tile != b'.' {
                    break;
                }
            }
        }
    }
    sum
}
