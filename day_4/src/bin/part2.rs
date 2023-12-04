use regex::{Regex,Captures};
use std::borrow::BorrowMut;
use std::cmp::max;
use std::collections::BTreeMap;
use std::cell::RefCell;

#[derive(Debug)]
struct Ticket {
    number: u32,
    winning: Vec<u32>,
    have: Vec<u32>
}

impl Ticket {
    fn new(s: &str) -> Self {
        let rex = Regex::new(r"Card\s+(\d+):\s+(.*)").unwrap();
        println!("Doing '{}'", s);
        let captures = rex.captures(s).unwrap();
        let number = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let data = captures.get(2).unwrap().as_str();
        let num_groups = data.split('|').collect::<Vec<&str>>();
        let winning = num_groups[0].split_whitespace().map(|num_str| num_str.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let have = num_groups[1].split_whitespace().map(|num_str| num_str.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        Ticket {
            number,
            winning,
            have
        }
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let lines = input.split('\n');
    let tickets = lines.filter(|l| !l.is_empty()).map(|line| Ticket::new(&line)).collect::<Vec<Ticket>>();
    for ticket in &tickets {
        println!("{:?}", ticket);
    }
    let mut map = BTreeMap::<u32, RefCell<usize>>::new();
    tickets.iter().for_each(|ticket| { map.insert(ticket.number, RefCell::new(1)); });

    for (tick_num, count) in &map {
        let ticket = tickets.iter().find(|t| t.number == *tick_num).unwrap();
        let winning_nums_cnt = ticket.have.iter().filter(|num| ticket.winning.contains(num)).count();
        for i in 1..=winning_nums_cnt {
            let k = tick_num + (i as u32);
            let mut cnt = map.get(&k).unwrap().borrow_mut();
            *cnt += *count.borrow();
        }
    }

    let sum = map.values().fold(0, |acc,x| acc+*x.borrow());
    println!("Sum = {}", sum);
}
