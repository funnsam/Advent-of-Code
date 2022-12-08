use std::fs;
 
fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();
    let mut incrd: u64 = 0;
    let mut prev: u64 = 0;

    for el in input.lines() {
        if el.parse::<u64>().unwrap() > prev {
            incrd += 1;
        }
        prev = el.parse::<u64>().unwrap();
    }

    println!("{}", incrd-1);
}
