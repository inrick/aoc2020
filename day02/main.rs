use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Default)]
struct Policy {
    a: i32,
    b: i32,
    c: u8,
    p: Vec<u8>,
}

fn main() {
    let policies: Vec<Policy> = BufReader::new(std::io::stdin())
        .lines()
        .map(|l| parse_line(&l.unwrap()))
        .collect();

    let a = policies
        .iter()
        .filter(|&p| {
            let n = p.p.iter().filter(|&&c| c == p.c).count() as i32;
            p.a <= n && n <= p.b
        })
        .count();

    let b = policies
        .iter()
        .filter(|&p| {
            let (a, b) = (p.a as usize - 1, p.b as usize - 1);
            (p.p[a] == p.c) != (p.p[b] == p.c)
        })
        .count();

    println!("a) {}", a);
    println!("b) {}", b);
}

fn parse_line(s: &str) -> Policy {
    let mut splits = s.split(" ");
    let mut range = splits.next().unwrap().split("-");
    let (a, b) = (
        range.next().unwrap().parse().unwrap(),
        range.next().unwrap().parse().unwrap(),
    );
    let mut snd = splits.next().unwrap().bytes();
    let c = snd.next().unwrap();
    let p = splits.next().unwrap().into();

    assert_eq!(range.next(), None);
    assert_eq!(snd.next().unwrap(), b':');
    assert_eq!(snd.next(), None);
    assert_eq!(splits.next(), None);

    Policy { a, b, c, p }
}
