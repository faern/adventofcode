use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    let data_file = File::open(Path::new("input")).unwrap();
    let data = BufReader::new(&data_file);
    let mut nice_strings1 = 0;
    let mut nice_strings2 = 0;
    for line in data.lines() {
        let s = line.unwrap();
        if nice1(&s[..]) { nice_strings1 += 1; }
        if nice2(s) { nice_strings2 += 1; }
    }
    println!("The input has {} nice strings matching first rules", nice_strings1);
    println!("The input has {} nice strings matching the updated rules", nice_strings2);
}

fn nice1(line: &str) -> bool {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let mut evil_pairs = HashMap::new();
    evil_pairs.insert('b', 'a');
    evil_pairs.insert('d', 'c');
    evil_pairs.insert('q', 'p');
    evil_pairs.insert('y', 'x');

    let mut vowels_left = 3;
    let mut found_double_letter = false;
    let mut found_evil_pair = false;
    let mut last_c = '_';
    for c in line.chars() {
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
    vowels_left == 0 && found_double_letter && !found_evil_pair
}

fn nice2(s: String) -> bool {
    let len = s.len();
    let mut found_pair = false;
    let mut found_repeat_letter = false;
    for i in 0..(len-2) {
        if !found_repeat_letter && s[i..i+1] == s[i+2..i+3] {
            found_repeat_letter = true;
        }
        if !found_pair {
            for j in (i+2)..(len-1) {
                if s[i..i+2] == s[j..j+2] {
                    found_pair = true;
                    break;
                }
            }
        }
        if found_pair && found_repeat_letter { break; }
    }
    found_pair && found_repeat_letter
}
