use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();
    let mut rawcrate: Vec<String> = Vec::new();

    for el in input.lines() {   // Get the crates
        if el == "" {
            break;
        }
        rawcrate.push(el.to_string());
    }

    let n = rawcrate.len() - 1;  // Do that because Rust is bad
    rawcrate[n] = trim_whitespace(&rawcrate.last().unwrap().clone());

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..=15 {stacks.push(Vec::new());};

    for el in rawcrate.iter() {
        for (j, ch) in el.chars().enumerate() {
            if ch == ' ' {continue;};

            stacks[j/2].push(ch);
        }
    }

    for i in 0..=15 {stacks[i].pop();stacks[i].reverse();};
    //println!("{:?}", stacks);

    let mut skipped = false;
    for el in input.lines() {   // Sort the crates
        if el == ""      {skipped = true; continue;}
        else if !skipped {continue;}

        let numbers: _ = el.split_whitespace().collect::<Vec<&str>>();

        for _ in 1..=numbers[0].parse::<usize>().unwrap() {
            let a = stacks[numbers[1].parse::<usize>().unwrap()-1].pop().unwrap();
            stacks[numbers[2].parse::<usize>().unwrap()-1].push(a);
        }
        
        // println!("{:?}", stacks);
    }

    for e in &stacks {
        if e.len() == 0 {break;}
        print!("{}", e.last().unwrap());
    }
}

pub fn trim_whitespace(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    s.split_whitespace().for_each(|w| {
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(w);
    });
    result
}