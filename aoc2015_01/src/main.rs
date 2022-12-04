use std::env;
use std::io::*;
use std::fs::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).expect("Failed to open file"); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");
    
    let floor = data.chars().filter( |c| *c == '(').count() - data.chars().filter( |c| *c == ')').count();
    println!("Silver: {}", floor);

    let mut floor = 0;
    let mut pos = 1;
    for c in data.chars()
    {
        if c == '('
        {
            floor += 1;
        }
        else if c == ')'
        {
            floor -= 1;
        }
        else
        {
            panic!("Unexpected character");
        }
        if floor == -1
        {
            break;
        }
        pos += 1;
    }
    println!("Gold: {}", pos);
}
