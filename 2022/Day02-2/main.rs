use std::fs;
use std::collections::HashMap;

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();

    let mut hardcode_value: HashMap<&str, u64> = HashMap::from([
        ("A X", 3 + 0),
        ("A Y", 1 + 3),
        ("A Z", 2 + 6),

        ("B X", 1 + 0),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),

        ("C X", 2 + 0),
        ("C Y", 3 + 3),
        ("C Z", 1 + 6),
    ]);

    let mut score: u64 = 0;

    for el in input.lines() {
        score += hardcode_value[el];
    }

    println!("{}", score);
} 
