use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();

    let mut curr_cal: u64 = 0;
    let mut cals: Vec<u64> = Vec::new();

    for el in input.lines() {
        if el == "" {
            cals.push(curr_cal);
            curr_cal = 0;
        } else {
            curr_cal += el.parse::<u64>().unwrap();
        }
    }

    cals.sort();
    cals.reverse();

    println!("{}", cals[0] + cals[1] + cals[2]);
}
