use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

use Instruction::*;

fn main() {
    let instrs: Vec<Instruction> = BufReader::new(std::io::stdin())
        .lines()
        .map(|l| parse_line(&l.unwrap()))
        .collect();

    let (a, _) = exec(&instrs);

    let b = {
        let mut b = 0;
        let len = instrs.len();
        'search: for i in 0..len {
            let mut instrs = instrs.clone();
            match instrs[i] {
                Acc(_) => continue,
                Jmp(n) => instrs[i] = Nop(n),
                Nop(n) => instrs[i] = Jmp(n),
            };
            let (acc, pc) = exec(&instrs);
            if pc == len as i32 {
                b = acc;
                break 'search;
            }
        }
        b
    };

    println!("a) {}", a);
    println!("b) {}", b);
}

fn exec(instrs: &Vec<Instruction>) -> (i32, i32) {
    let len = instrs.len() as i32;
    let mut acc: i32 = 0;
    let mut pc: i32 = 0;
    let mut seen = vec![false; len as usize];
    while 0 <= pc && pc < len && !seen[pc as usize] {
        seen[pc as usize] = true;
        match instrs[pc as usize] {
            Acc(n) => acc += n,
            Jmp(n) => pc += n-1, // Always +1 at bottom of loop
            Nop(_) => (),
        };
        pc += 1;
    }
    (acc, pc)
}

fn parse_line(s: &str) -> Instruction {
    let mut it = s.split(" ");
    let instr = it.next().unwrap();
    let n = it.next().unwrap().parse::<i32>().unwrap();
    assert_eq!(it.next(), None);
    match instr {
        "acc" => Acc(n),
        "jmp" => Jmp(n),
        "nop" => Nop(n),
        _ => panic!("unknown instruction {}", instr),
    }
}
