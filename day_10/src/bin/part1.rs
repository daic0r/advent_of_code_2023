use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug)]
enum MapTile {
    LeftToRight,
    TopToBottom,
    TopToRight,
    TopToLeft,
    BottomToRight,
    BottomToLeft,
    Ground,
    Start
}

#[derive(Debug)]
struct Map {
    data: Vec<Vec<MapTile>>
}

impl Map {
    fn new(d: Vec<Vec<char>>) -> Self {
        Map {
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
            .collect()
        }
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
                    Start => write!(f, "S")?
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    let data = include_str!("../../input2.txt")
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


        let map = Map::new(data.clone());

        println!("\n{}", map);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_map() {
        let data = include_str!("../../input2.txt")
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
