use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let nums: Vec<isize> = BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap().parse::<isize>().unwrap())
        .collect();

    // Dumb loops since input is small.

    'search1: for (i, a) in nums.iter().enumerate() {
        for b in &nums[i..] {
            if a + b == 2020 {
                println!("a) {}", a * b);
                break 'search1;
            }
        }
    }

    'search2: for a in &nums {
        for b in &nums {
            for c in &nums {
                if a + b + c == 2020 {
                    println!("b) {}", a * b * c);
                    break 'search2;
                }
            }
        }
    }
}
