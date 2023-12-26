#[derive(Debug, Clone, Copy)]
struct Vec2(f64, f64);
#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec3(f64, f64, f64);

const DIVISOR: f64 = 100_000.0;

impl std::ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl std::ops::Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f64) -> Vec2 {
        Vec2(self.0 * rhs, self.1 * rhs)
    }
}
impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}
impl From<Vec2> for Vec3 {
    fn from(v: Vec2) -> Vec3 {
        Vec3(v.0, v.1, 0.0)
    }
}
impl From<Vec3> for Vec2 {
    fn from(v: Vec3) -> Vec2 {
        Vec2(v.0, v.1)
    }
}
impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() <= 0.000001f64 && 
        (self.1 - other.1).abs() <= 0.000001f64
    }
}
impl From<&str> for Vec3 {
    fn from(value: &str) -> Vec3 {
        let mut splits = value.split(",").map(|s| s.trim());
        let x = splits.next().unwrap().parse::<f64>().unwrap() / DIVISOR;
        let y = splits.next().unwrap().parse::<f64>().unwrap() / DIVISOR;
        let z = splits.next().unwrap().parse::<f64>().unwrap() / DIVISOR;
        Vec3 {
            0: x,
            1: y,
            2: z
        }
    }
}

#[derive(Debug)]
struct Hailstone {
    pos: Vec3,
    vel: Vec3
}

impl Hailstone {
    fn cross(&self ,rhs: &Hailstone) -> Option<(f64, f64)> {
        let t1;
        if rhs.vel.1 != 0.0 {
            let quot = rhs.vel.0 / rhs.vel.1;
            let (self_pos_x, self_vel_x) = (self.pos.0 - quot * self.pos.1,
                self.vel.0 - quot * self.vel.1);
            if self_vel_x.abs() <= f64::EPSILON {
                return None;
            }
            let mut rhs_pos_x = rhs.pos.0 - quot * rhs.pos.1;
            rhs_pos_x -= self_pos_x;
            t1 = rhs_pos_x / self_vel_x;
        } else {
            println!("Alternate value");
            t1 = rhs.pos.0 - self.pos.0
        }
        let t2 = (self.pos.0 + (self.vel.0 * t1) - rhs.pos.0) / rhs.vel.0;
        if t1 < 0.0 || t2 < 0.0 {
            return None;
        }
        //assert_eq!(Vec2::from(self.proceed_to(t1).pos), Vec2::from(rhs.proceed_to(t2).pos));
        Some((t1, t2))
    }

    fn proceed_to(&self, t: f64) -> Hailstone {
        Hailstone {
            pos: self.pos + (self.vel * t),
            vel: self.vel
        }
    }
}

impl From<&str> for Hailstone {
    fn from(value: &str) -> Hailstone {
        let mut splits = value.split(" @ ");
        let pos = Vec3::from(splits.next().unwrap());
        let vel = Vec3::from(splits.next().unwrap());
        Hailstone {
            pos,
            vel
        }
    }
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    let hailstones = content
        .lines()
        .map(Hailstone::from)
        .collect::<Vec<_>>();

    for h in &hailstones {
        println!("{:?}", h);
    }

    const RANGE_MIN: f64 = 200000000000000.0 / DIVISOR;
    const RANGE_MAX: f64 = 400000000000000.0 / DIVISOR;

    let mut cnt = 0usize;
    for i in 1..hailstones.len()-1 {
        for h1 in hailstones.iter().skip(i-1) {
            for h2 in hailstones.iter().skip(i) {
                // println!("Stone A: {:?}", h1); 
                // println!("Stone B: {:?}", h2); 
                let ts = h1.cross(h2);
                if let Some(ts) = ts {
                    // println!("Intersection after t1 = {}, t2 = {}", ts.0, ts.1);
                    let new_h1 = h1.proceed_to(ts.0);
                    let new_h2 = h2.proceed_to(ts.1);
                    // println!("Intersection at A (x,y) = ({},{})", new_h1.pos.0, new_h1.pos.1);
                    // println!("Intersection at B (x,y) = ({},{})", new_h2.pos.0, new_h2.pos.1);
                    if new_h1.pos.0 >= RANGE_MIN && new_h1.pos.0 <= RANGE_MAX &&
                        new_h1.pos.1 >= RANGE_MIN && new_h1.pos.1 <= RANGE_MAX
                    {
                        // println!("->> TEST AREA");
                        cnt += 1;
                    }
                } else {
                    //println!("-> No intersection");
                }
            }
        }
    }
    println!();
    println!("{} in test area", cnt);

}
