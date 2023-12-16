use std::collections::{VecDeque, HashSet};

type Grid = Vec<Vec<GridCell>>;
type Vec2 = (isize, isize);
type GridPos = (usize, usize);

#[derive(Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
struct GridCell {
    cell: char,
    energized: bool
}

impl GridCell {
    fn new(ch: char) -> Self {
        GridCell {
            cell: ch,
            energized: false
        }
    }
}


fn trace_beam(grid: &mut Grid, start: GridPos, move_dir: Vec2) -> Option<(GridPos, [Vec2; 2])> {
    let mut cur_pos = start;
    let mut cur_dir = move_dir;
    loop {
        let new_x = cur_pos.0.checked_add_signed(cur_dir.0);
        if new_x.is_none() || new_x.unwrap() > grid[0].len()-1 {
            if cfg!(feature="debug_output") {
                println!("END: Leaving grid at {:?}+{:?}", cur_pos, cur_dir);
            }
            break;
        }
        let new_y = cur_pos.1.checked_add_signed(cur_dir.1);
        if new_y.is_none() || new_y.unwrap() > grid.len()-1 {
            if cfg!(feature="debug_output") {
                println!("END: Leaving grid at {:?}+{:?}", cur_pos, cur_dir);
            }
            break;
        }
        cur_pos = (new_x.unwrap(), new_y.unwrap());
        let mut cur_cell = &mut grid[cur_pos.1][cur_pos.0];
        match cur_cell.cell {
            '.' => (),
            '\\' => {
                cur_dir = (cur_dir.1, cur_dir.0);
                cur_cell.energized = true; 
            },
            '/' => {
                cur_dir = (-cur_dir.1, -cur_dir.0);
                cur_cell.energized = true; 
            },
            '-' => {
                cur_cell.energized = true; 
                match cur_dir {
                    (1, 0) | (-1, 0) => (),
                    (0, 1) | (0, -1) => return Some((cur_pos, [(1, 0), (-1, 0)])), 
                    _ => panic!("Shouldn't happen")
                }
            },
            '|' => {
                cur_cell.energized = true; 
                match cur_dir {
                    (1, 0) | (-1, 0) => return Some((cur_pos, [(0, 1), (0, -1)])),
                    (0, 1) | (0, -1) => (), 
                    _ => panic!("Shouldn't happen")
                }
            }
            _ => panic!("Shouldn't happen")
        }
    }
    None
}

fn main() {
    let mut grid = include_str!("../../input2.txt")
        .lines()
        .map(|l| l.chars().map(|ch| GridCell::new(ch)).collect::<Vec<GridCell>>())
        .collect::<Vec<Vec<GridCell>>>();

    let mut start_queue = VecDeque::from([((0usize, 0usize), (1isize, 0isize))]);
    let mut visited = vec![];
    while !start_queue.is_empty() {
        let start = start_queue.pop_front().unwrap();
        if visited.iter().position(|x: &((usize,usize),(isize,isize))| x.0 == start.0 && x.1 == start.1).is_some() {
            continue;
        }
        let continuation = trace_beam(&mut grid, start.0, start.1);
        visited.push(start);
        if let Some(c) = continuation {
            println!("SPLIT: ({:?}) <- {:?} -> ({:?})", c.1[0], c.0, c.1[1]);
            start_queue.push_back((c.0, c.1[0])); 
            start_queue.push_back((c.0, c.1[1])); 
        }
    }

    for line in &grid {
        for cell in line {
            if cell.energized {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    let sum = grid
        .iter()
        .map(|l| l.iter().filter(|c| c.energized).count())
        .sum::<usize>();

    println!("Number of energized cells: {}", sum);
}
