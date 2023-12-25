#[derive(Debug, Clone, Copy)]
struct Vec2(f32, f32);
#[derive(Debug, Clone, Copy)]
struct Vec3(f32, f32, f32);

impl std::ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Vec2 {
        Vec2(self.0 * rhs, self.1 * rhs)
    }
}
impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Vec3 {
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
impl From<&str> for Vec3 {
    fn from(value: &str) -> Vec3 {
        let mut splits = value.split(",").map(|s| s.trim());
        let x = splits.next().unwrap().parse::<f32>().unwrap();
        let y = splits.next().unwrap().parse::<f32>().unwrap();
        let z = splits.next().unwrap().parse::<f32>().unwrap();
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
    fn cross(&self ,rhs: &Hailstone) -> Option<(f32, f32)> {
        let t1;
        if rhs.vel.1 != 0.0 {
            let quot = rhs.vel.0 / rhs.vel.1;
            let (self_pos_x, self_vel_x) = (self.pos.0 - quot * self.pos.1,
                self.vel.0 - quot * self.vel.1);
            if self_vel_x == 0.0 {
                return None;
            }
            let mut rhs_pos_x = rhs.pos.0 - quot * rhs.pos.1;
            rhs_pos_x -= self_pos_x;
            t1 = rhs_pos_x / self_vel_x;
        } else {
            t1 = rhs.pos.0 - self.pos.0
        }
        let t2 = -(self.pos.0 + (self.vel.0 * t1) - rhs.pos.0);
        Some((t1, t2))
    }

    fn proceed_to(&self, t: f32) -> Hailstone {
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
    let content = std::fs::read_to_string("input2.txt").unwrap();

    let hailstones = content
        .lines()
        .map(Hailstone::from)
        .collect::<Vec<_>>();

    for h in &hailstones {
        println!("{:?}", h);
    }

    for i in 1..hailstones.len()-1 {
        for h1 in hailstones.iter().skip(i-1) {
            for h2 in hailstones.iter().skip(i) {
                println!("Stone A: {:?}", h1); 
                println!("Stone B: {:?}", h2); 
                let ts = h1.cross(h2);
                if let Some(ts) = ts {
                    println!("Intersection after t1 = {}, t2 = {}", ts.0, ts.1);
                    let new_h1 = h1.proceed_to(ts.0);
                    let new_h2 = h2.proceed_to(ts.1);
                    println!("Intersection at A (x,y) = ({},{})", new_h1.pos.0, new_h1.pos.1);
                    println!("Intersection at B (x,y) = ({},{})", new_h2.pos.0, new_h2.pos.1);
                } else {
                    println!("-> No intersection");
                }
            }
        }
    }

}
