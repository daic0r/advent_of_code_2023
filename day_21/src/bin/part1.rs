use std::collections::VecDeque;

struct Map {
    data: Vec<Vec<char>>,
}

impl Map {
    fn get_neighbors(&self, coord: (usize, usize)) -> Vec<(usize, usize)> {
        const OFFSETS: [(isize, isize); 4] = [
            (-1, 0),
            (0, -1),
            (1, 0),
            (0, 1)
        ];

        let mut ret = vec![];
        for offset in &OFFSETS {
            let (neigh_x, neigh_y) = (coord.0.checked_add_signed(offset.0), coord.1.checked_add_signed(offset.1));
            if neigh_x.is_none() || neigh_y.is_none() 
            {
                continue;
            }
            let neigh_x = neigh_x.unwrap();
            let neigh_y = neigh_y.unwrap();
            if neigh_x > self.data.first().unwrap().len()-1
                || neigh_y > self.data.len()-1
            {
                continue;
            }
            if self.data[neigh_y][neigh_x] != '#' {
                ret.push((neigh_x, neigh_y));
            }
        }
        ret
    }

    fn find_num_reachable_tiles(&self, start: (usize, usize), steps: usize) -> usize {
        // ((coord.x, coord.y), steps)
        let mut the_queue: VecDeque<((usize, usize), usize)> = VecDeque::new();        

        the_queue.push_back((start, 0));

        while !the_queue.iter().all(|e| e.1 == steps) {
            let tile = the_queue.pop_front().unwrap();

            for neighbor in self.get_neighbors(tile.0) {
                let data = (neighbor, tile.1 + 1);
                if !the_queue.contains(&data) {
                    the_queue.push_back(data);
                }
            }
        }

        the_queue.len()
    }

    fn find_start(&self) -> (usize, usize) {
        let line_start = self.data
            .iter()
            .enumerate()
            .fold(0, |acc,(line_idx,line)|
                acc + (line.contains(&'S') as usize) * line_idx
            );
        (self.data[line_start].iter().position(|&c| c == 'S').unwrap(), line_start) 
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        Map {
            data: value
                    .lines()
                    .map(|l| l.chars().collect::<Vec<_>>())
                    .collect::<Vec<_>>()
        }
    }
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    let m = Map::from(content.as_str());

    println!("Start is: {:?}", m.find_start());
    let num_tiles = m.find_num_reachable_tiles(m.find_start(), 64);

    println!("Number of reachable tiles: {}", num_tiles);
}
