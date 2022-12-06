use std::collections::VecDeque;
use std::env;
use std::fs::*;
use std::io::*;

fn rule1 (s: &str) -> bool {
    let mut count = 0;
    for c in s.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => count += 1,
            _ => (),
        }
    }
    count >= 3
}

fn rule2 (s: &str) -> bool {
    let mut prev = ' ';
    for c in s.chars() {
        if c == prev {
            return true;
        }
        prev = c;
    }
    false
}

fn rule3 (s: &str) -> bool {
    let mut prev = ' ';
    for c in s.chars() {
        match (prev, c) {
            ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => return false,
            _ => (),
        }
        prev = c;
    }
    true
}

fn rule4 (s: &str) -> bool {
    let mut prevprev = ' ';
    let mut prev = s.chars().nth(0).unwrap();
    let mut pairs = Vec::new();
    for c in s.chars().skip(1) {
        if pairs.contains(&(prev,c)) && ((prevprev , prev) != (prev, c)) {
            return true;
        }
        pairs.push((prev,c));  
        prevprev = prev;
        prev = c;
    }
    false
}

fn rule5 (s: &str) -> bool {
    if s.len() < 3 {
        return false;
    }
    let mut buffer = VecDeque::new();
    buffer.push_back(s.chars().nth(0).unwrap());
    buffer.push_back(s.chars().nth(1).unwrap());
    
    for c in s.chars().skip(2) {
        buffer.push_back(c);
        if buffer.front() == buffer.back() {
            return true;
        }
        buffer.pop_front();
    }
    false
}

fn apply_ruleset(s: &str, rules: &[fn(&str) -> bool]) -> bool {
    for rule in rules {
        if !rule(s) {
            return false;
        }
    }
    true
}
fn main() 
{
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap();
    let lines: Vec<_> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect();

   
    let silver = lines.iter().map(|line|apply_ruleset(line.as_str(), &[rule1, rule2, rule3])).filter(| x | *x).count();
    println!("Silver: {}", silver);

    let gold = lines.iter().map(|line|apply_ruleset(line.as_str(), &[rule4, rule5])).filter(| x | *x).count();
    println!("Gold: {}", gold);
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_ruleset1_1()
    {
        let ruleset = [rule1, rule2, rule3];
        const DATA: &str = "ugknbfddgicrmopn";
        assert!(apply_ruleset(DATA, &ruleset));
    }
    #[test]
    fn test_ruleset1_2()
    {
        let ruleset = [rule1, rule2, rule3];
        const DATA: &str = "aaa";
        assert!(apply_ruleset(DATA, &ruleset));
    }
    #[test]
    fn test_ruleset1_3()
    {
        let ruleset = [rule1, rule2, rule3];
        const DATA: &str = "jchzalrnumimnmhp";
        assert!(!apply_ruleset(DATA, &ruleset));
    }
    #[test]
    fn test_ruleset1_4()
    {
        let ruleset = [rule1, rule2, rule3];
        const DATA: &str = "haegwjzuvuyypxyu";
        assert!(!apply_ruleset(DATA, &ruleset));
    }
    #[test]
    fn test_ruleset1_5()
    {
        let ruleset = [rule1, rule2, rule3];
        const DATA: &str = "dvszwmarrgswjxmb";
        assert!(!apply_ruleset(DATA, &ruleset));
    }
    #[test]
    fn test_ruleset2_1()
    {
        let ruleset = [rule4, rule5];
        const DATA: &str = "qjhvhtzxzqqjkmpb";
        assert!(apply_ruleset(DATA, &ruleset));
    }
    #[test]
    fn test_ruleset2_2()
    {
        let ruleset = [rule4, rule5];
        const DATA: &str = "xxyxx";
        assert!(apply_ruleset(DATA, &ruleset));
    }
    #[test]
    fn test_ruleset2_3()
    {
        let ruleset = [rule4, rule5];
        const DATA: &str = "uurcxstgmygtbstg";
        assert!(!apply_ruleset(DATA, &ruleset));
    }    
    #[test]
    fn test_ruleset2_4()
    {
        let ruleset = [rule4, rule5];
        const DATA: &str = "ieodomkazucvgmuy";
        assert!(!apply_ruleset(DATA, &ruleset));
    }
    #[test]
    fn test_rule4()
    {
        const DATA: &str = "ueihvxviirnooomi";
        assert!(!rule4(DATA));
    }

}