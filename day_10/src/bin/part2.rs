use std::fmt::Display;
use std::collections::LinkedList;

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
}

#[derive(Debug)]
struct Map {
    data: Vec<Vec<MapTile>>,
}

impl Map {
    fn new(d: Vec<Vec<char>>) -> Self {
        let mut ret = Map {
            data: d
                .iter()
                .map(|row| row.iter().map(|col| {
                    use MapTile::*;
                    match col {
                        'S' => Start,
                        '.' => Ground,
                        '|' => TopToBottom,
                        '-' => LeftToRight,
                        'F' => BottomToRight,
                        'J' => TopToLeft,
                        '7' => BottomToLeft,
                        'L' => TopToRight,
                        _ => panic!("Invalid tile")
                    }
                }).collect::<Vec<MapTile>>())
            .collect(),
        };
        ret
    }

    fn mark_main_loop(&mut self) -> Node {
        let coord_start = self.data
            .iter()
            .enumerate()
            .fold((0,0), |acc,l| {
                let start_col = l.1
                    .iter()
                    .enumerate()
                    .fold(None, |acc,ch|
                        match ch.1 {
                            MapTile::Start => Some(ch.0),
                            _ => acc
                        });
                match start_col {
                    Some(col) => (col, l.0),
                    None => acc
                }
            });
        println!("Start at: {:?}", coord_start);

        let start = self.data.get(coord_start.1).unwrap().get(coord_start.0).unwrap();
        let adj_start = self.get_connected_neighbors(coord_start.0, coord_start.1);
        // ((Coords), dist_to_start)
        let mut path_1_last = Node {
            dist_from_start: 0,
            coord: coord_start,
            tile: *start
        };
        let mut path_2_last = Node {
            dist_from_start: 0,
            coord: coord_start,
            tile: *start
        };
        let mut start = self.get_tile_mut(coord_start.0, coord_start.1);
        *start = MapTile::MainLoop;
        let mut path_1_cur = *adj_start.get(0).unwrap();
        let mut path_2_cur = *adj_start.get(1).unwrap();
        while path_1_cur.0 != path_2_cur.0 {
            let tmp1 = self.get_connected_neighbors(path_1_cur.0.0, path_1_cur.0.1);
            let tmp2 = self.get_connected_neighbors(path_2_cur.0.0, path_2_cur.0.1);

            let neigh_1 = tmp1.iter()
                .find(|(coord,tile)| *coord != path_1_last.coord)
                .unwrap();
            let neigh_2 = tmp2.iter()
                .find(|(coord,tile)| *coord != path_2_last.coord)
                .unwrap();

            path_1_last = Node {
                dist_from_start: path_1_last.dist_from_start + 1,
                coord: path_1_cur.0,
                tile: path_1_cur.1
            };
            path_2_last = Node {
                dist_from_start: path_2_last.dist_from_start + 1,
                coord: path_2_cur.0,
                tile: path_2_cur.1
            };
            *self.get_tile_mut(path_1_last.coord.0, path_1_last.coord.1) = MapTile::MainLoop;
            *self.get_tile_mut(path_2_last.coord.0, path_2_last.coord.1) = MapTile::MainLoop;

            path_1_cur = *neigh_1;
            path_2_cur = *neigh_2;
        }
        path_1_last = Node {
            dist_from_start: path_1_last.dist_from_start + 1,
            coord: path_1_cur.0,
            tile: path_1_cur.1
        };
        path_2_last = Node {
            dist_from_start: path_2_last.dist_from_start + 1,
            coord: path_2_cur.0,
            tile: path_2_cur.1
        };
        *self.get_tile_mut(path_1_last.coord.0, path_1_last.coord.1) = MapTile::MainLoop;
        *self.get_tile_mut(path_2_last.coord.0, path_2_last.coord.1) = MapTile::MainLoop;

        println!("Arrived at: {:?}", path_1_last);
        println!("Arrived at: {:?}", path_2_last);

        path_1_last
    }

    fn calc_tiles_inside_loop(&self) -> usize {
       self.data 
           .iter()
           .fold(0, |acc,l| {
               let cnt = l
               .iter()
               .enumerate()
               .fold(0, |acc,(idx,&tile)| {
                   if tile == MapTile::MainLoop {
                       return acc;
                   }
                   if l.iter().enumerate().take_while(|(i,&tile)| *i < idx && tile != MapTile::MainLoop).count() == idx {
                       return acc;
                   }
                   acc + (l[idx+1..]
                    .iter()
                    .map(|&tile| tile == MapTile::MainLoop)
                    .filter(|&is_loop| is_loop)
                    .count() % 2) as usize
               });
               println!("Row: {} tiles inside", cnt);
               acc + cnt
           })
    }

    fn get_tile(&self, x: usize, y: usize) -> &MapTile {
        self.data.get(y).unwrap().get(x).unwrap()
    }
    fn get_tile_mut(&mut self, x: usize, y: usize) -> &mut MapTile {
        self.data.get_mut(y).unwrap().get_mut(x).unwrap()
    }

    fn get_connected_neighbors(&self, x: usize, y: usize) -> Vec<((usize, usize), MapTile)> {
        let this_tile = self.get_tile(x, y);
        let mut ret = vec![];
        for neighbor in &NEIGHBORS {
            let coord = (x.checked_add_signed(neighbor.0), y.checked_add_signed(neighbor.1));
            if coord.0.is_none() || coord.1.is_none() {
                continue;
            }
            if coord.0.unwrap() > self.data.first().unwrap().len()-1 || coord.1.unwrap() > self.data.len()-1 {
                continue;
            }
            let neighbor_tile = self.get_tile(coord.0.unwrap(), coord.1.unwrap());
            if this_tile.can_connect(*neighbor_tile, *neighbor) {
                ret.push(((coord.0.unwrap(), coord.1.unwrap()), *neighbor_tile)); 
            }
        }
        println!("Neighbors of {:?} are {:?}", (x, y), ret);
        assert!(ret.len() <= 2);
        ret
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for col in row {
                use MapTile::*;
                match col {
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
