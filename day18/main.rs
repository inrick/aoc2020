use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let input: Vec<_> = BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap())
        .collect();

    let a = input
        .iter()
        .map(|l| eval(&parse_expr0_a(&mut Lexer::new(l.as_bytes()))))
        .sum::<i64>();

    let b = input
        .iter()
        .map(|l| eval(&parse_expr0_b(&mut Lexer::new(l.as_bytes()))))
        .sum::<i64>();

    println!("a) {}", a);
    println!("b) {}", b);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Token {
    EOF,
    Num(i64),
    Op(u8),
    LPar,
    RPar,
}

use Token::*;

#[derive(Debug)]
enum Expr {
    Int(i64),
    BinOp(u8, Box<Expr>, Box<Expr>),
}

use Expr::*;

struct Lexer<'a> {
    buf: &'a [u8],
    i: usize,
    tok: Token,
    tok_next: Token,
    peeked: bool,
}

impl Lexer<'_> {
    fn new(buf: &[u8]) -> Lexer {
        Lexer {
            buf: buf,
            i: 0,
            tok: EOF,
            tok_next: EOF,
            peeked: false,
        }
    }

    fn next(&mut self) -> Token {
        if self.peeked {
            self.peeked = false;
            self.tok = self.tok_next;
            return self.tok;
        }
        let len = self.buf.len();
        while self.i < len && self.buf[self.i].is_ascii_whitespace() {
            self.i += 1;
        }
        if self.i == len {
            return EOF;
        }
        let mut j = self.i + 1;
        let c = self.buf[self.i];
        let tok = match c {
            b'+' | b'*' => Op(c),
            b'(' => LPar,
            b')' => RPar,
            _ if c.is_ascii_digit() => {
                while j < len && self.buf[j].is_ascii_digit() {
                    j += 1;
                }
                let n = parse_num(&self.buf[self.i..j]);
                Num(n)
            }
            _ => panic!("unexpected: {}", c),
        };
        self.i = j;
        self.tok = tok;
        tok
    }

    fn peek(&mut self) -> Token {
        if self.peeked {
            return self.tok_next;
        }
        let prev = self.tok;
        self.tok_next = self.next();
        self.tok = prev;
        self.peeked = true;
        self.tok_next
    }
}

fn parse_num(s: &[u8]) -> i64 {
    let len = s.len();
    let mut n = 0;
    let mut mult = 1;
    for i in 0..len {
        n += mult * (s[len - i - 1] - b'0') as i64;
        mult *= 10;
    }
    n
}

fn parse_expr0_a(mut l: &mut Lexer) -> Expr {
    let mut e = parse_expr1_a(&mut l);
    while let Op(c) = l.peek() {
        l.next();
        e = BinOp(c, Box::new(e), Box::new(parse_expr1_a(&mut l)));
    }
    e
}

fn parse_expr1_a(mut l: &mut Lexer) -> Expr {
    match l.next() {
        Num(n) => Int(n),
        LPar => {
            let e = parse_expr0_a(&mut l);
            assert_eq!(l.next(), RPar);
            e
        }
        _ => panic!("token {:?}", l.tok),
    }
}

fn parse_expr0_b(mut l: &mut Lexer) -> Expr {
    let mut e = parse_expr1_b(&mut l);
    while l.peek() == Op(b'*') {
        l.next();
        e = BinOp(b'*', Box::new(e), Box::new(parse_expr1_b(&mut l)));
    }
    e
}

fn parse_expr1_b(mut l: &mut Lexer) -> Expr {
    let mut e = parse_expr2_b(&mut l);
    while l.peek() == Op(b'+') {
        l.next();
        e = BinOp(b'+', Box::new(e), Box::new(parse_expr2_b(&mut l)));
    }
    e
}

fn parse_expr2_b(mut l: &mut Lexer) -> Expr {
    match l.next() {
        Num(n) => Int(n),
        LPar => {
            let e = parse_expr0_b(&mut l);
            assert_eq!(l.next(), RPar);
            e
        }
        _ => panic!("token {:?}", l.tok),
    }
}

fn eval(expr: &Expr) -> i64 {
    match expr {
        &Int(n) => n,
        BinOp(b'+', e0, e1) => eval(&e0) + eval(&e1),
        BinOp(b'*', e0, e1) => eval(&e0) * eval(&e1),
        _ => panic!(),
    }
}
