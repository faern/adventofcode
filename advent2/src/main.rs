use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let data_file = File::open(Path::new("input")).unwrap();
    let data = BufReader::new(&data_file);
    let mut needed_paper = 0;
    let mut needed_ribbon = 0;
    let mut num_packages = 0;
    for line in data.lines() {
        let pkg = Pkg::new(&line.unwrap()[..]);
        needed_paper += pkg.needed_paper();
        needed_ribbon += pkg.needed_ribbon();
        num_packages += 1;
    }
    println!("The elves need to order {} square feet of paper, and {} feet of ribbon, to wrap their {} gifts",
             needed_paper, needed_ribbon, num_packages);
}

struct Pkg {
    sides: Vec<u32>,
}

impl Pkg {
    pub fn new(pkg: &str) -> Self {
        let sides: Vec<u32> = pkg.split("x").map(|s| s.parse().unwrap()).collect();
        Pkg { sides: sides }
    }

    pub fn needed_paper(&self) -> u32 {
        let len = self.sides.len();
        let mut sides_size = 0;
        let mut smallest_side_size = !0 as u32; // Start with MAX
        for i in 0..len {
            for j in (i+1)..len {
                let side = self.sides[i] * self.sides[j];
                sides_size += side;
                if side < smallest_side_size {
                    smallest_side_size = side;
                }
            }
        }
        2 * sides_size + smallest_side_size
    }

    pub fn needed_ribbon(&self) -> u32 {
        let mut smallest = !0 as u32;
        let mut second_smallest = !0 as u32;
        for side in self.sides.iter() {
            if *side <= smallest {
                second_smallest = smallest;
                smallest = *side;
            } else if *side < second_smallest {
                second_smallest = *side;
            }
        }
        2 * (smallest + second_smallest) + self.pkg_volume()
    }

    pub fn pkg_volume(&self) -> u32 {
        let mut product = 1;
        for side in self.sides.iter() {
            product *= *side
        }
        product
    }
}
