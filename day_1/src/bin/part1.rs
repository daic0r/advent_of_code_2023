use std::fs::File;
use std::io::{BufRead, BufReader};

fn value_from_line(line: &str) -> u32 {
    let dig1 = line.chars().find(|c| c.is_ascii_digit()).unwrap().to_digit(10).unwrap();
    let dig2 = line.chars().rfind(|c| c.is_ascii_digit()).unwrap().to_digit(10).unwrap();
    return (10 * dig1) + dig2;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let sum = reader.lines().fold(0, |acc,line| acc+value_from_line(&line.unwrap()));
    println!("Sum: {}", sum);
}
