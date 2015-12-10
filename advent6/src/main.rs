extern crate regex;

use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::cmp::{min, max};
use std::thread;

use regex::Regex;

const SIZE: usize = 1000;

fn main() {
    // Do everything in separate thread since that allows me to control stack size.
    // Main thread has a fixed stack.
    thread::Builder::new().stack_size(32 * 1024 * 1024).spawn(move || {
        let mut lights1 = [[false; SIZE]; SIZE];
        let mut lights2 = [[0 as u32; SIZE]; SIZE];

        let data_file = File::open(Path::new("input")).unwrap();
        let data = BufReader::new(&data_file);
        for line in data.lines() {
            let (action, x, y) = parse_instruction(&line.unwrap()[..]);
            apply_action1(&action[..], x, y, &mut lights1);
            apply_action2(&action[..], x, y, &mut lights2);
        }

        let mut lights_on = 0;
        let mut lights_value = 0;
        for x in 0..SIZE {
            for y in 0..SIZE {
                if lights1[x][y] { lights_on += 1; }
                lights_value += lights2[x][y];
            }
        }
        println!("After following the instructions {} lights are turned on", lights_on);
        println!("After reinterpreting the instructions and applying the new ones, we have a light value of {}", lights_value);
    }).unwrap().join().unwrap();
}

fn parse_instruction(instr: &str) -> (String, (usize, usize), (usize, usize)) {
    let re = Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let parts = re.captures(instr).unwrap();
    let action = parts.at(1).unwrap();
    let x1 = parts.at(2).unwrap().parse::<usize>().unwrap();
    let y1 = parts.at(3).unwrap().parse::<usize>().unwrap();
    let x2 = parts.at(4).unwrap().parse::<usize>().unwrap();
    let y2 = parts.at(5).unwrap().parse::<usize>().unwrap();

    let x = (min(x1, x2), max(x1, x2));
    let y = (min(y1, y2), max(y1, y2));
    (action.to_string(), x, y)
}

fn apply_action1(action: &str, (xl, xh): (usize, usize), (yl, yh): (usize, usize), lights: &mut [[bool; SIZE]; SIZE]) {
    for x in xl..xh+1 {
        for y in yl..yh+1 {
            let value = lights[x][y];
            lights[x][y] = match action {
               "turn on" => true,
               "turn off" => false,
               "toggle" => !value,
               _ => panic!("Invalid instruction"),
            };
        }
    }
}

fn apply_action2(action: &str, (xl, xh): (usize, usize), (yl, yh): (usize, usize), lights: &mut [[u32; SIZE]; SIZE]) {
    for x in xl..xh+1 {
        for y in yl..yh+1 {
            let value = lights[x][y];
            lights[x][y] = match action {
               "turn on" => value + 1,
               "turn off" => if value > 0 { value - 1 } else { 0 },
               "toggle" => value + 2,
               _ => panic!("Invalid instruction"),
            };
        }
    }
}
