use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();
    let mut total: u64 = 0;

    for el in input.lines() {
        let (el1, el2) = el.split_at(el.len()/2);
        let mut already_counted: Vec<char> = Vec::new();

        for elc1 in el1.chars() {
            for elc2 in el2.chars() {
                if elc1 == elc2 && !already_counted.contains(&elc1) {
                    if elc1 >= 'A' && elc1 <= 'Z' { // A - Z
                        total += elc1 as u64 - 38;
                    } else {
                        total += elc1 as u64 - 96;
                    }

                    already_counted.push(elc1);

                    break;
                }
            }
        }
    }

    println!("{}", total);
}