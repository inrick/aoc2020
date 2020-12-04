use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Default)]
struct Passport {
    keys: Vec<String>,
    vals: Vec<String>,
}

impl Passport {
    pub fn find(&self, key: &str) -> Option<String> {
        for (i, k) in self.keys.iter().enumerate() {
            if k == key {
                return Some(self.vals[i].to_owned());
            }
        }
        return None;
    }
}

struct Rule<'a> {
    key: &'a str,
    pred: fn(&str) -> bool,
}

fn parse_num(s: &str) -> isize {
    s.parse().unwrap()
}

fn main() {
    let input = BufReader::new(std::io::stdin());

    let mut passports = Vec::new();
    let mut p = Passport::default();
    for l in input.lines() {
        let l = l.unwrap();
        if l == "" {
            passports.push(p);
            p = Passport::default();
            continue;
        }
        for s in l.split(" ") {
            let mut s = s.split(":");
            p.keys.push(s.next().unwrap().to_string());
            p.vals.push(s.next().unwrap().to_string());
        }
    }
    passports.push(p);

    let rules = [
        Rule {
            key: "byr",
            pred: |s| {
                let n = parse_num(s);
                1920 <= n && n <= 2002
            },
        },
        Rule {
            key: "iyr",
            pred: |s| {
                let n = parse_num(s);
                2010 <= n && n <= 2020
            },
        },
        Rule {
            key: "eyr",
            pred: |s| {
                let n = parse_num(s);
                2020 <= n && n <= 2030
            },
        },
        Rule {
            key: "hgt",
            pred: |s| {
                let len = s.len();
                if len < 2 {
                    return false;
                }
                match &s[len - 2..] {
                    "cm" => {
                        let n = parse_num(&s[..len - 2]);
                        150 <= n && n <= 193
                    }
                    "in" => {
                        let n = parse_num(&s[..len - 2]);
                        59 <= n && n <= 76
                    }
                    _ => false,
                }
            },
        },
        Rule {
            key: "hcl",
            pred: |s| {
                let b = s.as_bytes();
                b.len() == 7
                    && b[0] == b'#'
                    && b[1..].iter().all(|c| b"0123456789abcdef".contains(c))
            },
        },
        Rule {
            key: "ecl",
            pred: |s| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&s),
        },
        Rule {
            key: "pid",
            pred: |s| s.len() == 9 && s.as_bytes().iter().all(|c| b"0123456789".contains(c)),
        },
    ];

    let mut a = 0;
    let mut b = 0;
    for p in &passports {
        a += rules
            .iter()
            .all(|r| p.find(r.key).is_some()) as usize;
        b += rules
            .iter()
            .all(|r| p.find(r.key).map_or(false, |val| (r.pred)(&val))) as usize;
    }

    println!("a) {}", a);
    println!("b) {}", b);
}
