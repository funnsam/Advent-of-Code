use std::fs;
 
fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let mut incrd: u64 = 0;
    let mut prev: u64 = 0;

    for (i, el) in input.lines().enumerate() {
        if i < 3 {continue;}

        if el.parse::<u64>().unwrap()+lines[i-1].parse::<u64>().unwrap()+lines[i-2].parse::<u64>().unwrap() > prev {
            incrd += 1;
        }
        prev = el.parse::<u64>().unwrap()+lines[i-1].parse::<u64>().unwrap()+lines[i-2].parse::<u64>().unwrap();
    }

    println!("{}", incrd-1);
}
