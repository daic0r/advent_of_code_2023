use std::collections::HashMap;
use regex::Regex;

fn count_steps(seq: &str, matrix: HashMap::<String, (String, String)>) -> usize {
    let mut seq_idx = 0usize;
    let mut ret_step_cnt = 0usize;

    let mut cur_nodes = matrix
        .keys()
        .filter(|k| k.chars().nth(2).unwrap() == 'A')
        .map(|k| k.as_str())
        .collect::<Vec<&str>>();
    for node in &cur_nodes {
        println!("{}", node);
    }
    while !cur_nodes.iter().all(|k| k.chars().nth(2).unwrap() == 'Z') {
        match seq.chars().nth(seq_idx).unwrap() {
            'L' => {
                cur_nodes = cur_nodes
                    .iter()
                    .map(|k| matrix
                        .get(*k)
                        .unwrap()
                        .0.as_str())
                    .collect();
            }
            'R' => {
                cur_nodes = cur_nodes
                    .iter()
                    .map(|k| matrix
                        .get(*k)
                        .unwrap()
                        .1.as_str())
                    .collect();
            }
            _ => panic!("Should never happen")
        };
        seq_idx = (seq_idx + 1) % seq.len();
        ret_step_cnt += 1;
    }
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
