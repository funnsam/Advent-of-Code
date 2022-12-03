use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();
    let mut total: u64 = 0;

    let lines: Vec<&str> = input.split("\n").collect();

    for i in 0..=(lines.len()-1)/3 {
        let real_i = i*3;

        for el in lines[real_i].chars() {
            if lines[real_i+1].matches(el).count() != 0 && lines[real_i+2].matches(el).count() != 0 {
                total += calc_score(el);
                break;
            }
        }
    }

    println!("{}", total);
}

fn calc_score(ch: char) -> u64 {
    if ch >= 'A' && ch <= 'Z' {
        return ch as u64 - 38;
    }
    ch as u64 - 96
}