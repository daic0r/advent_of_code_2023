use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;
use core::fmt::Formatter;

#[derive(Debug, Clone)]
struct Node {
    coord: Point,
    // for debugging
    symbol: Option<char>,
    prev: Option<Point>
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.coord == other.coord && self.prev == other.prev
    }
}
impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.coord.hash(state);
    }
}

impl Eq for Node {}

impl Node {
    fn new(coord: &Point) -> Self {
        Self {
            coord: coord.clone(),
            symbol: None,
            prev: None
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node({})", self.coord)
    }
}

type Map = Vec<Vec<char>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point(usize, usize);

impl Point {
    fn sub(&self, other: &Self) -> Vector {
        Vector(self.0 as isize - other.0 as isize, self.1 as isize - other.1 as isize)
    }
}
impl std::ops::Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector(self.0 as isize - rhs.0 as isize, self.1 as isize - rhs.1 as isize)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Vector(isize, isize);

#[derive(Debug, Hash, PartialEq, Eq)]
struct Edge(Point, usize);

const OFFSETS: [Vector; 4] = [ Vector(1, 0), Vector(-1, 0), Vector(0, 1), Vector(0, -1)];

fn get_neighbors(my_node: &Point, prev: Option<&Point>, map: &Map) -> Vec<Point> {
    let mut ret = vec![];

    let offsets = match map[my_node.1][my_node.0] {
        '>' => &[ Vector(1, 0) ],
        '<' => &[ Vector(-1, 0) ],
        '^' => &[ Vector(0, -1) ],
        'v' => &[ Vector(0, 1) ],
        '.' => &OFFSETS[..],
        _ => &[]
    };

    for offset in offsets {
        let neighbor = (my_node.0.checked_add_signed(offset.0), my_node.1.checked_add_signed(offset.1));
        if neighbor.0.is_none() || neighbor.1.is_none()
            || neighbor.0.unwrap() > map.first().unwrap().len()-1
                || neighbor.1.unwrap() > map.len()-1
                || map[neighbor.1.unwrap()][neighbor.0.unwrap()] == '#'
                || prev.is_some() && *prev.unwrap() == Point(neighbor.0.unwrap(), neighbor.1.unwrap())
        {
            continue;
        }
        let next_point = Point(neighbor.0.unwrap(), neighbor.1.unwrap());
        let dir = next_point - *my_node;
        let legal_travel = match map[next_point.1][next_point.0] {
            '>' if dir== Vector(1, 0) => true,
            '<' if dir== Vector(-1, 0) => true,
            '^' if dir== Vector(0, -1) => true,
            'v' if dir== Vector(0, 1) => true,
            '.' => true,
            _ => false
        };
        if legal_travel {
        if my_node == &Point(5, 13) {
            println!("Considering neighbor: {:?} with legal_travel: {}", neighbor, legal_travel);
        }
            ret.push(Point(neighbor.0.unwrap(), neighbor.1.unwrap()));
        }
    }
    ret
}

// fn find_path(map: &Map) -> Option<Vec<Node>> {
//     let mut prevoius = HashSet::new();
//
//     let start = Point(map.first().unwrap().iter().position(|&c| c == '.').unwrap(), 0usize);
//     let dest = Point(map.last().unwrap().iter().position(|&c| c == '.').unwrap(), map.len()-1);
//     let h = |coord: &Point| (std::cmp::max(coord.0, dest.0) - std::cmp::min(coord.0, dest.0)) + (std::cmp::max(coord.1, dest.1) - std::cmp::min(coord.1, dest.1));
//
//     println!("Start: {:?}", start);
//     println!("Dest: {:?}", dest);
//
//     let mut g_costs: HashMap<Node, usize> = HashMap::new();
//     let mut f_costs: HashMap<Node, usize> = HashMap::new();
//     let mut came_from: HashMap<Node, Node> = HashMap::new();
//
//     g_costs.insert(Node::new(&start), 0);
//     f_costs.insert(Node::new(&start),  h(&start));
//
//     let mut open_set = PriorityQueue::new();
//     open_set.push(Node::new(&start), h(&start));
//
//     let mut ret = vec![];
//
//     while !open_set.is_empty() {
//         let current = open_set.pop().unwrap().0;
//         if cfg!(feature="debug_output") {
//             println!("({},{})", current.coord.0, current.coord.1);
//         }
//         // Arrived at destination
//         if current.coord == dest {
//             let mut tmp_ret = vec![ current.clone() ];
//             let mut cur = &current;
//             while let Some(tmp) = came_from.get(cur) {
//                 if tmp.coord != start {
//                     tmp_ret.push(tmp.clone());
//                 }
//                 cur = tmp;
//             }
//             if tmp_ret.len() > ret.len() {
//                 ret = tmp_ret;
//             }
//         }
//
//         // Process neighbors
//         for mut neighbor in get_neighbors(&current, &map, &mut prevoius) {
//             let tentative_g_cost = *g_costs.get(&current).unwrap();
//             if cfg!(feature="debug_output") {
//                 print!("g: {}", tentative_g_cost);
//             }
//
//             if tentative_g_cost < *g_costs.get(&neighbor).unwrap_or(&usize::MAX) {
//             //if !g_costs.contains_key(&neighbor) {
//                 if current.coord == Point(11, 4) {
//                     println!("Considering neighbor: {:?}", neighbor);
//                 }
//                 let dir_to_neighbor = neighbor.coord.sub(&current.coord);
//                 neighbor.symbol = match dir_to_neighbor {
//                     Vector(-1, 0) => Some('<'),
//                     Vector(0, -1) => Some('^'),
//                     Vector(1, 0) => Some('>'),
//                     Vector(0, 1) => Some('v'),
//                     _ => panic!("Shouldn't happen")
//                 };
//                 g_costs.insert(neighbor.clone(), tentative_g_cost);
//                 f_costs.insert(neighbor.clone(), tentative_g_cost + h(&neighbor.coord));
//                 came_from.insert(neighbor.clone(), current.clone());
//                 //if cfg!(feature="debug_output") {
//                 if current.coord == Point(11, 3) {
//                     println!("f of {}: {}", neighbor, f_costs[&neighbor]);
//                     //println!("CHOSE: {:?} with edge weight {}", neighbor, map[neighbor.1][neighbor.0].to_digit(10).unwrap());
//                 }
//                 //}
//                 //if !open_set.iter().any(|p| *p.0 == neighbor) {
//                     open_set.push(neighbor.clone(), f_costs[&neighbor]);
//                     prevoius.insert(neighbor.coord);
//                 //}
//             } else {
//                 if cfg!(feature="debug_output") {
//                     println!(" >= {}, ", g_costs[&neighbor]);
//                     //println!("CHOSE: {:?} with edge weight {}", neighbor, map[neighbor.1][neighbor.0].to_digit(10).unwrap());
//                 }
//             }
//         }
//     }
//     Some(ret)
// }

fn read_graph(map: &Map) -> HashMap<Point, HashSet<Edge>> {
    let mut ret = HashMap::new();
    let start = Point(map.first().unwrap().iter().position(|&c| c == '.').unwrap(), 0usize);
    let dest = Point(map.last().unwrap().iter().position(|&c| c == '.').unwrap(), map.len()-1);

    // cur point, from node, edge length, previous point
    let mut the_stack: Vec<(Point, Point, usize, Option<Point>)> = vec![ (start.clone(), start, 0, None) ];
    while !the_stack.is_empty() {
        let (cur, from, edge_len, prev) = the_stack.pop().unwrap();

        let neighs = get_neighbors(&cur, prev.as_ref(), &map);
        if cur == Point(5, 13) {
            println!("Considering neighbors: {:?}", neighs);
        }
        if neighs.len() == 1 {
            the_stack.push((neighs.first().unwrap().clone(), from, edge_len + 1, Some(cur)));
        } else {
            let mut set1 = HashSet::new();
            set1.insert(Edge(cur, edge_len));
            ret.entry(from).and_modify(|v: &mut HashSet<Edge>| { v.insert(Edge(cur.clone(), edge_len)); })
                .or_insert(set1);
            let mut set2 = HashSet::new();
            set2.insert(Edge(from, edge_len));
            ret.entry(cur.clone()).and_modify(|v: &mut HashSet<Edge>| { v.insert(Edge(from, edge_len)); })
                .or_insert(set2);
            for neigh in &neighs {
                the_stack.push((neigh.clone(), cur, 1, Some(cur)));
            }
        }
    }
    ret
}

fn find_path_len(start: &Point, dest: &Point, map: &HashMap<Point, Vec<(Point, usize)>>) -> usize {
    let mut cur = start;
    let len = 0usize;
    while cur != dest {
        let entry = &map[cur];

    }
    len
}

fn main() {
    let map = include_str!("../../input2.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let graph = read_graph(&map);
    //assert!(graph[&Point(3,5)].len() == 3);
    for (node, edges) in &graph {
        print!("{} -> ", node); 
        for edge in edges {
            print!("({}, {}), ", edge.0, edge.1);
        }
        println!();
    }

    // let path = find_path(&map);
    //
    // if path.is_none() {
    //     println!("No path found");
    // } else if let Some(path) = path {
    //     for p in &path {
    //         println!("{:?}", p.coord);
    //     }
    //     for (row_idx,row) in map.iter().enumerate() {
    //         for (col_idx,col) in row.iter().enumerate() {
    //             if let Some(node) = path.iter().find(|p| p.coord == Point(col_idx,row_idx)) {
    //                 //print!("{}", node.symbol.unwrap());
    //                 print!("O");
    //             } else {
    //                 print!("{}", map[row_idx][col_idx]);
    //             }
    //         }
    //         println!();
    //     }
    //     println!("Length of path: {}", path.len());
    // }

}
