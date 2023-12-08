use std::collections::HashMap;
use regex::Regex;

fn count_steps(seq: &str, matrix: HashMap::<String, (String, String)>) -> usize {
    let mut seq_idx = 0usize;
    let mut ret_step_cnt = 0usize;

    let mut cur_node = matrix.get_key_value("AAA").unwrap();
    while cur_node.0 != "ZZZ" {
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
    ret_step_cnt
}


fn main() {
    let lines = include_str!("../../input.txt").split('\n').filter(|l| !l.is_empty());

    let mut seq = "";
    let rex = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();
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
