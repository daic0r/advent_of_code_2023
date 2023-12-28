use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;
use core::fmt::Formatter;

#[derive(Debug, Clone)]
struct Node {
    coord: Point,
    // for debugging
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

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node({})", self.coord)
    }
}

type Map = Vec<Vec<char>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point(usize, usize);

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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
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
            ret.push(Point(neighbor.0.unwrap(), neighbor.1.unwrap()));
        }
    }
    ret
}

fn read_graph(map: &Map) -> HashMap<Point, HashSet<Edge>> {
    let mut ret = HashMap::new();
    let start = Point(map.first().unwrap().iter().position(|&c| c == '.').unwrap(), 0usize);

    // cur point, from node, edge length, previous point
    let mut the_stack: Vec<(Point, Point, usize, Option<Point>)> = vec![ (start.clone(), start, 0, None) ];
    while !the_stack.is_empty() {
        let (cur, from, edge_len, prev) = the_stack.pop().unwrap();

        let neighs = get_neighbors(&cur, prev.as_ref(), &map);
        if neighs.len() == 1 {
            the_stack.push((neighs.first().unwrap().clone(), from, edge_len + 1, Some(cur)));
        } else {
            let mut set2 = HashSet::new();
            set2.insert(Edge(from, edge_len));
            ret.entry(cur.clone()).and_modify(|v: &mut HashSet<Edge>| { 
                // let mut to_rem = None;
                // if let Some(prev_entry) = v.iter().find(|&e| e.0 == from) {
                //     if edge_len > prev_entry.1 {
                //         to_rem = Some(*prev_entry);
                //     }
                // }
                // let old_len = v.len();
                // if let Some(to_rem) = to_rem {
                //     assert!(v.remove(&to_rem));
                //     assert!(v.len() == old_len-1);
                // }
                v.insert(Edge(from, edge_len)); 
            })
                .or_insert(set2);
            for neigh in &neighs {
                the_stack.push((neigh.clone(), cur, 1, Some(cur)));
            }
        }
    }
    ret
}

#[derive(Debug, Clone)]
struct Path {
    path: Vec<Point>,
    len: usize
}

fn find_path_len(start: &Point, dest: &Point, graph: &HashMap<Point, HashSet<Edge>>) -> usize {
    let mut the_stack = vec![ Path { path: vec![*dest], len: 0 }];
    let mut memory: HashMap<Point, Path> = HashMap::new();
    while let Some(cur) = the_stack.pop() {
        let head = *cur.path.last().unwrap();
        // Start node won't have an entry
        if !graph.contains_key(&head) {
            continue;
        }
        let entry = &graph[&head];
        println!("Now at {:?}", cur);

        for edge in entry {
            // if cur.path.contains(&edge.0) {
            //     continue;
            // }
            let mut tmp = cur.clone();
            tmp.path.push(edge.0);
            tmp.len += edge.1;
            if !memory.contains_key(&edge.0) || tmp.len > memory.get(&edge.0).unwrap().len {
                if memory.contains_key(&edge.0) {
                    println!("Replacing {:?} with {:?}", memory.get(&edge.0).unwrap(), tmp);
                }
                memory.insert(edge.0, tmp.clone());
                the_stack.push(tmp);
            }
        }
        println!("-------------------");
    }
    memory[start].len
}

fn main() {
    let map = include_str!("../../input.txt")
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

    let start = Point(map.first().unwrap().iter().position(|&c| c == '.').unwrap(), 0usize);
    let dest = Point(map.last().unwrap().iter().position(|&c| c == '.').unwrap(), map.len()-1);
    let path_len = find_path_len(&start, &dest, &graph);
    println!("Path length: {}", path_len);

}
