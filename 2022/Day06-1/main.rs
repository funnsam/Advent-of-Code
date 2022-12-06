use std::fs;
use std::hash::Hash;
use std::collections::HashSet;

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();
    let mut nth_char: u64 = 0;

    let mut buf: Vec<char> = vec![' '; 4];

    for (i, el) in input.chars().enumerate() {
        nth_char += 1;

        buf.push(el);
        buf.remove(0);

        if has_unique_elements(buf.clone()) && i > 3 {
            break;
        }
    }

    eprintln!("{nth_char}");
}

fn has_unique_elements<T>(iter: T) -> bool where
    T: IntoIterator,
    T::Item: Eq + Hash, {
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}