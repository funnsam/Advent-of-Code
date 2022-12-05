use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();
    let mut total: u64 = 0;

    for el in input.lines() {
        let els = el.split(",").collect::<Vec<_>>();

        let firstrange  = els[0].split("-").collect::<Vec<_>>();
        let secondrange = els[1].split("-").collect::<Vec<_>>();

        'a: for i in firstrange[0].parse::<u64>().unwrap()..=firstrange[1].parse::<u64>().unwrap() {
            for j in secondrange[0].parse::<u64>().unwrap()..=secondrange[1].parse::<u64>().unwrap() {
                if i == j {
                    total += 1;
                    break 'a;
                }
            }
        }
    }

    println!("{}", total);
}