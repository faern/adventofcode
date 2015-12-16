use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashSet, HashMap};

fn main() {
    let data_file = File::open(Path::new("input")).unwrap();
    let data = BufReader::new(&data_file);
    let mut cities_set = HashSet::new();
    let mut edges = HashMap::new();
    for line in data.lines().map(|l| l.unwrap()) {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let from = parts[0].to_string();
        let to = parts[2].to_string();
        let distance = parts[4].parse::<u32>().unwrap();
        cities_set.insert(from.to_string());
        cities_set.insert(to.to_string());
        edges.insert((from.clone(), to.clone()), distance.clone());
        edges.insert((to, from), distance);
    }

    let mut city_permutations = vec![];
    let mut cities = cities_set.into_iter().collect::<Vec<String>>();
    permute(cities.len(), &mut cities, &mut city_permutations);

    let mut minimum_distance = !0 as u32;
    for permutation in city_permutations {
        let mut distance = 0;
        let mut last_city = None;
        for city in permutation {
            if let Some(last_c) = last_city {
                let dist = edges.get(&(last_c, city.clone())).unwrap();
                distance += *dist;
            }
            last_city = Some(city);
        }
        if distance < minimum_distance {
            minimum_distance = distance;
        }
    }
    println!("The shortest way to everyone is {}", minimum_distance);
}

/// Heap's algorithm for permutations
fn permute<T: Clone>(n: usize, v: &mut Vec<T>, out: &mut Vec<Vec<T>>) {
    if n == 1 {
        out.push(v.clone());
    } else {
        for i in 0..n-1 {
            permute(n-1, v, out);
            if n % 2 == 0 {
                v.swap(i, n-1);
            } else {
                v.swap(0, n-1);
            }
        }
        permute(n-1, v, out);
    }
}
