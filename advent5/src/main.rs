use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let mut evil_pairs = HashMap::new();
    evil_pairs.insert('b', 'a');
    evil_pairs.insert('d', 'c');
    evil_pairs.insert('q', 'p');
    evil_pairs.insert('y', 'x');

    let data_file = File::open(Path::new("input")).unwrap();
    let data = BufReader::new(&data_file);
    let mut nice_strings = 0;
    for line in data.lines() {
        let mut vowels_left = 3;
        let mut found_double_letter = false;
        let mut found_evil_pair = false;
        let mut last_c = '_';
        for c in line.unwrap().chars() {
            if vowels_left > 0 && vowels.contains(&c) {
                vowels_left -= 1;
            }
            if c == last_c {
                found_double_letter = true;
            }
            if let Some(prev) = evil_pairs.get(&c) {
                if *prev == last_c {
                    found_evil_pair = true;
                    break;
                }
            }
            last_c = c;
        }
        if vowels_left == 0 && found_double_letter && !found_evil_pair {
            nice_strings += 1;
        }
    }
    println!("The input has {} nice strings", nice_strings);
}
