use std::collections::BTreeMap;
use regex::{Regex,Captures};

#[derive(Debug)]
struct Mapping {
    from: String,
    to: String,
    mapping: BTreeMap::<usize, (usize, usize)>
}

impl Mapping {
    pub fn new(s: &str) -> Self {
        let mut ret = Mapping {
            from: String::new(),
            to: String::new(),
            mapping: BTreeMap::new()
        };

        let lines = s.split('\n');
        let parser = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
        for (idx,line) in lines.enumerate() {
            if line.is_empty() {
                continue;
            }
            if idx == 0 {
                let rex = Regex::new("(.+)-to-(.+) ").unwrap();
                let caps = rex.captures(line).unwrap();
                ret.from = String::from(caps.get(1).unwrap().as_str());
                ret.to = String::from(caps.get(2).unwrap().as_str());
                continue;
            }
            println!("Finna parse {}", line);
            let caps = parser.captures(line).unwrap();
            let source_idx = caps.get(2).unwrap().as_str();
            let dest_idx = caps.get(1).unwrap().as_str();
            let len_range = caps.get(3).unwrap().as_str();
            ret.mapping.insert(source_idx.parse().unwrap(), (dest_idx.parse().unwrap(), len_range.parse().unwrap()));
        }

        ret
    }
}

fn main() {
    let content = include_str!("../../input2.txt");
    
    let categories = content.split("\n\n");
    let mut mappings = vec![];
    for (idx,cat) in categories.enumerate() {
        if idx == 0 {
            continue;
        }
        mappings.push(Mapping::new(cat));
        println!("{:?}", mappings.last().unwrap());
    }
}
