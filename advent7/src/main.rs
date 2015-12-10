extern crate regex;

use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::fmt;

use regex::Regex;

enum Node {
    And(String, String),
    Or(String, String),
    Not(String),
    LShift(String, String), // (input, shift length input)
    RShift(String, String),
    Value(String),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Node::And(ref in1, ref in2) => write!(f, "And({} & {})", in1, in2),
            &Node::Or(ref in1, ref in2) => write!(f, "Or({} | {})", in1, in2),
            &Node::Not(ref val) => write!(f, "Not({})", val),
            &Node::LShift(ref val, ref shift) => write!(f, "LShift({} << {})", val, shift),
            &Node::RShift(ref val, ref shift) => write!(f, "RShift({} >> {})", val, shift),
            &Node::Value(ref val) => write!(f, "Value({})", val),
        }
    }
}

struct Circut {
    nodes: HashMap<String, Node>,
}

impl Circut {
    pub fn new() -> Self {
        Circut { nodes: HashMap::new() }
    }

    pub fn add_node(&mut self, n: &str) {
        let (output, node) = Self::parse_connection(n);
        if self.creates_loop(&node, &mut vec![output.clone()]) {
            panic!("A loop: {}", n);
        }
        if let Some(_) = self.nodes.insert(output, node) {
            panic!("Overwriting old output. This is wrong");
        }
    }

    pub fn v(&self, wire: &str) -> u16 {
        if let Some(node) = self.nodes.get(wire) {
            //println!("{} => {}", wire, node);
            match node {
                &Node::And(ref in1, ref in2) => self.v(in1) & self.v(in2),
                &Node::Or(ref in1, ref in2) => self.v(in1) | self.v(in2),
                &Node::Not(ref val) => !self.v(val),
                &Node::LShift(ref val, ref shift) => self.v(val) << self.v(shift),
                &Node::RShift(ref val, ref shift) => self.v(val) >> self.v(shift),
                &Node::Value(ref val) => self.v(val),
            }
        } else {
            //println!("value {}", wire);
            match wire.parse::<u16>() {
                Ok(i) => i,
                Err(_) => panic!("Invalid circut"),
            }
        }
    }

    fn creates_loop(&self, node: &Node, visited: &mut Vec<String>) -> bool {
        match node {
            &Node::And(ref in1, ref in2) => self.is_loop(in1, visited) || self.is_loop(in2, visited),
            &Node::Or(ref in1, ref in2) => self.is_loop(in1, visited) || self.is_loop(in2, visited),
            &Node::Not(ref val) => self.is_loop(val, visited),
            &Node::LShift(ref val, ref shift) => self.is_loop(val, visited) || self.is_loop(shift, visited),
            &Node::RShift(ref val, ref shift) => self.is_loop(val, visited) || self.is_loop(shift, visited),
            &Node::Value(ref val) => self.is_loop(val, visited),
        }
    }

    fn is_loop(&self, check: &str, visited: &mut Vec<String>) -> bool {
        if visited.contains(&check.to_string()) {
            println!("loop");
            true
        } else if let Some(node) = self.nodes.get(check) {
            println!("maybe");
            visited.push(check.to_string());
            self.creates_loop(node, visited)
        } else {
            println!("noop");
            false
        }
    }

    fn parse_connection(s: &str) -> (String, Node) {
        let binary = Regex::new("([:alnum:]+) (AND|OR|LSHIFT|RSHIFT) ([:alnum:]+) -> ([:alpha:]+)").unwrap(); // lf OP lq -> ls
        let unary = Regex::new("(?:(NOT) )?([:alnum:]+) -> ([:alpha:]+)").unwrap(); // NOT? kt -> ku
        if let Some(c) = binary.captures(s) {
            let op = c.at(2).unwrap().to_string();
            let out = c.at(4).unwrap().to_string();
            let in1 = c.at(1).unwrap().to_string();
            let in2 = c.at(3).unwrap().to_string();
            (out, match &op[..] {
                "AND" => Node::And(in1, in2),
                "OR" => Node::Or(in1, in2),
                "LSHIFT" => Node::LShift(in1, in2),
                "RSHIFT" => Node::RShift(in1, in2),
                _ => unreachable!(),
            })
        } else if let Some(c) = unary.captures(s) {
            let not = c.at(1).is_some();
            let in1 = c.at(2).unwrap().to_string();
            let out = c.at(3).unwrap().to_string();
            (out, match not {
                true => Node::Not(in1),
                false => Node::Value(in1),
            })
        } else {
            panic!("Invalid instruction")
        }
    }
}

fn main() {
    let mut circut = Circut::new();
    let data_file = File::open(Path::new("input")).unwrap();
    let data = BufReader::new(&data_file);
    for (line, i) in data.lines().zip(1..) {
        println!("Adding node {}", i);
        circut.add_node(&line.unwrap()[..]);
    }
    println!("Circuit parsed, calculating signal on a...");
    let value_a = circut.v("a");
    println!("Bobby will get signal {} on wire a", value_a);
}
