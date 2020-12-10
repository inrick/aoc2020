use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let nums = {
        let mut nums: Vec<i64> = BufReader::new(std::io::stdin())
            .lines()
            .map(|l| l.unwrap().parse().unwrap())
            .collect();
        nums.sort();
        // Add outlet and built-in adapter
        nums.insert(0, 0);
        nums.push(nums[nums.len() - 1] + 3);
        nums
    };

    let diffs = nums.windows(2).map(|a| a[1] - a[0]).collect::<Vec<i64>>();

    let a = {
        let d1 = diffs.iter().filter(|&&n| n == 1).count();
        let d3 = diffs.iter().filter(|&&n| n == 3).count();
        d1 * d3
    };

    let b = diffs
        .split(|&n| n == 3)
        .map(|x| match x.len() {
            // Manual enumeration after seeing how few the alternatives were
            0 | 1 => 1, //    1
            2 => 2,     //   11   02
            3 => 4,     //  111  102  003  021
            4 => 7,     // 1111 1102 1003 1021 0031 0211 0202
            _ => panic!(),
        })
        .product::<i64>();

    println!("a) {}", a);
    println!("b) {}", b);
}
