use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use core::fmt::Formatter;

#[derive(Debug, Clone)]
struct Node {
    coord: Point,
    prev: (Option<Vector>, usize),
    // for debugging
    symbol: Option<char>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.coord == other.coord && self.prev == other.prev
    }
}
impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.coord.hash(state);
        self.prev.hash(state);
    }
}

impl Eq for Node {}

impl Node {
    fn new(coord: &Point) -> Self {
        Self {
            coord: coord.clone(),
            symbol: None,
            prev: (None, 0)
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node({})", self.coord)
    }
}

type Map = Vec<Vec<char>>;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Point(usize, usize);

impl Point {
    fn sub(&self, other: &Self) -> Vector {
        Vector(self.0 as isize - other.0 as isize, self.1 as isize - other.1 as isize)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Vector(isize, isize);

const OFFSETS: [Vector; 4] = [ Vector(1, 0), Vector(-1, 0), Vector(0, 1), Vector(0, -1)];

fn get_neighbors(my_node: &Node, map: &Map) -> Vec<Node> {
    let mut ret = vec![];

    let forbidden_dir: Option<Vector> = if my_node.prev.1 == 3 {
        my_node.prev.0
    } else {
        None
    };
    if cfg!(feature="debug_output") {
        if (forbidden_dir.is_some()) {
            println!("Forbidden dir: {:?}", forbidden_dir);
        }
    }

    for offset in &OFFSETS {
        let neighbor = (my_node.coord.0.checked_add_signed(offset.0), my_node.coord.1.checked_add_signed(offset.1));
        if neighbor.0.is_none() || neighbor.1.is_none()
            || neighbor.0.unwrap() > map.first().unwrap().len()-1
                || neighbor.1.unwrap() > map.len()-1
                || (my_node.prev.0.is_some() && my_node.prev.0.unwrap().0 == -offset.0 && my_node.prev.0.unwrap().1 == -offset.1)
                || (forbidden_dir.is_some() && *offset == forbidden_dir.unwrap())
        {
            continue;
        }
        let mut neighbor_node = my_node.clone();
        neighbor_node.coord = Point(neighbor.0.unwrap(),  neighbor.1.unwrap());
        if let Some(prev) = &my_node.prev.0 {
            if *prev != *offset {
                neighbor_node.prev.0 = Some(*offset);
                neighbor_node.prev.1 = 1;
            } else {
                neighbor_node.prev.1 += 1;
            }
        } else {
            neighbor_node.prev.0 = Some(*offset);
            neighbor_node.prev.1 = 1;
        }
        ret.push(neighbor_node);
    }
    ret
}

fn heat_loss(p: &Point, map: &Map) -> usize {
    map[p.1][p.0].to_digit(10).unwrap() as usize
}

fn find_path(map: &Map) -> Option<Vec<Node>> {
    assert!(heat_loss(&Point(0,0), &map) == 2);
    assert!(heat_loss(&Point(3,1), &map) == 5);
    assert!(heat_loss(&Point(5,12), &map) == 7);

    let start = Point(0usize, 0usize);
    let dest = Point(map.first().unwrap().len()-1, map.len()-1);
    let h = |coord: &Point| (std::cmp::max(coord.0, dest.0) - std::cmp::min(coord.0, dest.0)) + (std::cmp::max(coord.1, dest.1) - std::cmp::min(coord.1, dest.1));

    let mut g_costs: HashMap<Node, usize> = HashMap::new();
    let mut f_costs: HashMap<Node, usize> = HashMap::new();
    let mut came_from: HashMap<Node, Node> = HashMap::new();

    g_costs.insert(Node::new(&start), 0);
    f_costs.insert(Node::new(&start),  h(&start));

    let mut open_set = PriorityQueue::new();
    open_set.push(Node::new(&start), Reverse(h(&start)));

    while !open_set.is_empty() {
        let current = open_set.pop().unwrap().0;
        if cfg!(feature="debug_output") {
            println!("({},{})", current.coord.0, current.coord.1);
        }
        // Arrived at destination
        if current.coord == dest {
            let mut ret = vec![ current.clone() ];
            let mut cur = &current;
            while let Some(tmp) = came_from.get(cur) {
                if tmp.coord != start {
                    ret.push(tmp.clone());
                }
                cur = tmp;
            }
            return Some(ret);
        }

        // Process neighbors
        for mut neighbor in get_neighbors(&current, &map) {
            if cfg!(feature="debug_output") {
                println!("\t: {}, {:?} ", neighbor.coord, neighbor.prev);
            }
            let tentative_g_cost = g_costs.get(&current).unwrap() + heat_loss(&neighbor.coord, &map);
            if cfg!(feature="debug_output") {
                print!("g: {}", tentative_g_cost);
            }

            if tentative_g_cost < *g_costs.get(&neighbor).unwrap_or(&usize::MAX) {
                if cfg!(feature="debug_output") {
                    print!(" < {}, ", *g_costs.get(&neighbor).unwrap_or(&usize::MAX));
                    //println!("CHOSE: {:?} with edge weight {}", neighbor, map[neighbor.1][neighbor.0].to_digit(10).unwrap());
                }

                let dir_to_neighbor = neighbor.coord.sub(&current.coord);
                neighbor.symbol = match dir_to_neighbor {
                    Vector(-1, 0) => Some('<'),
                    Vector(0, -1) => Some('^'),
                    Vector(1, 0) => Some('>'),
                    Vector(0, 1) => Some('v'),
                    _ => panic!("Shouldn't happen")
                };
                g_costs.insert(neighbor.clone(), tentative_g_cost);
                f_costs.insert(neighbor.clone(), tentative_g_cost + h(&neighbor.coord));
                came_from.insert(neighbor.clone(), current.clone());
                if cfg!(feature="debug_output") {
                    println!("f: {}", f_costs[&neighbor]);
                    //println!("CHOSE: {:?} with edge weight {}", neighbor, map[neighbor.1][neighbor.0].to_digit(10).unwrap());
                }
                if !open_set.iter().any(|p| *p.0 == neighbor) {
                    open_set.push(neighbor.clone(), Reverse(f_costs[&neighbor]));
                }
            } else {
                if cfg!(feature="debug_output") {
                    println!(" >= {}, ", g_costs[&neighbor]);
                    //println!("CHOSE: {:?} with edge weight {}", neighbor, map[neighbor.1][neighbor.0].to_digit(10).unwrap());
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
            println!("{:?}", p.coord);
        }
        for (row_idx,row) in map.iter().enumerate() {
            for (col_idx,col) in row.iter().enumerate() {
                if let Some(node) = path.iter().find(|p| p.coord == Point(col_idx,row_idx)) {
                    print!("{}", node.symbol.unwrap());
                } else {
                    print!("{}", heat_loss(&Point(col_idx, row_idx), &map));
                }
            }
            println!();
        }
        let heat_loss = path.iter().map(|p| heat_loss(&p.coord, &map)).sum::<usize>();
        println!("Length of path: {}", path.len());
        println!("Heat loss = {}", heat_loss);
    }

}
