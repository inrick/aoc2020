use std::io::BufRead;
use std::io::BufReader;

const NPREAMBLE: usize = 25;

fn main() {
    let nums: Vec<i64> = BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();

    let a = {
        let mut not_sum = 0;
        'search: for (i, &n) in nums.iter().enumerate().skip(NPREAMBLE) {
            for (j, &a) in nums[i - NPREAMBLE..i].iter().enumerate() {
                for &b in &nums[i - NPREAMBLE + j..i] {
                    if a + b == n {
                        continue 'search;
                    }
                }
            }
            not_sum = n;
            break;
        }
        not_sum
    };

    let b = {
        // Let i point to first element of sum, j to last.
        let mut i = 0;
        let mut j = 0;
        let mut sum = nums[i];
        loop {
            while sum < a {
                j += 1;
                sum += nums[j];
            }
            while a < sum {
                sum -= nums[i];
                i += 1;
            }
            if sum == a && i != j {
                let min = nums[i..=j].iter().min().unwrap();
                let max = nums[i..=j].iter().max().unwrap();
                break min + max;
            }
        }
    };

    println!("a) {}", a);
    println!("b) {}", b);
}
