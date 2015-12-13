use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::Chars;

fn main() {
    let data_file = File::open(Path::new("input")).unwrap();
    let data = BufReader::new(&data_file);

    let mut input_chars = 0;
    let mut mem_chars = 0;
    let mut encoded_chars = 0;
    for opt_line in data.lines() {
        let line = opt_line.unwrap();
        input_chars += line.len() as u32;
        mem_chars += count_mem_bytes(&line[..]);
        encoded_chars += encode(&line[..]).len() as u32;
    }
    println!("{} code chars minus {} mem chars = {}", input_chars, mem_chars, input_chars - mem_chars);
    println!("Encoding the {} input chars becomes {} encoded chars. An increase of {}", input_chars, encoded_chars, encoded_chars - input_chars);
}

fn count_mem_bytes(s: &str) -> u32 {
    let mut chars = s.chars();
    while let Some(c) = chars.next() { // Discard starting stuff
        match c {
            ' ' => (), // Skip whitespaces
            '"' => break, // String has started, stop discarding
            _ => panic!("Malformed string. {} can't appear before opening \"", c),
        }
    }
    let mut mem_bytes = 0;
    while let Some(c) = chars.next() {
        match c {
            '\\' => match chars.next() {
                Some('\\') => (),
                Some('"') => (),
                Some('x') => assert!(read_hex_byte(&mut chars)),
                _ => panic!("Malformed escape code"),
            },
            '"' => break, // String ended
            _ => (), // Any other char
        }
        mem_bytes += 1;
    }
    mem_bytes
}

fn read_hex_byte(iter: &mut Chars) -> bool {
    let (a, b) = (iter.next().unwrap(), iter.next().unwrap());
    a.is_digit(16) && b.is_digit(16)
}

fn encode(s: &str) -> String {
    let mut chars = s.chars();
    while let Some(c) = chars.next() { // Discard starting stuff
        match c {
            ' ' => (), // Skip whitespaces
            '"' => break, // String has started, stop discarding
            _ => panic!("Malformed string. {} can't appear before opening \"", c),
        }
    }
    let mut out_s = String::from("\"\\\"");
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                out_s.push_str("\\\\");
                match chars.next().unwrap() {
                    '\\' => out_s.push_str("\\\\"),
                    '"' => out_s.push_str("\\\""),
                    c2 => out_s.push(c2),
                }
            },
            '"' => break, // String ended
            _ => out_s.push(c), // Any other char
        }
    }
    out_s.push_str("\\\"\"");
    out_s
}
