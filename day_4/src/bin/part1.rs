use regex::{Regex,Captures};
use std::cmp::max;

#[derive(Debug)]
struct Ticket {
    winning: Vec<u32>,
    have: Vec<u32>
}

impl Ticket {
    fn new(s: &str) -> Self {
        let rex = Regex::new(r"Card\s+\d+:\s+(.*)").unwrap();
        println!("Doing '{}'", s);
        let data = rex.captures(s).unwrap().get(1).unwrap().as_str();
        let num_groups = data.split('|').collect::<Vec<&str>>();
        let winning = num_groups[0].split_whitespace().map(|num_str| num_str.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let have = num_groups[1].split_whitespace().map(|num_str| num_str.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        Ticket {
            winning,
            have
        }
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let lines = input.split('\n');
    let tickets = lines.filter(|l| !l.is_empty()).map(|line| Ticket::new(&line)).collect::<Vec<Ticket>>();
    let mut sum = 0;
    for ticket in &tickets {
        println!("{:?}", ticket);
        let cnt = ticket.have.iter().filter(|num| ticket.winning.contains(num)).count();
        if cnt == 0 {
            continue;
        }
        sum += 2u32.pow(max(0, (cnt as i32) - 1) as u32);
    }
    println!("Sum = {}", sum);
}
