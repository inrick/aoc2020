use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
enum Instr {
    Mask(u64, u64, Vec<usize>),
    Mem(u64, u64),
}

use Instr::*;

fn main() {
    let input: Vec<_> = BufReader::new(std::io::stdin())
        .lines()
        .map(|l| parse_line(l.unwrap()))
        .collect();

    let a = {
        let mut mem = HashMap::new();
        let mut mask_and = !0;
        let mut mask_or = 0;
        for instr in &input {
            match instr {
                Mask(and, or, _) => {
                    mask_and = *and;
                    mask_or = *or;
                }
                Mem(a, b) => {
                    mem.insert(a, (b & mask_and) | mask_or);
                }
            };
        }
        mem.iter().map(|(_, v)| v).sum::<u64>()
    };

    let b = {
        let mut mem = HashMap::new();
        let mut mask_or = 0;
        let mut floating = &Vec::new();
        let mut n = 0;
        for instr in &input {
            match instr {
                Mask(_, or, fl) => {
                    mask_or = *or;
                    floating = &fl;
                    n = 1 << floating.len();
                }
                &Mem(mut a, b) => {
                    a |= mask_or;
                    for pat in 0..n {
                        mem.insert(apply(pat, floating, a), b);
                    }
                }
            };
        }
        mem.iter().map(|(_, v)| v).sum::<u64>()
    };

    println!("a) {}", a);
    println!("b) {}", b);
}

fn apply(pat: u64, floating: &Vec<usize>, mut x: u64) -> u64 {
    for (i, j) in floating.iter().enumerate() {
        match (pat >> i) & 1 {
            0 => x &= !(1 << j),
            1 => x |= 1 << j,
            _ => panic!(),
        };
    }
    x
}

fn parse_line(s: String) -> Instr {
    let mut it = s.split(" = ");
    let cmd = it.next().unwrap();
    if cmd == "mask" {
        let m: Vec<u8> = it.next().unwrap().into();
        let mut and = !0;
        let mut or = 0;
        let mut floating = Vec::new();
        for i in 0..36usize {
            match m[i] {
                b'0' => and &= !(1 << (35 - i)),
                b'1' => or |= 1 << (35 - i),
                b'X' => floating.push(35 - i),
                _ => panic!(),
            };
        }
        Mask(and, or, floating)
    } else if cmd.starts_with("mem") {
        let cmd = cmd.as_bytes();
        let addr = std::str::from_utf8(&cmd[4..cmd.len() - 1])
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let val = it.next().unwrap().parse::<u64>().unwrap();
        Mem(addr, val)
    } else {
        panic!()
    }
}
