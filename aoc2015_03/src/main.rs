use std::collections::HashSet;
use std::env;
use std::io::*;
use std::fs::*;

fn move_santa(pos: (i32, i32), dir: char) -> (i32, i32) {
    match dir {
        '^' => (pos.0, pos.1 + 1),
        'v' => (pos.0, pos.1 - 1),
        '>' => (pos.0 + 1, pos.1),
        '<' => (pos.0 - 1, pos.1),
        _ => panic!("Invalid direction"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).expect("Failed to open file"); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");

    let visited_house_coords: Vec<(i32, i32)> = data.chars().fold(vec![(0,0)], |mut visited, c| {
        let (x, y) = visited.iter().last().unwrap();
        visited.push(move_santa((*x, *y), c));
        visited
    });

    let visited_unique = visited_house_coords.iter().collect::<HashSet<_>>().len();
    println!("Silver: {}", visited_unique);

    let visited_coords_santa: Vec<(i32, i32)> = data.chars().step_by(2).fold(vec![(0,0)], |mut visited, c| {
        let (x, y) = visited.iter().last().unwrap();
        visited.push(move_santa((*x, *y), c));
        visited
    });

    let visited_coords_robot: Vec<(i32, i32)> = data.chars().skip(1).step_by(2).fold(vec![(0,0)], |mut visited, c| {
        let (x, y) = visited.iter().last().unwrap();
        visited.push(move_santa((*x, *y), c));
        visited
    });

    let visited_unique_2 = visited_coords_santa.iter().chain(visited_coords_robot.iter()).collect::<HashSet<_>>().len();
    println!("Gold: {}", visited_unique_2);
}
