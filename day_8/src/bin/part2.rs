use std::collections::HashMap;
use regex::Regex;
use std::{thread,time};
use num::integer::lcm;

fn count_steps_to_end_state(start_node: &str, seq: &str, matrix: &HashMap::<String, (String, String)>) -> usize {
    let mut seq_idx = 0usize;
    let mut ret_step_cnt = 0usize;

    let mut cur_node = matrix.get_key_value(start_node).unwrap();
    while cur_node.0.chars().nth(2).unwrap() != 'Z' || seq_idx != 0 {
        let nxt;
        match seq.chars().nth(seq_idx).unwrap() {
            'L' => nxt = cur_node.1.0.as_str(),
            'R' => nxt = cur_node.1.1.as_str(),
            _ => panic!("Should never happen")
        };
        cur_node = matrix.get_key_value(nxt).unwrap();
        seq_idx = (seq_idx + 1) % seq.len();
        ret_step_cnt += 1;
    }
    println!("{}: {}, {}", start_node, seq_idx, ret_step_cnt);
    assert!(seq_idx == 0);
    ret_step_cnt
}
fn count_steps(seq: &str, matrix: HashMap::<String, (String, String)>) -> usize {
    let mut start_nodes = matrix
        .keys()
        .filter(|k| k.chars().nth(2).unwrap() == 'A')
        .map(|k| k.as_str())
        .collect::<Vec<&str>>();

    let ret_step_cnt = start_nodes.iter()
        .map(|str_node| count_steps_to_end_state(str_node, seq, &matrix))
        .fold(1, |acc,x| lcm(acc, x));

    ret_step_cnt
}


fn main() {
    let lines = include_str!("../../input.txt").split('\n').filter(|l| !l.is_empty());

    let mut seq = "";
    let rex = Regex::new(r"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)").unwrap();
    let mut adj_matrix = HashMap::new();
    for (idx,line) in lines.enumerate() {
        if idx == 0 {
            seq = line;
            continue;
        }
        let caps = rex.captures(line).unwrap();
        let node = caps.get(1).unwrap().as_str();
        let left_node = caps.get(2).unwrap().as_str();
        let right_node = caps.get(3).unwrap().as_str();
        adj_matrix.insert(String::from(node), (String::from(left_node), String::from(right_node)));
        println!("{} = ({}, {})", node, left_node, right_node);
    }

    let steps = count_steps(seq, adj_matrix);

    println!("Steps needed: {}", steps);
    
}
