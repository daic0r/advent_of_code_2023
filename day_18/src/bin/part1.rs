use regex::Regex;

#[derive(Debug, Clone)]
enum Direction {
    Up(usize),
    Down(usize),
    Right(usize),
    Left(usize),
}

#[derive(Debug, Clone)]
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

fn get_dims(instr: &Vec<Instruction>) -> (usize, usize) {
    use Direction::*;
    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut min_y = usize::MAX;
    let mut max_y = usize::MIN;
    let mut x: usize = 0;
    let mut y: usize = 0;
    for i in instr {
        match i.dig {
            Left(n) => x = x.checked_sub(n).unwrap(),
            Right(n) => x = x.checked_add(n).unwrap(),
            Up(n) => y = y.checked_sub(n).unwrap(),
            Down(n) => y = y.checked_add(n).unwrap()
        };
        min_x = std::cmp::min(min_x, x);
        max_x = std::cmp::max(max_x, x);
        min_y = std::cmp::min(min_y, y);
        max_y = std::cmp::max(max_y, y);
    }
    max_x += 1;
    max_y += 1;

    (max_x, max_y)
}

fn draw_map(instr: &Vec<Instruction>, map: &mut Vec<Vec<char>>) {
    use Direction::*;

    let mut x = 0usize;
    let mut y = 0usize;
    for i in instr {
        match i.dig {
            Left(n) | Right(n) => {
                println!("{:?}", i.dig);
                let step;
                let to_x = if let Left(_) = i.dig {
                    let tmp = x.checked_sub(n).unwrap();
                    step = -1isize;
                    tmp
                } else {
                    let tmp = x.checked_add(n).unwrap();
                    step = 1isize;
                    tmp
                };
                while x != to_x {
                    map[y][x] = '#'; 
                    x = x.checked_add_signed(step).unwrap();
                }
            },
            Up(n) | Down(n) => {
                println!("{:?}", i.dig);
                let step;
                let to_y = if let Up(_) = i.dig {
                    let tmp = y.checked_sub(n).unwrap();
                    step = -1isize;
                    tmp
                } else {
                    let tmp = y.checked_add(n).unwrap();
                    step = 1isize;
                    tmp
                };
                while y != to_y {
                    map[y][x] = '#'; 
                    y = y.checked_add_signed(step).unwrap();
                }
            },
        };

    }
}

fn main() {
    use Direction::*;

    let instr = include_str!("../../input2.txt")
        .lines()
        .map(Instruction::new)
        .collect::<Vec<Instruction>>();
    
    for i in &instr {
        println!("{:?}", i);
    }

    let (max_x, max_y) = get_dims(&instr);
    println!("Dimensions: {}, {}", max_x, max_y);

    let mut map = std::iter::repeat(
            Vec::from_iter(std::iter::repeat('.')
                .take(max_x)
                .collect::<Vec<char>>()
            ))
            .take(max_y)
            .collect::<Vec<Vec<char>>>();


    draw_map(&instr, &mut map);

    for line in &map {
        for ch in line {
            print!("{}", ch);
        }
        println!();
    }
}
