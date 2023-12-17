use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    coord: Point,
    f_cost: usize,
    g_cost: usize,
    prev: Option<Point>
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

const OFFSETS: [Vector; 4] = [ Vector(-1, 0), Vector(0, -1), Vector(1, 0), Vector(0, 1) ];

fn find_path(map: &Map) -> Option<Vec<Point>> {
    let start = Point(0usize, 0usize);
    let dest = Point(map.first().unwrap().len()-1, map.len()-1);
    let h = |coord: &Point| (std::cmp::max(coord.0, dest.0) - std::cmp::min(coord.0, dest.0)) + (std::cmp::max(coord.1, dest.1) - std::cmp::min(coord.1, dest.1));
    let mut nodes = map
        .iter()
        .enumerate()
        .map(|(row_idx,row)| row.iter().enumerate().map(|(col_idx,_)| {
                let coord = Point(col_idx, row_idx);
                Node::new(&coord)
        }).collect::<Vec<Node>>())
        .fold(vec![], |mut acc,row_vec| {
            acc.extend(row_vec);
            acc
        });
    nodes.iter().enumerate().inspect(|(idx,node)| {
        let y = idx / map.first().unwrap().len();
        let x = idx % map.first().unwrap().len();
        assert_eq!(Point(x,y), node.coord);
    }).for_each(|_| {});
    {
        let start_node = nodes.iter_mut().find(|n| n.coord == start).unwrap();
        start_node.g_cost = 0;
        start_node.f_cost = h(&start);
        println!("Start distance: {}", start_node.f_cost);
    }

    let get_neighbors = |my_node: &Node, nodes: &Vec<Node>| {
        let mut ret = vec![];

        for offset in &OFFSETS {
            let neighbor = (my_node.coord.0.checked_add_signed(offset.0), my_node.coord.1.checked_add_signed(offset.1));
            if neighbor.0.is_none() || neighbor.1.is_none()
                || neighbor.0.unwrap() > map.first().unwrap().len()-1
                    || neighbor.1.unwrap() > map.len()-1
                    || (my_node.prev.is_some() && my_node.prev.as_ref().unwrap().0 == neighbor.0.unwrap() && my_node.prev.as_ref().unwrap().1 == neighbor.1.unwrap())
            {
                continue;
            }
            let neighbor = Point(neighbor.0.unwrap(),  neighbor.1.unwrap());
            ret.push(neighbor);
        }
        ret
    };
    let check_forbidden_dir = |my_node: &Node, nodes: &Vec<Node>| {
        let mut cur = my_node;
        let mut last_dir = Vector(0, 0);
        let mut last_dir_cnt = 0usize;
        let mut steps = 0usize;
        while let Some(from) = &cur.prev {
            if steps == 3 {
                break;
            }
            let vec_from_last = Vector(cur.coord.0 as isize - from.0 as isize, cur.coord.1 as isize - from.1 as isize);
            if vec_from_last == last_dir {
                last_dir_cnt += 1;
            } else {
                last_dir = vec_from_last;
                last_dir_cnt = 0;
            }
            cur = nodes.iter().find(|n| n.coord == *cur.prev.as_ref().unwrap()).unwrap();
            steps += 1;
        }
        if last_dir_cnt == 3 {
            Some(last_dir)
        } else {
            None
        }
    };

    let mut f_costs = BTreeMap::new();
    f_costs.insert(h(&start), start);

    while !f_costs.is_empty() {
        let f_costs_first = f_costs.pop_first().unwrap();
        let current = nodes
            .iter()
            .find(|n| { 
                n.coord == f_costs_first.1
            })
            .unwrap()
            .clone();

        // Arrived at destination
        if current.coord == dest {
            let mut ret = vec![ dest ];
            let mut coord_cur = current.prev.clone();
            while let Some(cur) = &coord_cur {
                ret.push(cur.clone());
                let cur_node = nodes.iter().find(|n| n.coord == *cur).unwrap();
                coord_cur = cur_node.prev.clone();
            }
            ret.reverse();
            return Some(ret);
        }

        let forbidden = check_forbidden_dir(&current, &nodes);
        // Process neighbors
        for neighbor in get_neighbors(&current, &nodes) {
            if let Some(prev) = &current.prev {
                assert!(*prev != neighbor);
            }
            if let Some(forbidden) = &forbidden {
                if Vector(neighbor.0 as isize - current.coord.0 as isize, neighbor.1 as isize - current.coord.1 as isize)
                    == *forbidden
                {
                    continue;
                }
            }
            if cfg!(feature="debug_output") {
                //println!("CHECK: {:?}->{:?}", current, neighbor);
            }
            let g_cost = current.g_cost + map[neighbor.1][neighbor.0].to_digit(10).unwrap() as usize;
            let neighbor_node = nodes.iter_mut().find(|n| n.coord == neighbor).unwrap();
            if g_cost < neighbor_node.g_cost {
                if cfg!(feature="debug_output") {
                    //println!("CHOSE: {:?} with edge weight {}", neighbor, map[neighbor.1][neighbor.0].to_digit(10).unwrap());
                }
                neighbor_node.prev = Some(current.coord.clone());
                neighbor_node.g_cost = g_cost;
                neighbor_node.f_cost = g_cost + h(&neighbor);
                if !f_costs.iter().any(|kvp| *kvp.1 == neighbor) {
                    f_costs.insert(neighbor_node.f_cost, neighbor);
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
        for (row_idx,row) in map.iter().enumerate() {
            for (col_idx,col) in row.iter().enumerate() {
                if path.iter().any(|p| *p == Point(col_idx,row_idx)) {
                    print!("X");
                } else {
                    print!("{}", map[row_idx][col_idx]);
                }
            }
            println!();
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
