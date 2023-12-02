use std::fs::{File};
use std::io::{BufRead, BufReader};
use regex::{Regex,Matches};
use std::str::FromStr;

#[derive(Debug)]
struct Pick {
    red: u32,
    green: u32,
    blue: u32
}

impl Pick {
    fn new(red: u32, green: u32, blue: u32) -> Pick {
        Pick { red, green, blue }
    }
}

impl FromStr for Pick {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 4 green, 2 blue
        let rex = Regex::new(r"(\d+) (red|green|blue)").unwrap();
        let mut ret = Pick::new(0, 0, 0);
        s.split(',').map(|str| str.trim()).map(|str| rex.captures(str).unwrap()).for_each(|cap| {
            let count = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let color = cap.get(2).unwrap().as_str();
            match color {
                "red" => ret.red = count,
                "green" => ret.green = count,
                "blue" => ret.blue = count,
                _ => panic!("Unknown color")
            }
        });
        Ok(ret)
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    picks: Vec<Pick>
}

impl Game {
    fn new(id: u32) -> Game {
        Game { id, picks: Vec::new() }
    }
}

impl FromStr for Game {
    // Game 1: 4 green, 2 blue; 1 red, 1 blue, 4 green; 3 green, 4 blue, 1 red; 7 green, 2 blue, 4 red; 3 red, 7 green; 3 red, 3 green
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rex = Regex::new(r"Game (\d+): (.*)").unwrap();
        let cap = rex.captures(s).unwrap();
        let id = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let picks = cap.get(2).unwrap().as_str().split(';').map(|str| str.trim()).map(|str| Pick::from_str(str).unwrap()).collect();
        Ok(Game { id, picks })
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let reference = Pick::new(12, 13, 14);
    let mut sum_powers = 0u32;
    for l in reader.lines() {
        let line = l.unwrap();
        if let Ok(game) = Game::from_str(&line) {
            let mut max_r = 0u32;
            let mut max_g = 0u32;
            let mut max_b = 0u32;
            for pick in game.picks {
                if pick.red > max_r {
                    max_r = pick.red;
                }
                if pick.green > max_g {
                    max_g = pick.green;
                }
                if pick.blue > max_b {
                    max_b = pick.blue;
                }
            }
            sum_powers += max_r*max_g*max_b;
        }
    }
    println!("Sum of powers: {}", sum_powers);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pick() {
        let pick = Pick::from_str("4 green, 2 blue").unwrap();
        assert_eq!(pick.red, 0);
        assert_eq!(pick.green, 4);
        assert_eq!(pick.blue, 2);
    }

    #[test]
    fn test_game() {
        let game = Game::from_str("Game 1: 4 green, 2 blue; 1 red, 1 blue, 4 green; 3 green, 4 blue, 1 red; 7 green, 2 blue, 4 red; 3 red, 7 green; 3 red, 3 green").unwrap();
        assert_eq!(game.id, 1);
        assert_eq!(game.picks.len(), 6);
        assert_eq!(game.picks[0].red, 0);
        assert_eq!(game.picks[0].green, 4);
        assert_eq!(game.picks[0].blue, 2);
        assert_eq!(game.picks[1].red, 1);
        assert_eq!(game.picks[1].green, 4);
        assert_eq!(game.picks[1].blue, 1);
        assert_eq!(game.picks[2].red, 1);
        assert_eq!(game.picks[2].green, 3);
        assert_eq!(game.picks[2].blue, 4);
        assert_eq!(game.picks[3].red, 4);
        assert_eq!(game.picks[3].green, 7);
        assert_eq!(game.picks[3].blue, 2);
        assert_eq!(game.picks[4].red, 3);
        assert_eq!(game.picks[4].green, 7);
        assert_eq!(game.picks[4].blue, 0);
        assert_eq!(game.picks[5].red, 3);
        assert_eq!(game.picks[5].green, 3);
        assert_eq!(game.picks[5].blue, 0);
    }
}
