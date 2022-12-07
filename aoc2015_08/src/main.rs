use std::env;
use std::fs::*;
use std::io::*;

fn count_chars(s: &str) -> usize
{
    let mut count = 0;
    let mut chars = s.chars();
    while let Some(c) = chars.next()
    {
        if c == '\\'
        {
            let next = chars.next().unwrap();
            if next == 'x' {
                chars.next();
                chars.next();
            }
        }
        count += 1;
    }
    count - 2
}

fn count_extend_char(s: &str) -> usize
{
   
    let mut count = s.len();
    count += s.chars().filter(|c| *c == '"' || *c == '\\').count();
    count + 2
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap();
    let lines: Vec<_> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect();

    let silver = lines.iter().map(|line| line.len() - count_chars(line)).sum::<usize>();
    println!("Silver = {}", silver);

    let gold = lines.iter().map(|line| count_extend_char(line) - count_chars(line)).sum::<usize>();
    println!("Gold = {}", gold);
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_count_chars()
    {
        assert_eq!(count_chars(r#""""#), 0);
        assert_eq!(count_chars(r#""abc""#), 3);
        assert_eq!(count_chars(r#""aaa\"aaa""#), 7);
        assert_eq!(count_chars(r#""\x27""#), 1);
    }

    #[test]
    fn test_extend_chars()
    {
        assert_eq!(count_extend_char(r#""""#), 6);
        assert_eq!(count_extend_char(r#""abc""#), 9);
        assert_eq!(count_extend_char(r#""aaa\"aaa""#), 16);
        assert_eq!(count_extend_char(r#""\x27""#), 11);
    }
}