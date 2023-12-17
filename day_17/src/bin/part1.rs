use std::collections::BinaryHeap;
use std::io::Write;
use std::ops::{Add,Sub};
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
struct Node {
    coord: Point,
    f_cost: usize,
    g_cost: usize,
    prev: Option<Point>
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.f_cost.cmp(&other.f_cost));
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.f_cost.cmp(&other.f_cost);
    }
}

impl Node {
    fn new(coord: &Point) -> Self {
        Self {
            coord: coord.clone(),
            f_cost: usize::MAX,
            g_cost: usize::MAX,
            prev: None
        }
    }

    fn g_cost(&mut self, g: usize) -> &mut Self {
        self.g_cost = g; 
        self
    }
    fn f_cost(&mut self, f: usize) -> &mut Self {
        self.f_cost = f;
        self
    }
}

type Map = Vec<Vec<char>>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point(usize, usize);

#[derive(Debug, PartialEq, Eq)]
struct Vector(isize, isize);

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            0: self.0 - other.0,
            1: self.1 - other.1
        }
    }
}
impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, other: Vector) -> Self::Output {
        Self {
            0: self.0.checked_add_signed(other.0).unwrap(),
            1: self.1.checked_add_signed(other.1).unwrap()
        }
    }
}
impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            0: self.0 + other.0,
            1: self.1 + other.1
        }
    }
}

const OFFSETS: [Vector; 4] = [ Vector(-1, 0), Vector(0, -1), Vector(1, 0), Vector(0, 1) ];

fn find_path(map: &Map) -> Option<Vec<Point>> {
    let start = Point(0usize, 0usize);
    let dest = Point(map.first().unwrap().len()-1, map.len()-1);
    let h = |coord: &Point| (std::cmp::max(coord.0, dest.0) - std::cmp::min(coord.0, dest.0)) + (std::cmp::max(coord.1, dest.1) - std::cmp::min(coord.1, dest.1));
    let nodes = map
        .iter()
        .enumerate()
        .map(|(row_idx,row)| row.iter().enumerate().map(|(col_idx,_)| {
                let coord = Point(col_idx, row_idx);
                RefCell::new(Node::new(&coord))
        }).collect::<Vec<RefCell<Node>>>())
        .fold(vec![], |mut acc,row_vec| {
            acc.extend(row_vec);
            acc
        });
    nodes.iter().enumerate().inspect(|(idx,node)| {
        let y = idx / map.first().unwrap().len();
        let x = idx % map.first().unwrap().len();
        assert_eq!(Point(x,y), node.borrow().coord);
    }).for_each(|_| {});
    {
        let mut start_node = nodes.iter().find(|n| n.borrow().coord == start).unwrap().borrow_mut();
        start_node.g_cost = 0;
        start_node.f_cost = h(&start);
        println!("Start distance: {}", start_node.f_cost);
    }

    let mut last_dir = Vector(0,0);
    let mut last_dir_cnt: usize = 0;
    let mut get_neighbors = |coord: &Point, from: Option<&Point>| {
        if let Some(from) = from {
            let vec_from_last = Vector(coord.0 as isize - from.0 as isize, coord.1 as isize - from.1 as isize);
            if vec_from_last == last_dir {
                last_dir_cnt += 1;
            } else {
                last_dir = vec_from_last;
                last_dir_cnt = 0;
            }
        }
        let mut ret = vec![];

        for offset in &OFFSETS {
            let neighbor = (coord.0.checked_add_signed(offset.0), coord.1.checked_add_signed(offset.1));
            if neighbor.0.is_none() || neighbor.1.is_none()
                || neighbor.0.unwrap() > map.first().unwrap().len()-1
                    || neighbor.1.unwrap() > map.len()-1
                    || (*offset == last_dir && last_dir_cnt == 3)
                    || (from.is_some() && from.unwrap().0 == neighbor.0.unwrap() && from.unwrap().1 == neighbor.1.unwrap())
            {
                continue;
            }
            let neighbor = Point(neighbor.0.unwrap(),  neighbor.1.unwrap());
            ret.push(neighbor);
        }
        ret
    };


    let mut open_set = BinaryHeap::from([ nodes.iter().find(|n| n.borrow().coord == start).unwrap() ]);
    while !open_set.is_empty() {
        let current = open_set.pop().unwrap(); 
        // Arrived at destination
        if current.borrow().coord == dest {
            let mut ret = vec![ dest ];
            let mut coord_cur = current.borrow().prev.clone();
            while let Some(cur) = coord_cur {
                ret.push(cur.clone());
                let cur_node = nodes.iter().find(|n| n.borrow().coord == cur).unwrap();
                coord_cur = cur_node.borrow().prev.clone();
            }
            ret.reverse();
            return Some(ret);
        }

        // Process neighbors
        for neighbor in get_neighbors(&current.borrow().coord, current.borrow().prev.as_ref()) {
            if let Some(prev) = &current.borrow().prev {
                assert!(*prev != neighbor);
            }
            let g_cost = current.borrow().g_cost + map[neighbor.1][neighbor.0].to_digit(10).unwrap() as usize;
            let neighbor_node = nodes.iter().find(|n| n.borrow().coord == neighbor).unwrap();
            if g_cost < neighbor_node.borrow().g_cost {
                neighbor_node.borrow_mut().prev = Some(current.borrow().coord.clone());
                neighbor_node.borrow_mut().g_cost = g_cost;
                neighbor_node.borrow_mut().f_cost = g_cost + h(&neighbor);
                if !open_set.iter().any(|n| n.borrow().coord == neighbor) {
                    open_set.push(neighbor_node);
                }
            }
        }
    }
    None
}

fn main() {
    let map = include_str!("../../input2.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let path = find_path(&map);

    if path.is_none() {
        println!("No path found");
    } else if let Some(path) = path {
        for p in &path {
            println!("{:?}", p);
        }
        let heat_loss = path.iter().map(|p| {
            if p.1 != 0 || p.0 != 0 {
                map[p.1][p.0].to_digit(10).unwrap() as usize
            } else {
                0
            }
        }).sum::<usize>();
        println!("Length of path: {}", path.len());
        println!("Heat loss = {}", heat_loss);
    }

}
