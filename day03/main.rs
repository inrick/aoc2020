use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let chart: Vec<Vec<u8>> = BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap().as_bytes().to_owned())
        .collect();

    let (rows, cols) = (chart.len(), chart[0].len());
    for c in &chart {
        assert_eq!(cols, c.len());
    }

    let a = {
        let mut col = 0;
        let mut trees = 0;
        for row in 0..rows {
            trees += (chart[row][col%cols] == b'#') as usize;
            col += 3;
        }
        trees
    };

    let b = {
        let mut prod = 1;
        for (right, down) in vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
            let mut trees = 0;
            let mut row = 0;
            let mut col = 0;
            // Suppose this is progress, no longer having a 3 clause for-loop.
            // Apparently for row in (0..rows).step_by(down) is an improvement.
            while row < rows {
                trees += (chart[row][col%cols] == b'#') as usize;
                row += down;
                col += right;
            }
            prod *= trees;
        }
        prod
    };

    println!("a) {}", a);
    println!("b) {}", b);
}
