use std::fmt::Display;
use std::cell::RefCell;

/*
enum Offset {
    Zero,
    Pos(usize),
    Neg(usize)
}
const NEIGHBORS: [(Offset, Offset); 8] = [
    (Offset::Neg(1), Offset::Neg(1)),
    (Offset::Zero,  Offset::Neg(1)),
    (Offset::Pos(1),  Offset::Neg(1)),
    (Offset::Neg(1),  Offset::Zero),
    (Offset::Pos(1),   Offset::Zero),
    (Offset::Neg(1),  Offset::Pos(1)),
    (Offset::Zero,   Offset::Pos(1)),
    (Offset::Pos(1),   Offset::Pos(1))
];
*/
const NEIGHBORS: [(isize, isize); 4] = [
    (0,  -1),
    (-1,  0),
    (1,   0),
    (0,   1),
];

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum MapTile {
    LeftToRight,
    TopToBottom,
    TopToRight,
    TopToLeft,
    BottomToRight,
    BottomToLeft,
    Ground,
    Start,
    MainLoop
}

impl MapTile {
    fn can_connect(&self, rhs: MapTile, offs: (isize,isize)) -> bool {
        use MapTile::*;

        match self {
            LeftToRight => match offs {
                (-1, 0) => match rhs {
                    Start | LeftToRight | TopToRight | BottomToRight => true,
                    _ => false
                },
                (1, 0) => match rhs {
                    Start | LeftToRight | TopToLeft | BottomToLeft => true,
                    _ => false
                },
                _ => false
            },
            TopToBottom => match offs {
                (0, -1) => match rhs {
                    Start | TopToBottom | BottomToRight | BottomToLeft => true,
                    _ => false
                },
                (0, 1) => match rhs {
                    Start | TopToBottom | TopToRight | TopToLeft => true,
                    _ => false
                },
                _ => false
            },
            TopToRight => match offs {
                (1, 0) => match rhs {
                    Start | LeftToRight | BottomToLeft | TopToLeft => true,
                    _ => false
                },
                (0, -1) => match rhs {
                    Start | TopToBottom | BottomToLeft | BottomToRight => true,
                    _ => false
                },
                _ => false
            },
            TopToLeft => match offs {
                (-1, 0) => match rhs {
                    Start | LeftToRight | BottomToRight | TopToRight => true,
                    _ => false
                },
                (0, -1) => match rhs {
                    Start | TopToBottom | BottomToLeft | BottomToRight => true,
                    _ => false
                },
                _ => false
            },
            BottomToRight => match offs {
                (1, 0) => match rhs {
                    Start | LeftToRight | TopToLeft | BottomToLeft => true,
                    _ => false
                },
                (0, 1) => match rhs {
                    Start | TopToBottom | TopToRight | TopToLeft => true,
                    _ => false
                },
                _ => false
            },
            BottomToLeft => match offs {
                (-1, 0) => match rhs {
                    Start | LeftToRight | TopToRight | BottomToRight => true,
                    _ => false
                },
                (0, 1) => match rhs {
                    Start | TopToBottom | TopToLeft | TopToRight => true,
                    _ => false
                },
                _ => false
            },
            Ground => false,
            Start => match offs {
                (-1, 0) => match rhs {
                    LeftToRight | TopToRight | BottomToRight => true,
                    _ => false
                },
                (1, 0) => match rhs {
                    LeftToRight | TopToLeft | BottomToLeft => true,
                    _ => false
                },
                (0, 1) => match rhs {
                    TopToBottom | TopToRight | TopToLeft => true,
                    _ => false
                },
                (0, -1) => match rhs {
                    TopToBottom | BottomToRight | BottomToLeft => true,
                    _ => false
                },
                _ => true
            },
            MainLoop => false
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Node {
    coord: (usize, usize),
    tile: MapTile,
    dist_from_start: usize,
    offset_from_prev: (isize, isize)
}

#[derive(Debug)]
struct Map {
    data: Vec<Vec<RefCell<Node>>>,
}

impl Map {
    fn new(d: Vec<Vec<char>>) -> Self {
        let mut ret = Map {
            data: d
                .iter()
                .enumerate()
                .map(|(idx_row,row)| row.iter().enumerate().map(|(idx_col,col)| {
                    use MapTile::*;
                    RefCell::new(Node {
                        coord: (idx_col, idx_row),
                        tile: match col {
                            'S' => Start,
                            '.' => Ground,
                            '|' => TopToBottom,
                            '-' => LeftToRight,
                            'F' => BottomToRight,
                            'J' => TopToLeft,
                            '7' => BottomToLeft,
                            'L' => TopToRight,
                            _ => panic!("Invalid tile")
                        },
                        dist_from_start: 0,
                        offset_from_prev: (0, 0)
                    })
                }).collect::<Vec<RefCell<Node>>>())
            .collect(),
        };
        ret
    }

    fn mark_main_loop(&self) -> &RefCell<Node> {
        let coord_start = self.data
            .iter()
            .enumerate()
            .fold((0,0), |acc,l| {
                let start_col = l.1
                    .iter()
                    .enumerate()
                    .fold(None, |acc,ch|
                        match ch.1.borrow_mut().tile {
                            MapTile::Start => Some(ch.0),
                            _ => acc
                        });
                match start_col {
                    Some(col) => (col, l.0),
                    None => acc
                }
            });
        println!("Start at: {:?}", coord_start);

        let mut start: &RefCell<Node> = self.get_tile(coord_start);
        let adj_start = self.get_connected_neighbors(coord_start);
        // ((Coords), dist_to_start)
        let mut path_1_last = start;
        let mut path_2_last = start;

        let mut start = self.get_tile(coord_start);
        start.borrow_mut().tile = MapTile::MainLoop;

        let mut path_1_cur = adj_start.get(0).unwrap().1;
        let mut path_2_cur = adj_start.get(1).unwrap().1;
        while path_1_cur.borrow().coord != path_2_cur.borrow().coord {
            let tmp1 = self.get_connected_neighbors(path_1_cur.borrow().coord);
            let tmp2 = self.get_connected_neighbors(path_2_cur.borrow().coord);

            let neigh_1 = tmp1.iter()
                .find(|node| node.1.borrow().coord != path_1_last.borrow().coord)
                .unwrap();
            let neigh_2 = tmp2.iter()
                .find(|node| node.1.borrow().coord != path_2_last.borrow().coord)
                .unwrap();

            let cur_1_coord = path_1_cur.borrow().coord;
            let cur_2_coord = path_2_cur.borrow().coord;
            path_1_cur.borrow_mut().offset_from_prev = ((cur_1_coord.0 as isize - path_1_last.borrow().coord.0 as isize), (cur_1_coord.1 as isize - path_1_last.borrow().coord.1 as isize));
            path_2_cur.borrow_mut().offset_from_prev = ((cur_2_coord.0 as isize - path_2_last.borrow().coord.0 as isize), (cur_2_coord.1 as isize - path_2_last.borrow().coord.1 as isize));
            println!("Path 1: {:?} -> {:?}", path_1_last, path_1_cur);
            println!("Path 2: {:?} -> {:?}", path_2_last, path_2_cur);

            path_1_last = path_1_cur;
            path_1_last.borrow_mut().dist_from_start += 1;
            path_2_last = path_2_cur;
            path_2_last.borrow_mut().dist_from_start += 1;

            path_1_last.borrow_mut().tile = MapTile::MainLoop;
            path_2_last.borrow_mut().tile = MapTile::MainLoop;

            path_1_cur = neigh_1.1;
            path_2_cur = neigh_2.1;
            assert!(neigh_1.0 != (0,0));
            assert!(neigh_2.0 != (0,0));
        }
            let cur_1_coord = path_1_cur.borrow().coord;
            let cur_2_coord = path_2_cur.borrow().coord;
            path_1_cur.borrow_mut().offset_from_prev = ((cur_1_coord.0 as isize - path_1_last.borrow().coord.0 as isize), (cur_1_coord.1 as isize - path_1_last.borrow().coord.1 as isize));
            path_2_cur.borrow_mut().offset_from_prev = ((cur_2_coord.0 as isize - path_2_last.borrow().coord.0 as isize), (cur_2_coord.1 as isize - path_2_last.borrow().coord.1 as isize));
            println!("Path 1: {:?} -> {:?}", path_1_last, path_1_cur);
        path_1_last = path_1_cur;
        path_1_last.borrow_mut().dist_from_start += 1;
        path_2_last = path_2_cur;
        path_2_last.borrow_mut().dist_from_start += 1;

        path_1_last.borrow_mut().tile = MapTile::MainLoop;
        path_2_last.borrow_mut().tile = MapTile::MainLoop;

        println!("Arrived at: {:?}", path_1_last);
        println!("Arrived at: {:?}", path_2_last);

        path_1_last
    }

    fn calc_tiles_inside_loop(&self) -> usize {
       self.data 
           .iter()
           .enumerate()
           .fold(0, |acc,(row_num,l)| {
               let cnt = l
               .iter()
               .enumerate()
               .fold(0, |acc,(idx,node)| {
                   if node.borrow().tile == MapTile::MainLoop {
                       return acc;
                   }

                   if l.iter().enumerate().take_while(|(i,node)| *i < idx && node.borrow().tile != MapTile::MainLoop).count() == idx {
                       return acc;
                   }
                   println!("We at ({},{})", idx, row_num);
                   let cnt_intersec = ((l[idx+1..]
                    .iter()
                    .filter(|node| node.borrow().tile == MapTile::MainLoop)
                    .fold(0, |acc,node| acc + match node.borrow().offset_from_prev {
                        (-1, _) | (_, -1) => 1,
                        (1, _) | (_, 1) => -1,
                        (0, 0) => 0,
                        x => panic!("Oughtn't happen ({},{}): {:?}", node.borrow().coord.0, node.borrow().coord.1, x)
                     }) != 0) as usize);
                    acc + cnt_intersec
               });
               println!("Row: {} tiles inside", cnt);
               acc + cnt
           })
    }

    fn get_tile(&self, coord: (usize, usize)) -> &RefCell<Node> {
        self.data.get(coord.1).unwrap().get(coord.0).unwrap()
    }

    fn get_connected_neighbors(&self, coord: (usize, usize)) -> Vec<((isize,isize), &RefCell<Node>)> {
        let this_tile = self.get_tile(coord);
        let mut ret = vec![];
        for neighbor in &NEIGHBORS {
            let coord = (coord.0.checked_add_signed(neighbor.0), coord.1.checked_add_signed(neighbor.1));
            if coord.0.is_none() || coord.1.is_none() {
                continue;
            }
            if coord.0.unwrap() > self.data.first().unwrap().len()-1 || coord.1.unwrap() > self.data.len()-1 {
                continue;
            }
            let neighbor_tile = self.get_tile((coord.0.unwrap(), coord.1.unwrap()));
            assert!(*neighbor != (0, 0));
            if this_tile.borrow().tile.can_connect(neighbor_tile.borrow().tile, *neighbor) {
                ret.push((*neighbor, neighbor_tile));
            }
        }
        assert!(ret.len() <= 2);
        ret
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for col in row {
                use MapTile::*;
                match col.borrow().tile {
                    LeftToRight => write!(f, "-")?,
                    TopToBottom => write!(f, "|")?,
                    TopToRight => write!(f, "L")?,
                    TopToLeft => write!(f, "J")?,
                    BottomToRight => write!(f, "F")?,
                    BottomToLeft => write!(f, "7")?,
                    Ground => write!(f, ".")?,
                    Start => write!(f, "S")?,
                    MainLoop => write!(f, "X")?
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    let data = include_str!("../../input2_part2.txt")
        .split('\n')
        .filter_map(
            |l| match l.is_empty() { 
                false => Some(l.to_owned()), 
                true => None
            })
        .map(|l| l.chars()
            .inspect(|c| print!("{}", c))
            .collect::<Vec<char>>())
        .inspect(|l| println!())
        .collect::<Vec<Vec<char>>>();


        let mut map = Map::new(data.clone());

        println!("\n{}", map);

        let end = map.mark_main_loop();
        println!("\n{}", map);
        
        let num_inside = map.calc_tiles_inside_loop();
        println!("Tiles inside: {}", num_inside);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_map() {
        let data = include_str!("../../input.txt")
            .split('\n')
            .filter_map(
                |l| match l.is_empty() { 
                    false => Some(l.to_owned()), 
                    true => None
                })
        .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let map = Map::new(data.clone());
        assert_eq!(data, map.to_string()
            .split('\n')
            .filter_map(
                |l| match l.is_empty() { 
                    false => Some(l.to_owned()), 
                    true => None
                })
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>());
    }
}
