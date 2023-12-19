use regex::Regex;
use std::collections::VecDeque;

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
}

impl Instruction {
    fn new(s: &str) -> Self {
        let rex = Regex::new(r"([R,D,L,U]) (\d+) \(#([a-f0-9]{6})\)").unwrap();
        let cap = rex.captures(s).unwrap();
        // let dir = cap.get(1).unwrap().as_str();
        // let num = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
        // use Direction::*;
        // Self {
        //     dig: match dir {
        //         "R" => Right(num),
        //         "L" => Left(num),
        //         "U" => Up(num),
        //         "D" => Down(num),
        //         _ => panic!("Invalid input")
        //     },
        // }
        let color = cap.get(3).unwrap().as_str();
        let num = usize::from_str_radix(&color[..5], 16).unwrap();
        use Direction::*;
        Self {
            dig: match &color[5..6] {
                "0" => Right(num),
                "1" => Down(num),
                "2" => Left(num),
                "3" => Up(num),
                _ => panic!("Invalid input")
            },
        }
    }
}

type FieldDim = ((isize, isize), (isize,isize));

fn get_dims(instr: &Vec<Instruction>, vertices: &mut Vec<(isize,isize)>) -> FieldDim {
    use Direction::*;
    let mut min_x = isize::MAX;
    let mut max_x = isize::MIN;
    let mut min_y = isize::MAX;
    let mut max_y = isize::MIN;
    let mut x: isize = 0;
    let mut y: isize = 0;
    vertices.push((x,y));
    for i in instr {
        //println!("At ({},{})", x, y);
        match i.dig {
            Left(n) => x = x.checked_sub_unsigned(n).unwrap(),
            Right(n) => x = x.checked_add_unsigned(n).unwrap(),
            Up(n) => y = y.checked_sub_unsigned(n).unwrap(),
            Down(n) => y = y.checked_add_unsigned(n).unwrap()
        };
        min_x = std::cmp::min(min_x, x);
        max_x = std::cmp::max(max_x, x);
        min_y = std::cmp::min(min_y, y);
        max_y = std::cmp::max(max_y, y);

        vertices.push((x,y));
    }
    ((min_x, max_x), (min_y, max_y))
}

fn calc_area(instr: &Vec<Instruction>, height: usize) -> u128 {
    use Direction::*;

    let mut cur_height = height;
    let mut area = (1 * height) as i128;

    let dir = match instr.first().unwrap().dig {
        Left(_) => -1i128,
        Right(_) => 1i128,
        _ => panic!("Oops")
    };

    for i in instr {
        match i.dig {
            Right(n) => {
                let amount = (n * cur_height) as i128;
                println!("Right n = {}: Height: {}, Area: {}, amount = {}", n, cur_height, area, amount);
                area = area.checked_add(dir * amount).unwrap();
            },
            Down(n) => {
                cur_height -= n;
                println!("Down n = {}: Height: {}, Area: {}", n, cur_height, area);
            },
            Left(n) => {
                // -1 cos we only need to subtract what's OUTSIDE!
                let amount = (n * (cur_height-1)) as i128;
                println!("Left n = {}: Height: {}, Area: {}, amount = {}", n, cur_height, area, amount);
                area = area.checked_add(-dir * amount).unwrap();
            },
            Up(n) => {
                cur_height += n;
                println!("Up n = {}: Height: {}, Area: {}", n, cur_height, area);
            }
        }
    }

    area as u128
}

fn shoelace(vertices: &Vec<(isize,isize)>) -> usize {
    let mut area = vertices
        .windows(2)
        .map(|p| (p[0].0*p[1].1 - p[0].1*p[1].0)
            // Add edges
            + (p[0].0 - p[1].0).abs() 
            + (p[0].1 - p[1].1).abs())
        .sum::<isize>();

    area += (vertices.last().unwrap().0*vertices.first().unwrap().1 
        - vertices.last().unwrap().1*vertices.first().unwrap().0).abs();

    // Also add start pixel
    (area / 2 + 1) as usize
}

fn print_map(map: &Vec<Vec<char>>) {
    for line in map {
        for ch in line {
            print!("{}", ch);
        }
        println!();
    }
}

fn main() {
    use Direction::*;

    let instr = include_str!("../../input.txt")
        .lines()
        .map(Instruction::new)
        .collect::<Vec<Instruction>>();
    
    for i in &instr {
        println!("{:?}", i);
    }

    let mut vertices: Vec<(isize,isize)> = Vec::new();

    let dims = get_dims(&instr, &mut vertices);
    let ((min_x, max_x), (min_y, max_y)) = dims;
    let width = (max_x + min_x.abs() + 1) as usize;
    let height = (max_y + min_y.abs() + 1) as usize;

    println!("{}x{} pixels", width, height);

    println!();

    for p in &vertices {
        println!("({},{})", p.0, p.1);
    }
    let area = shoelace(&vertices);
    println!("Area: {}", area);
}
