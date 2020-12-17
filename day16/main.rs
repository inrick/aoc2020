use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let (fields, ranges, my_ticket, all_tickets) = {
        let mut fields = Vec::new();
        let mut ranges = Vec::new();
        let mut my_ticket = Vec::new();
        let mut all_tickets = Vec::new();

        let mut st = 0;
        for l in BufReader::new(std::io::stdin()).lines() {
            let l = l.unwrap();
            if l == "" {
                st = -1;
                continue;
            }
            match st {
                -1 => match &l[..] {
                    "your ticket:" => st = 1,
                    "nearby tickets:" => st = 2,
                    _ => panic!(),
                },
                0 => {
                    let mut it = l.split(": ");
                    let field: String = it.next().unwrap().into();
                    fields.push(field);
                    let raw_ranges = it.next().unwrap().split(" or ");
                    let mut r = Vec::new();
                    for x in raw_ranges {
                        let mut it = x.split('-');
                        let a = it.next().unwrap().parse::<usize>().unwrap();
                        let b = it.next().unwrap().parse::<usize>().unwrap();
                        r.push(a..=b);
                    }
                    ranges.push(r);
                }
                1 => {
                    my_ticket = l
                        .split(',')
                        .map(|x| x.parse().unwrap())
                        .collect::<Vec<usize>>();
                }
                2 => {
                    all_tickets.push(
                        l.split(',')
                            .map(|x| x.parse().unwrap())
                            .collect::<Vec<usize>>(),
                    );
                }
                _ => panic!(),
            }
        }
        (fields, ranges, my_ticket, all_tickets)
    };

    let mut valid_tickets: Vec<Vec<usize>> = Vec::new();
    let a = {
        let all_ranges: Vec<_> = ranges.iter().flat_map(|x| x).collect();
        let mut sum = 0;
        'search: for ticket in all_tickets {
            for n in &ticket {
                if !all_ranges.iter().any(|r| r.contains(&n)) {
                    sum += n;
                    continue 'search;
                }
            }
            valid_tickets.push(ticket);
        }
        sum
    };

    let b = {
        let len = ranges.len();

        let mut inappropriate: Vec<(usize, Vec<bool>)> = Vec::new();
        for i in 0..len {
            inappropriate.push((i, vec![false; len]));
        }

        for t in &valid_tickets {
            for i in 0..len {
                for j in 0..len {
                    if !ranges[i].iter().any(|r| r.contains(&t[j])) {
                        inappropriate[i].1[j] = true;
                    }
                }
            }
        }
        inappropriate.sort_by_key(|v| len - v.1.iter().filter(|&&x| x).count());

        let mut assignment = vec![0; len];
        let mut taken = vec![false; len];
        for &(i, ref not_allowed) in &inappropriate {
            let mut j = 0;
            while taken[j] || not_allowed[j] {
                j += 1;
            }
            assignment[i] = j;
            taken[j] = true;
        }

        let mut prod = 1;
        for i in 0..len {
            if fields[i].starts_with("departure") {
                prod *= my_ticket[assignment[i]];
            }
        }
        prod
    };

    println!("a) {}", a);
    println!("b) {}", b);
}
