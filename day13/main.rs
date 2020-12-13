use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let (earliest, buses) = {
        let mut input = BufReader::new(std::io::stdin()).lines();
        let earliest = input.next().unwrap().unwrap().parse::<i64>().unwrap();
        let buses: Vec<Option<i64>> = input
            .next()
            .unwrap()
            .unwrap()
            .split(',')
            .map(|x| x.parse().ok())
            .collect();
        (earliest, buses)
    };

    let a = {
        let active: Vec<i64> = buses.iter().flat_map(|&x| x).collect();
        let mut min_id = -1;
        let mut min_wait = i64::MAX;
        for &id in &active {
            let wait = (1 + (earliest - 1) / id) * id - earliest;
            if wait < min_wait {
                min_id = id;
                min_wait = wait;
            }
        }
        min_id * min_wait
    };

    let b = {
        let indexed: Vec<_> = buses
            .iter()
            .enumerate()
            .flat_map(|(i, &x)| x.map(|x| (i as i64, x)))
            .collect();
        // Chinese remainder theorem, following notation of Hardy & Wright.
        let mut x = 0;
        let m: i64 = indexed.iter().map(|a| a.1).product();
        for &(i, mi) in &indexed {
            #[allow(non_snake_case)]
            let Mi = m / mi;
            let ci = (mi - i) % mi + mi * (mi < i) as i64;
            let mut ni = 1;
            while (ni * Mi) % mi != 1 {
                ni += 1;
            }
            x += ni * Mi * ci;
            x %= m;
        }
        x
    };

    println!("a) {}", a);
    println!("b) {}", b);
}
