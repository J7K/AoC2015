use std::env;
use std::io::*;
use std::fs::*;

fn main()
{
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap();
    let lines: Vec<_> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect();
    
    let boxes = lines.iter()
        .map(|line| line.as_str().split('x'))
        .into_iter()
        .map(|mut s| {
            let l = s.next().unwrap().parse::<usize>().unwrap();
            let w = s.next().unwrap().parse::<usize>().unwrap();
            let h = s.next().unwrap().parse::<usize>().unwrap();
            (l, w, h)
        })
        .collect::<Vec<_>>();

    let silver: usize = boxes.iter()
        .map(|(l, w, h)| required_wrap_surface(l, w, h))
        .sum();

    println!("Silver: {}", silver);

    let gold: usize = boxes.iter()
        .map(|(l, w, h)| required_ribbon_length(l, w, h))
        .sum();

    println!("Gold: {}", gold);
}

fn required_wrap_surface(l: &usize, w: &usize, h: &usize) -> usize {
    let lw = l * w;
    let wh = w * h;
    let hl = h * l;
    let min = lw.min(wh).min(hl);
    return 2 * lw + 2 * wh + 2 * hl + min;
}

fn required_ribbon_length(l: &usize, w: &usize, h: &usize) -> usize 
{
    let s1 = l + l + w + w;
    let s2 = w + w + h + h;
    let s3 = h + h + l + l;
    let min = s1.min(s2).min(s3);
    let volume = l * w * h;
    return min + volume;
}
