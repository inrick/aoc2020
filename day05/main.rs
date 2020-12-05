use std::io::BufRead;
use std::io::BufReader;

fn bin(lo: u8, hi: u8, s: &[u8]) -> isize {
    let mut a = 0;
    let mut b = (1 << s.len()) - 1;
    for &x in s {
        let half = (b - a + 1) / 2;
        if x == hi {
            a += half;
        } else if x == lo {
            b -= half;
        } else {
            panic!("unexpected: {}", x);
        }
    }
    assert_eq!(a, b);
    a
}

fn main() {
    let input = BufReader::new(std::io::stdin());

    let mut a = -1;
    let mut seats = Vec::new();
    for l in input.lines() {
        let s = l.unwrap().as_bytes().to_owned();
        assert_eq!(s.len(), 10);
        let row = bin(b'F', b'B', &s[..7]);
        let col = bin(b'L', b'R', &s[7..]);
        let seat = row * 8 + col;
        seats.push(seat);
        a = std::cmp::max(a, seat);
    }
    seats.sort();

    let mut b = -1;
    for i in 1..seats.len() {
        if seats[i - 1] + 2 == seats[i] {
            b = seats[i] - 1;
            break;
        }
    }

    println!("a) {}", a);
    println!("b) {}", b);
}
