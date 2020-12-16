fn main() {
    let input: Vec<usize> = "14,3,1,0,9,5"
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let a = run(&input, 2020);
    let b = run(&input, 30_000_000);

    println!("a) {}", a);
    println!("b) {}", b);
}

fn run(initial: &Vec<usize>, until: usize) -> usize {
    let mut seen: Vec<usize> = vec![0; until];
    let len = initial.len();
    for i in 1..len {
        seen[initial[i - 1]] = i;
    }
    let mut n = initial[len - 1];
    let mut j;
    for i in len..until {
        j = seen[n];
        seen[n] = i;
        n = (j != 0) as usize * (i - j);
    }
    n
}
