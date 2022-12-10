use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::*;
use std::io::*;

use itertools::Itertools;

fn read_line(s: &str) -> (String, String, usize)
{
    let mut parts = s.split(" = ");
    let mut nodes = parts.next().unwrap().split(" to ");
    let n1 = nodes.next().unwrap().to_string();
    let n2 = nodes.next().unwrap().to_string();
    let dist = parts.next().unwrap().parse::<usize>().unwrap();
    (n1, n2, dist)
}

fn find_dist(city1: &String, city2: &String, dists: &HashMap<(String, String), usize>) -> usize
{
    if let Some(dist) = dists.get(&(city1.to_string(), city2.to_string())) {
        *dist
    } else if let Some(dist) = dists.get(&(city2.to_string(), city1.to_string())) {
        *dist
    } else {
        panic!("No distance found for {} and {}", city1, city2);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap();
    let lines: Vec<_> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect();
    let nodes: Vec<_> = lines.iter().map(|l| read_line(l)).collect();

    let mut cities = nodes.iter().map(|(n1, n2, _)| n1).collect::<HashSet<_>>();
    cities.extend(nodes.iter().map(|(_, n2, _)| n2));

    let city_array = cities.iter().collect::<Vec<_>>();
    let mut dists: HashMap<(String, String), usize> = HashMap::new();

    let perms = city_array.iter().permutations(city_array.len());
    let mut min_dist = usize::max_value();
    nodes.iter().for_each(|(n1, n2, dist)| {
        dists.insert((n1.to_string(), n2.to_string()), *dist);
    });

    for perm in perms {
        let mut dist = 0;
        for i in 0..perm.len() - 1 {
            dist += find_dist(perm[i], perm[i + 1], &dists);
        }
        if dist < min_dist {
            min_dist = dist;
        }
    }
    
    println!("Silver: {}", min_dist);

    let mut max_dist = 0;
    let perms2 = city_array.iter().permutations(city_array.len());    
    for perm in perms2 {
        let mut dist = 0;
        for i in 0..perm.len() - 1 {
            dist += find_dist(perm[i], perm[i + 1], &dists);
        }
        if dist > max_dist {
            max_dist = dist;
        }
    }

    println!("Gold distance: {}", max_dist);
}
