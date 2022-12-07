use std::collections::HashMap;
use std::env;
use std::fs::*;
use std::io::*;
use regex::Regex;

enum WireOp
{
    SetValue(u16),
    SetRef(String),
    Not(String),
    And(String, String),
    Or(String, String),
    Lshift(String, u16),
    Rshift(String, u16),
    AndNum(String, u16),
}

struct Wires
{
    ops: HashMap<String, WireOp>,
    signals: HashMap<String, u16>,
}

impl Wires {
    fn new() -> Self {
        Wires {
            ops: HashMap::new(),
            signals: HashMap::new(),
        }
    }

    fn signal(&mut self, name: &str) -> u16
    {
        if let Some(signal) = self.signals.get(name)
        {
            return *signal;
        }
        else
        {
            let op = self.ops.get(name).unwrap();
            let signal = match op
            {
                WireOp::SetValue(value) => *value,
                WireOp::SetRef(source) => self.signal(&source.clone()),
                WireOp::Not(source) => !self.signal(&source.clone()),
                WireOp::And(source1, source2) => {
                    let left = source1.clone();
                    let right = source2.clone();
                    self.signal(&left) & self.signal(&right) 
                },
                WireOp::Or(source1, source2) => {
                    let left = source1.clone();
                    let right = source2.clone();
                    self.signal(&left) | self.signal(&right) 
                },
                WireOp::Lshift(source, shift) => {
                    let left = source.clone();
                    let right = shift.clone();
                    self.signal(&left) << &right
                },
                WireOp::Rshift(source, shift) => {
                    let left = source.clone();
                    let right = shift.clone();
                    self.signal(&left) >> &right
                },
                WireOp::AndNum(source, value) => {
                    let left = source.clone();
                    let right = value.clone();
                    self.signal(&left) & right
                },
            };
            self.signals.insert(name.to_string(), signal);
            return signal;
        }
    }

    fn apply_wire_op(&mut self, line: &str)
    {
        let re_assign_imm = Regex::new(r"^(\d+)\s->\s([a-z]+)").unwrap();
        let re_assign_wire = Regex::new(r"^([a-z]+)\s->\s([a-z]+)").unwrap();
        let re_unary_op = Regex::new(r"^NOT\s([a-z]+)\s->\s([a-z]+)").unwrap();
        let re_binary_op = Regex::new(r"^([a-z]+)\s(\D+)\s([a-z]+)\s->\s([a-z]+)").unwrap();
        let re_binary_ops_imm_1st = Regex::new(r"^(\d+)\s(\D+)\s([a-z]+)\s->\s([a-z]+)").unwrap();
        let re_binary_ops_imm_2nd = Regex::new(r"^([a-z]+)\s(\D+)\s(\d+)\s->\s([a-z]+)").unwrap();
        
        if let Some(caps) = re_assign_imm.captures(line)
        {
            let value = caps.get(1).unwrap().as_str().parse::<u16>().unwrap();
            let dest = caps.get(2).unwrap().as_str();
            self.ops.insert(dest.to_string(), WireOp::SetValue(value));
        }
        else if let Some(caps) = re_assign_wire.captures(line)
        {
            let source = caps.get(1).unwrap().as_str();
            let dest = caps.get(2).unwrap().as_str();
            self.ops.insert(dest.to_string(), WireOp::SetRef(source.to_string()));
        }
        else if let Some(caps) = re_unary_op.captures(line)
        {
            let source = caps.get(1).unwrap().as_str();
            let dest = caps.get(2).unwrap().as_str();
            self.ops.insert(dest.to_string(), WireOp::Not(source.to_string()));
        }    
        else if let Some(caps) = re_binary_op.captures(line)
        {
            let source1: &str = caps.get(1).unwrap().as_str();
            let source2: &str = caps.get(3).unwrap().as_str();
            let dest = caps.get(4).unwrap().as_str();
            let op  = match caps.get(2).unwrap().as_str()
            {
                "AND" => WireOp::And(source1.to_string(), source2.to_string()),
                "OR" => WireOp::Or(source1.to_string(), source2.to_string()),
                _ => panic!("Invalid operator"),
            };
            self.ops.insert(dest.to_string(), op);
        }    
        else if let Some(caps) = re_binary_ops_imm_1st.captures(line)
        {
            let immediate_operand = caps.get(1).unwrap().as_str().parse::<u16>().unwrap();
            let source: &str = caps.get(3).unwrap().as_str();
            let dest = caps.get(4).unwrap().as_str();
            let op = match caps.get(2).unwrap().as_str()
            {
                "AND" => WireOp::AndNum(source.to_string(), immediate_operand),
                _ => panic!("Invalid operator"),
            };
            self.ops.insert(dest.to_string(), op);
        }
        else if let Some(caps) = re_binary_ops_imm_2nd.captures(line)
        {
            let source: &str = caps.get(1).unwrap().as_str();
            let immediate_operand_2 = caps.get(3).unwrap().as_str().parse::<u16>().unwrap();
            let dest = caps.get(4).unwrap().as_str();      
            let op = match caps.get(2).unwrap().as_str()
            {
                "LSHIFT" => WireOp::Lshift(source.to_string(), immediate_operand_2),
                "RSHIFT" => WireOp::Rshift(source.to_string(), immediate_operand_2),
                _ => panic!("Invalid operator"),
            }; 
            self.ops.insert(dest.to_string(), op);
        }
        else
        {
            panic!("Invalid line: {}", line);
        }
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
    let mut wires = Wires::new();    
    lines.iter().for_each(|line| wires.apply_wire_op(line));

    println!("Silver = {}", wires.signal("a"));

    let mut wires_gold = Wires::new();
    lines.iter().for_each(|line| wires_gold.apply_wire_op(line));
    wires_gold.apply_wire_op(&format!("{} -> b", wires.signal("a")));
    println!("Gold = {}", wires_gold.signal("a"));
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_assign_imm()
    {
        let mut wires = Wires::new();
        wires.apply_wire_op("123 -> x");
        assert_eq!(wires.signal(&"x".to_string()), 123);
    }

    #[test]  
    fn test_assign_from_wire()
    {
        let mut wires = Wires::new();
        wires.apply_wire_op("x -> y");
        wires.apply_wire_op("123 -> x");
        assert_eq!(wires.signal("y"), 123);
    }

    #[test]
    fn test_read_wire_unary_op()
    {
        let mut wires = Wires::new();
        wires.apply_wire_op("123 -> x");
        wires.apply_wire_op("NOT x -> y");
        assert_eq!(wires.signal("y"), !123);
    }

    #[test]
    fn test_read_wire_binary_op()
    {
        let mut wires = Wires::new();
        wires.apply_wire_op("x AND y -> z");
        wires.apply_wire_op("123 -> x");
        wires.apply_wire_op("456 -> y");
        assert_eq!(wires.signal("z"), 123&456);
    }
    
    #[test]
    fn test_read_wireop_binary_immediate_1st()
    {
        let mut wires = Wires::new();
        wires.apply_wire_op("0 AND x -> y");
        wires.apply_wire_op("123 -> x");
        assert_eq!(wires.signal("y"), 0);
    }
    
    #[test]
    fn test_run_testcase1()
    {
        let data = "123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i";
        let mut wires = Wires::new();
        data.lines().for_each(|line| wires.apply_wire_op(line));
        assert!(wires.signal("d") == 72);
        assert!(wires.signal("e") == 507);
        assert!(wires.signal("f") == 492);
        assert!(wires.signal("g") == 114);
        assert!(wires.signal("h") == 65412);
        assert!(wires.signal("i") == 65079);
        assert!(wires.signal("x") == 123);
        assert!(wires.signal("y") == 456);        
    }
}