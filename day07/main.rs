use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;

type Bags = HashMap<String, Vec<(i32, String)>>;

fn main() {
    let bags: Bags = BufReader::new(std::io::stdin())
        .lines()
        .map(|l| parse_line(&l.unwrap()))
        .collect();

    // Subtract "shiny gold" bag itself from both a and b.
    let a = bags.iter().filter(|(c, _)| contains(&bags, c)).count() - 1;
    let b = count(&bags, "shiny gold") - 1;

    println!("a) {}", a);
    println!("b) {}", b);
}

fn contains(bags: &Bags, color: &str) -> bool {
    color == "shiny gold"
        || bags
            .get(color)
            .unwrap()
            .iter()
            .any(|(_, c)| contains(bags, c))
}

fn count(bags: &Bags, color: &str) -> i32 {
    1 + bags
        .get(color)
        .unwrap()
        .iter()
        .map(|(n, c)| n * count(bags, c))
        .sum::<i32>()
}

// Apologies. Will eventually try to learn some rust.
fn parse_line(s: &str) -> (String, Vec<(i32, String)>) {
    let mut it = s.split(" ");
    let color = it.next().unwrap().to_owned() + " " + it.next().unwrap();

    assert_eq!(it.next(), Some("bags"));
    assert_eq!(it.next(), Some("contain"));

    let mut contains = Vec::new();
    while let Some(next) = it.next() {
        if let Ok(n) = next.parse::<i32>() {
            let color = it.next().unwrap().to_owned() + " " + it.next().unwrap();
            let next = it.next().unwrap();
            assert!(["bag,", "bags,", "bag.", "bags."].contains(&next));
            contains.push((n, color));
        } else {
            assert_eq!(next, "no");
            assert_eq!(it.next(), Some("other"));
            assert_eq!(it.next(), Some("bags."));
        }
    }
    (color, contains)
}
