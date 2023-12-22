use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug)]
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

#[derive(Debug)]
struct Brick {
    name: Option<char>,
    extents: Extents 
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
}

impl From<&str> for Brick {
    fn from(value: &str) -> Self {
        Self {
            name: None,
            extents: Extents::from(value)
        }
    }
}

enum ViewDirection {
    Front,
    Side
}

fn print_bricks(bricks: &BTreeMap<usize, Vec<&Brick>>, view: ViewDirection) {
    let max_z = *bricks.last_key_value().unwrap().0;
    match view {
        ViewDirection::Front => {
            println!(" x ");
        },
        ViewDirection::Side => {
            println!(" y ");
        }
    }
    println!("012");
    for z in (0usize..=max_z).rev() {
        if !bricks.contains_key(&z) {
            println!("... {}", z);
            continue;
        }
        let mut z_container: Vec<&Brick> = bricks[&z].clone();
        match view {
            ViewDirection::Front => {
                z_container.sort_by_key(|b| b.extents.min.1);
            },
            ViewDirection::Side => {
                z_container.sort_by_key(|b| b.extents.min.0);
            }

        }
        for i in 0..3i32 {
            match view {
                ViewDirection::Front => {
                    for brick in &z_container {
                        if i >= brick.extents.min.0 && i <= brick.extents.max.0 {
                            print!("{}", brick.name.unwrap());
                        } else {
                            print!(".");
                        }
                    }
                },
                ViewDirection::Side => {
                    for brick in &z_container {
                        if i >= brick.extents.min.1 && i <= brick.extents.max.1 {
                            print!("{}", brick.name.unwrap());
                        } else {
                            print!(".");
                        }
                    }
                }
            }
        }
        println!(" {}", z);
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
            ret
        })
        .collect::<Vec<_>>();

    bricks.sort_by_key(|b| b.extents.min.2);

    let mut brick_levels: BTreeMap<usize, Vec<&Brick>> = BTreeMap::new();
    for brick in &bricks {
        for i in brick.extents.min.2 as usize..=brick.extents.max.2 as usize {
            brick_levels.entry(i)
                .and_modify(|v| { 
                    v.push(brick);
                })
                .or_insert(vec![ brick ]);
        }
    }

    for b in &bricks {
        println!("{:?}", b);
    }
    //println!("{:?}", brick_levels);

    print_bricks(&brick_levels, ViewDirection::Front);
    print_bricks(&brick_levels, ViewDirection::Side);

}
