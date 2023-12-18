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

type FieldDim = ((isize, isize), (isize,isize));

fn get_dims(instr: &Vec<Instruction>) -> FieldDim {
    use Direction::*;
    let mut min_x = isize::MAX;
    let mut max_x = isize::MIN;
    let mut min_y = isize::MAX;
    let mut max_y = isize::MIN;
    let mut x: isize = 0;
    let mut y: isize = 0;
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
    }
    ((min_x, max_x), (min_y, max_y))
}

fn normalize_coord(coord: (isize, isize), dims: FieldDim) -> (usize, usize) {
    ((coord.0 + dims.0.0.abs()) as usize, (coord.1 + dims.1.0.abs()) as usize)
}

fn draw_map(instr: &Vec<Instruction>, map: &mut Vec<Vec<char>>, dims: FieldDim) {
    use Direction::*;

    let mut x = 0isize;
    let mut y = 0isize;
    for i in instr {
        match i.dig {
            Left(n) | Right(n) => {
                //println!("{:?}", i.dig);
                let step;
                let to_x = if let Left(_) = i.dig {
                    let tmp = x.checked_sub_unsigned(n).unwrap();
                    step = -1isize;
                    tmp
                } else {
                    let tmp = x.checked_add_unsigned(n).unwrap();
                    step = 1isize;
                    tmp
                };
                while x != to_x {
                    let (tmp_x,tmp_y) = normalize_coord((x,y), dims);
                    map[tmp_y][tmp_x] = '#'; 
                    x = x.checked_add(step).unwrap();
                }
            },
            Up(n) | Down(n) => {
                //println!("{:?}", i.dig);
                let step;
                let to_y = if let Up(_) = i.dig {
                    let tmp = y.checked_sub_unsigned(n).unwrap();
                    step = -1isize;
                    tmp
                } else {
                    let tmp = y.checked_add_unsigned(n).unwrap();
                    step = 1isize;
                    tmp
                };
                while y != to_y {
                    let (tmp_x,tmp_y) = normalize_coord((x,y), dims);
                    map[tmp_y][tmp_x] = '#'; 
                    y = y.checked_add(step).unwrap();
                }
            },
        };

    }
}

fn find_fill_start_point(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (line_idx,line) in map.iter().enumerate() {
        let mut first = line.iter().position(|&c| c == '#').unwrap();
        let last = line.iter().rposition(|&c| c == '#').unwrap();
        while first < last {
            if line[first] == '.' {
                return Some((first, line_idx));
            }
            first += 1;
        }
    }
    None
}

const OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1)
];

fn flood_fill(map: &mut Vec<Vec<char>>, start: (usize,usize)) {
    let mut the_queue = VecDeque::new();
    the_queue.push_back(start);
    while !the_queue.is_empty() {
        let p = the_queue.pop_front().unwrap();
        if map[p.1][p.0] == '#' {
            continue;
        }
        
        //println!("---------------------------------");
        //println!("At ({},{})", p.0, p.1);
        // Process
        map[p.1][p.0] = '#';


        // Find neighbors
        for offset in OFFSETS {
            let neighbor = (p.0.checked_add_signed(offset.0), p.1.checked_add_signed(offset.1));
            if neighbor.0.is_none() || neighbor.1.is_none()
                || neighbor.0.unwrap() > map.first().unwrap().len()-1
                || neighbor.1.unwrap() > map.len()-1
                || map[neighbor.1.unwrap()][neighbor.0.unwrap()] == '#'
            {
                continue;
            }
            the_queue.push_back((neighbor.0.unwrap(), neighbor.1.unwrap()));
            //println!("\tAdding ({},{})", neighbor.0.unwrap(), neighbor.1.unwrap());
        }
    }
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
    
    /*
    for i in &instr {
        println!("{:?}", i);
    }
    */

    let dims = get_dims(&instr);
    let ((min_x, max_x), (min_y, max_y)) = dims;

    let mut map = std::iter::repeat(
            Vec::from_iter(std::iter::repeat('.')
                .take((max_x+min_x.abs()+1) as usize)
                .collect::<Vec<char>>()
            ))
            .take((max_y+min_y.abs()+1) as usize)
            .collect::<Vec<Vec<char>>>();

    println!("{}x{} pixels", map.first().unwrap().len(), map.len());


    draw_map(&instr, &mut map, dims);

    println!();

    //print_map(&map);

    let start = find_fill_start_point(&map);
    println!("Start point: {:?}", start);

    flood_fill(&mut map, start.unwrap());

    println!("Done filling.");
    print_map(&map);

    let sum = map
        .iter()
        .flatten()
        .fold(0usize, |acc,x| acc + match x {
            '#' => 1,
            '.' => 0,
            _ => panic!("Can't happen")
        });

    println!("Cubic meters: {}", sum);
}
