use regex::Regex;

#[derive(Debug)]
enum Direction {
    Up(usize),
    Down(usize),
    Right(usize),
    Left(usize),
}

#[derive(Debug)]
struct Instruction {
    dig: Direction,
    color: String
}

impl Instruction {
    fn new(s: &str) -> Self {
        let rex = Regex::new(r"([R,D,L,U]) (\d+) \(#([a-f0-9]{6})\)").unwrap();
        let cap = rex.captures(s).unwrap();
        let dir = cap.get(1).unwrap().as_str();
        let num = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let color = cap.get(3).unwrap().as_str();
        use Direction::*;
        Self {
            dig: match dir {
                "R" => Right(num),
                "L" => Left(num),
                "U" => Up(num),
                "D" => Down(num),
                _ => panic!("Invalid input")
            },
            color: color.to_owned()
        }
    }
}
fn main() {
    let instr = include_str!("../../input2.txt")
        .lines()
        .map(Instruction::new)
        .collect::<Vec<Instruction>>();
    
    for i in instr {
        println!("{:?}", i);
    }
}
