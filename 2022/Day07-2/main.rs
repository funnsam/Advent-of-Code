use std::fs;
use std::collections::HashMap;

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();
    let mut dirs: HashMap<String, u64> = HashMap::new();
    let mut cudir: String = ".".to_string();
    let mut state: u64 = 0;

    dirs.insert(".".to_string(), 0);

    for el in input.lines() {
        if state == 1 {
            if el.starts_with("$") {
                state = 0;
            } else {
                let thisf = el.split_whitespace().collect::<Vec<&str>>();
                if thisf[0] == "dir" {
                    dirs.insert(format!("{}/{}", cudir.clone(), thisf[1]), 0);
                } else {
                    dirs.insert(cudir.clone(), thisf[0].parse::<u64>().unwrap() + dirs.get(&cudir).unwrap());
                }
                continue;
            }
        }
        
        if el == "$ cd /" {
            cudir = ".".to_string();
        } else if el == "$ ls" {
            state = 1;
        } else if el == "$ cd .." {
            let mut a = cudir.split("/").collect::<Vec<&str>>();
            a.pop();

            cudir = a.join("/").to_string();
        } else if el.starts_with("$ cd") {
            let a = el.split_whitespace().collect::<Vec<&str>>();

            cudir = format!("{}/{}", cudir.clone(), a[2]);
        }
    }

    let mut total: u64 = 0; for (_, v) in dirs.iter() {total += v};
    let plzspace = 30000000-(70000000-total);
    let mut choices: HashMap<String, u64> = HashMap::new();

    for (k, v) in dirs.iter() {
        let mut sub = 0;
        for (k2, v2) in dirs.iter() {
            if k2.starts_with(&format!("{}/", k)) && k2 != k {
                sub += v2;
            }
        }

        if v + sub > plzspace {choices.insert(k.to_string(), v + sub);}
    }

    let mut a: Vec<u64> = choices.values().cloned().collect();
    a.sort();

    println!("{}", a[0]);
}