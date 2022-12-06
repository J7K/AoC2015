use std::env;
use std::fs::*;
use std::io::*;
use regex::Regex;

enum OpCode {
    On,
    Off,
    Toggle,
}

trait GridLike
{
    const GRID_SIZE: usize = 1000;

    fn new() -> Self;
    fn turn_on(&mut self, x: usize, y: usize);
    fn turn_off(&mut self, x: usize, y: usize);
    fn toggle(&mut self, x: usize, y: usize);
    fn apply_range(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, f: &dyn Fn(&mut Self, usize, usize))
    {
        for x in x0..=x1 {
            for y in y0..=y1 {
                f(self, x, y);
            }
        }
    }
    fn apply_op(&mut self, op: &GridOp)
    {
        match op.op_code {
            OpCode::On => self.apply_range(op.x0, op.y0, op.x1, op.y1, &Self::turn_on),
            OpCode::Off => self.apply_range(op.x0, op.y0, op.x1, op.y1, &Self::turn_off),
            OpCode::Toggle => self.apply_range(op.x0, op.y0, op.x1, op.y1, &Self::toggle),
        }
    }    
}

struct GridOnOff {
    grid: Vec<Vec<bool>>,
}

impl GridLike for GridOnOff
{
    fn new() -> Self {
        let mut grid = vec![vec![false; <GridOnOff as GridLike>::GRID_SIZE]; <GridOnOff as GridLike>::GRID_SIZE];
        
        GridOnOff { grid }
    }

    fn turn_on(&mut self, x: usize, y: usize) {
        self.grid[x][y] = true;
    }

    fn turn_off(&mut self, x: usize, y: usize) {
        self.grid[x][y] = false;
    }

    fn toggle(&mut self, x: usize, y: usize) {
        self.grid[x][y] = !self.grid[x][y];
    }
}

impl GridOnOff
{
    fn count_lights_on(&self) -> usize {
        self.grid.iter().flatten().filter(|&&x| x).count()
    }
}

struct GridBrightness {
    grid: Vec<Vec<usize>>,
}

impl GridLike for GridBrightness
{
    fn new() -> Self {
        let mut grid = vec![vec![0; <GridBrightness as GridLike>::GRID_SIZE]; <GridBrightness as GridLike>::GRID_SIZE];
        
        GridBrightness { grid }
    }

    fn turn_on(&mut self, x: usize, y: usize) {
        self.grid[x][y] += 1;
    }

    fn turn_off(&mut self, x: usize, y: usize) {
        if (self.grid[x][y] > 0) {
            self.grid[x][y] -= 1;
        }
    }

    fn toggle(&mut self, x: usize, y: usize) {
        self.grid[x][y] += 2;
    }
}

impl GridBrightness
{
    fn count_brightness(&self) -> usize {
        self.grid.iter().flatten().sum()
    }
}

struct GridOp {
    op_code: OpCode,
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
}

impl GridOp
{
    fn new(op_code_s: &str, x0_s: &str, y0_s: &str, x1_s: &str, y1_s: &str) -> GridOp {
        let op_code = match op_code_s {
            "turn on" => OpCode::On,
            "turn off" => OpCode::Off,
            "toggle" => OpCode::Toggle,
            _ => panic!("Unknown op code: {}", op_code_s),
        };
        let x0 = x0_s.parse::<usize>().unwrap();
        let y0 = y0_s.parse::<usize>().unwrap();
        let x1 = x1_s.parse::<usize>().unwrap();
        let y1 = y1_s.parse::<usize>().unwrap();
        GridOp { op_code, x0, y0, x1, y1 }
    }
}
 

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap(); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");
    
    let re = Regex::new(r"(turn\soff|turn\son|toggle)\s(\d+),(\d+)\D+(\d+),(\d+)").unwrap();
    let ops: Vec<GridOp> = re.captures_iter(&data)
        .map(|cap|GridOp::new(&cap[1], &cap[2], &cap[3], &cap[4], &cap[5]))
        .collect();
  
    let mut grid = GridOnOff::new();
    ops.iter().for_each(|op| grid.apply_op(op));
    let silver = grid.count_lights_on();
    println!("Silver: {}", silver);

    let mut grid2 = GridBrightness::new();
    ops.iter().for_each(|op| grid2.apply_op(op));
    let gold = grid2.count_brightness();
    println!("Gold: {}", gold);
}

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn test_turn_on_all()
    {
        let mut grid = GridOnOff::new();
        grid.apply_op(&GridOp::new("turn on", "0", "0", "999", "999"));
        assert_eq!(grid.count_lights_on(), <GridOnOff as GridLike>::GRID_SIZE * <GridOnOff as GridLike>::GRID_SIZE);
    }

    #[test]
    fn test_toggle_1st_row()
    {
        let mut grid = GridOnOff::new();
        grid.apply_op(&GridOp::new("toggle", "0", "0", "999", "0"));
        assert_eq!(grid.count_lights_on(), <GridOnOff as GridLike>::GRID_SIZE);
    }

    #[test]
    fn test_turn_on_middle_4()
    {
        let mut grid = GridOnOff::new();
        grid.apply_op(&GridOp::new("turn on", "499", "499", "500", "500"));
        assert_eq!(grid.count_lights_on(), 4);
    }

    #[test]
    fn test_turn_bright_1()
    {
        let mut grid = GridBrightness::new();
        grid.apply_op(&GridOp::new("turn on", "0", "0", "0", "0"));
        assert_eq!(grid.count_brightness(),  1);
    }

    #[test]
    fn test_turn_bright_all()
    {
        let mut grid = GridBrightness::new();
        grid.apply_op(&GridOp::new("turn on", "0", "0", "999", "999"));
        assert_eq!(grid.count_brightness(), <GridBrightness as GridLike>::GRID_SIZE * <GridBrightness as GridLike>::GRID_SIZE);
    }
}