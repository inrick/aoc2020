use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let input = BufReader::new(std::io::stdin());

    let mut groups: Vec<Vec<Vec<u8>>> = Vec::new();
    let mut g = Vec::new();
    for l in input.lines() {
        let l = l.unwrap();
        if l == "" {
            groups.push(g);
            g = Vec::new();
            continue;
        }
        g.push(l.as_bytes().to_owned());
    }
    groups.push(g);

    let mut a = 0;
    let mut b = 0;
    for group in &groups {
        let mut answers: [i32; 128] = [0; 128];
        for answer in group {
            for &c in answer {
                answers[c as usize] += 1;
            }
        }
        a += answers.iter().filter(|&&x| x > 0).count();
        b += answers.iter().filter(|&&x| x as usize == group.len()).count();
    }

    println!("a) {}", a);
    println!("b) {}", b);
}
