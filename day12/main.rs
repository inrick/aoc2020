use std::io::BufRead;
use std::io::BufReader;
use std::ops::Mul;
use std::ops::AddAssign;
use std::ops::MulAssign;

#[derive(Clone, Copy)]
struct C(i64, i64);

impl AddAssign<C> for C {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

impl MulAssign<C> for C {
    fn mul_assign(&mut self, other: Self) {
        let re = self.0 * other.0 - self.1 * other.1;
        let im = self.0 * other.1 + self.1 * other.0;
        *self = Self(re, im);
    }
}

impl Mul<i64> for C {
    type Output = Self;
    fn mul(self, other: i64) -> Self {
        Self(self.0 * other, self.1 * other)
    }
}

fn main() {
    let dirs: Vec<(u8, i64)> = BufReader::new(std::io::stdin())
        .lines()
        .map(|l| parse_line(&l.unwrap()))
        .collect();

    let east = C(1, 0);
    let north = C(0, 1);
    let west = C(-1, 0);
    let south = C(0, -1);
    let rots = [east, north, west, south];
    let origin = C(0, 0);

    let a = {
        let mut pos = origin;
        let mut dir = east;
        for &(m, n) in &dirs {
            match m {
                b'F' => pos += dir * n,
                b'N' => pos += north * n,
                b'S' => pos += south * n,
                b'E' => pos += east * n,
                b'W' => pos += west * n,
                b'L' => dir *= rots[((n/90) % 4) as usize],
                b'R' => dir *= rots[(3*(n/90) % 4) as usize],
                _ => panic!("unknown direction {}", m),
            }
        }
        manhattan(pos, origin)
    };

    let b = {
        let mut waypoint = C(10, 1);
        let mut pos = origin;

        for &(m, n) in &dirs {
            match m {
                b'F' => pos += waypoint * n,
                b'N' => waypoint += north * n,
                b'S' => waypoint += south * n,
                b'E' => waypoint += east * n,
                b'W' => waypoint += west * n,
                b'L' => waypoint *= rots[((n/90) % 4) as usize],
                b'R' => waypoint *= rots[(3*(n/90) % 4) as usize],
                _ => panic!("unknown direction {}", m),
            }
        }
        manhattan(pos, origin)
    };

    println!("a) {}", a);
    println!("b) {}", b);
}

fn manhattan(a: C, b: C) -> i64 {
    i64::abs(a.0 - b.0) + i64::abs(a.1 - b.1)
}

fn parse_line(s: &str) -> (u8, i64) {
    let s = s.as_bytes();
    (
        s[0],
        std::str::from_utf8(&s[1..])
            .unwrap()
            .parse::<i64>()
            .unwrap(),
    )
}
