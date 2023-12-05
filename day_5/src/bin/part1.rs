use std::collections::BTreeMap;
use regex::{Regex,Captures};
use std::ops::Bound::*;

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

    pub fn get(&self, src_idx: usize) -> usize {
        let rng: Vec<(&usize, &(usize,usize))> = self.mapping.range((Unbounded, Included(src_idx))).collect();
        println!("{:?}", rng);
        if rng.is_empty() {
            return src_idx;
        }

        let last = rng.last().unwrap();
        let rng_len = last.1.1;
        let src_rng_begin = last.0;
        let dst_rng_begin = last.1.0;
        if src_idx < src_rng_begin + rng_len {
            return dst_rng_begin + (src_idx - src_rng_begin);
        }

        return src_idx;
    }
}

fn main() {
    let content = include_str!("../../input.txt");
    
    let categories = content.split("\n\n");
    let mut mappings = vec![];
    let mut seeds = vec![];
    for (idx,cat) in categories.enumerate() {
        if idx == 0 {
            let rex = Regex::new(r"seeds:\s+(.*)").unwrap();
            let caps = rex.captures(cat).unwrap();
            let seed_nums = caps.get(1).unwrap().as_str().split(' ');
            for seed in seed_nums {
                seeds.push(seed.parse::<usize>().unwrap());
            }
            continue;
        }
        mappings.push(Mapping::new(cat));
        println!("{:?}", mappings.last().unwrap());
        
        println!("{}", mappings.last().unwrap().get(50));
    }


    let mut final_values = vec![];
    for seed in seeds {
        let mut src_cat = "seed";
        let mut mapping = mappings.iter().find(|m| m.from == src_cat);
        let mut src = seed;
        while let Some(m) = mapping {
            src = m.get(src);
            src_cat = m.to.as_str();
            mapping = mappings.iter().find(|m| m.from == src_cat);
        }
        final_values.push(src);
    }

    println!("{:?}", &final_values);

    println!("Lowest location: {}", final_values.iter().min().unwrap());

}
