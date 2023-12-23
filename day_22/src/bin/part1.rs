use std::borrow::BorrowMut;
use std::collections::BTreeMap;
use std::cell::RefCell;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, Default)]
struct Vec2d(i32,i32,i32);

impl From<&str> for Vec2d {
    fn from(value: &str) -> Self {
        let mut vals = value.split(',');
        Self {
            0: vals.next().unwrap().parse().unwrap(),
            1: vals.next().unwrap().parse().unwrap(),
            2: vals.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Extents {
    min: Vec2d,
    max: Vec2d
}

impl Extents {
    fn intersects(&self, rhs: &Extents) -> bool {
        let no_intersect = self.min.0 < rhs.min.0 && self.max.0 < rhs.min.0 ||
            self.min.0 > rhs.max.0 && self.max.0 > rhs.max.0 ||
            self.min.1 < rhs.min.1 && self.max.1 < rhs.min.1 ||
            self.min.1 > rhs.max.1 && self.max.1 > rhs.max.1 ||
            self.min.2 < rhs.min.2 && self.max.2 < rhs.min.2 ||
            self.min.2 > rhs.max.2 && self.max.2 > rhs.max.2;
        !no_intersect
    }
    
    fn contains(&self, point: Vec2d) -> bool {
        point.0 >= self.min.0 && point.0 <= self.max.0 &&
        point.1 >= self.min.1 && point.1 <= self.max.1 &&
        point.2 >= self.min.2 && point.2 <= self.max.2
    }
}

#[derive(Debug, Clone, Default)]
struct Brick {
    name: Option<char>,
    extents: Extents,
    rests_on: Vec<usize>
}
impl From<&str> for Extents {
    fn from(value: &str) -> Self {
        let mut vals = value.split('~');
        let min = vals.next().unwrap();
        let max = vals.next().unwrap();
        Self {
            min: Vec2d::from(min),
            max: Vec2d::from(max)
        }
    }
}

impl Brick {
    fn intersects(&self, rhs: &Brick) -> bool {
        self.extents.intersects(&rhs.extents)
    }

    fn lower(&self) -> Brick {
        let mut ret = self.clone();
        ret.extents.min.2 -= 1;
        ret.extents.max.2 -= 1;
        ret
    }
}

impl From<&str> for Brick {
    fn from(value: &str) -> Self {
        Self {
            name: None,
            extents: Extents::from(value),
            rests_on: vec![]
        }
    }
}

enum ViewDirection {
    Front,
    Side
}

fn print_bricks(map: &BTreeMap<usize, RefCell<Vec<usize>>>, bricks: &Vec<RefCell<Brick>>, view: ViewDirection) {
    let max_z = *map.last_key_value().unwrap().0;
    match view {
        ViewDirection::Front => {
            println!(" x ");
        },
        ViewDirection::Side => {
            println!(" y ");
        }
    }
    println!("012");
    for z in (1usize..=max_z).rev() {
        if !map.contains_key(&z) {
            println!("... {}", z);
            continue;
        }
        let mut z_container = map[&z].borrow().clone();
        match view {
            ViewDirection::Front => {
                z_container.sort_by_key(|&b| bricks[b].borrow().extents.min.1);
            },
            ViewDirection::Side => {
                z_container.sort_by_key(|&b| bricks[b].borrow().extents.min.0);
            }

        }
        for i in 0..3i32 {
            match view {
                ViewDirection::Front => {
                    let brick_count = z_container
                        .iter()
                        .filter(|&&brick| i >= bricks[brick].borrow().extents.min.0 && i <= bricks[brick].borrow().extents.max.0)
                        .count();
                    if brick_count > 1
                    {
                        print!("?");
                    } 
                    else 
                    if brick_count == 0 {
                        print!(".");
                    }
                    else
                    {
                        let brick_idx = z_container.iter().find(|&&brick| i >= bricks[brick].borrow().extents.min.0 && i <= bricks[brick].borrow().extents.max.0).unwrap();
                        let brick = bricks[*brick_idx].borrow();
                        if i >= brick.extents.min.0 && i <= brick.extents.max.0 {
                            print!("{}", brick.name.unwrap());
                        }
                    }
                },
                ViewDirection::Side => {
                    let brick_count = z_container
                        .iter()
                        .filter(|&&brick| i >= bricks[brick].borrow().extents.min.1 && i <= bricks[brick].borrow().extents.max.1)
                        .count();
                    if brick_count > 1
                    {
                        print!("?");
                    } 
                    else 
                    if brick_count == 0 {
                        print!(".");
                    }
                    else
                    {
                        let brick_idx = z_container.iter().find(|&&brick| i >= bricks[brick].borrow().extents.min.1 && i <= bricks[brick].borrow().extents.max.1).unwrap();
                        let brick = bricks[*brick_idx].borrow();
                        if i >= brick.extents.min.1 && i <= brick.extents.max.1 {
                            print!("{}", brick.name.unwrap());
                        }
                    }
                }
            }
        }
        println!(" {}", z);
    }
}

fn drop_pieces(map: &mut BTreeMap<usize, RefCell<Vec<usize>>>, bricks: &mut Vec<RefCell<Brick>>) {
    let max_z = *map.last_key_value().unwrap().0;
    for (idx,brick) in bricks.iter().enumerate() {
        loop {
            if brick.borrow().extents.min.2 < 2 {
                break;
            }
            let new_level = brick.borrow().extents.min.2 as usize-1usize;
            println!("{}", new_level);
            if !map.contains_key(&new_level) {
                map.insert(new_level, RefCell::new(vec![]));
            }
            let mut dst = map[&new_level].borrow_mut();
            let mut intersect = false;
            for &b2 in dst.iter() {
                if bricks[b2].borrow().intersects(&brick.borrow().lower()) {
                    brick.borrow_mut().rests_on.push(b2);
                    intersect = true;
                    //break;
                }
            }
            if !intersect {
                // Remove from old maximum level
                let map_entry = &map[&(brick.borrow().extents.max.2 as usize)];
                let pos = map_entry.borrow().iter().position(|&b| b == idx).unwrap();
                map_entry.borrow_mut().remove(pos);

                // Add to new minimum level
                dst.push(idx);

                let lowered = brick.borrow().lower();
                *brick.borrow_mut() = lowered;
            } else {
                break;
            }
        }
         
    }
}


fn main() {
    let content = std::fs::read_to_string("input2.txt").unwrap();

    let mut next_name = 'A';

    let mut max_z = 0;
    let mut bricks = content
        .lines()
        .map(|l| {
            let mut ret = Brick::from(l);
            max_z = std::cmp::max(ret.extents.max.2, max_z);
            ret.name = Some(next_name);
            next_name = char::from_u32(u32::from(next_name) + 1).unwrap();
            RefCell::new(ret)
        })
        .collect::<Vec<_>>();

    bricks.sort_by_key(|b| b.borrow().extents.min.2);

    let mut brick_levels: BTreeMap<usize, RefCell<Vec<usize>>> = BTreeMap::new();
    for (idx,brick) in bricks.iter().enumerate() {
        for i in brick.borrow().extents.min.2 as usize..=brick.borrow().extents.max.2 as usize {
            brick_levels.entry(i)
                .and_modify(|v| { 
                    v.deref().borrow_mut().push(idx);
                })
                .or_insert(RefCell::new(vec![idx]));
        }
    }

    for b in &bricks {
        println!("{:?}", b);
    }
    println!("{:?}", brick_levels);

    drop_pieces(&mut brick_levels, &mut bricks);

    print_bricks(&brick_levels, &bricks, ViewDirection::Front);
    println!();
    print_bricks(&brick_levels, &bricks, ViewDirection::Side);

    println!();
    for b in &bricks {
        print!("{} rests on ", b.borrow().name.unwrap());
        for support in &b.borrow().rests_on {
            print!("{} ", bricks[*support].borrow().name.unwrap());
        }
        println!();
    }

}
