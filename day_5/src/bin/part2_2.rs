use std::collections::{BTreeMap,HashMap};
use regex::{Regex,Captures};
use std::ops::Bound::*;
use std::thread;
use std::sync::{Arc,Mutex};
use std::cmp::min;

#[derive(Debug, Clone)]
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
        //println!("{:?}", rng);
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

    pub fn get_range(&self, src_rng: (usize,usize)) -> (usize,usize) {
        let mut rng: Vec<(&usize, &(usize,usize))> = self.mapping.range((Unbounded, Included(src_rng.0))).collect();
        //println!("{:?}", rng);
        if rng.is_empty() {
            return src_rng;
        }

        let mut ret = src_rng;

        let last = rng.last().unwrap();
        let rng_len = last.1.1;
        let src_rng_begin = last.0;
        let dst_rng_begin = last.1.0;
        if src_rng.0 < src_rng_begin + rng_len {
            let dst = dst_rng_begin + (src_rng.0 - src_rng_begin);
            // Get minimum of requested range length and remaining length of destination range
            let dst_interval_len = min(src_rng_begin + rng_len - src_rng.0, src_rng.1);
            ret = (dst, dst_interval_len);
        }

        ret
    }
}

fn find_mapped_range(seed_rng: (usize,usize), mappings: &HashMap::<String,Mapping>) -> (usize,usize) {
    let mut src_cat = "seed";
    //let mut mapping = mappings.iter().find(|m| m.from == src_cat);
    let mut mapping = mappings.get(src_cat);
    let mut src = seed_rng;
    while let Some(m) = mapping {
        print!("({},{}) -> ", src.0, src.1);
        src = m.get_range(src);
        println!("({},{})", src.0, src.1);  
        src_cat = m.to.as_str();
        mapping = mappings.get(src_cat);
        //mapping = mappings.iter().find(|m| m.from == src_cat);
    }

    src
}

fn get_final_values(seeds: Vec<(usize,usize)>, mappings: HashMap::<String,Mapping>) -> Vec<usize> {
    let final_values = vec![];
    println!("{:?}", seeds);
    for seed_rng in seeds {
        let mut rng = seed_rng;
        while rng.0 < seed_rng.0 + seed_rng.1 {
            let mapped_rng = find_mapped_range(rng, &mappings);
            println!("---------------------");
            //println!("({},{}) -> ({},{})", rng.0, rng.1, mapped_rng.0, mapped_rng.1);
            rng.0 += mapped_rng.1;
            rng.1 -= mapped_rng.1;
        }
        println!("------------------------------------------------------");
    }

    final_values
}

fn main() {
    let content = include_str!("../../input2.txt");
    
    let categories = content.split("\n\n");
    //let mut mappings = vec![];
    let mut mappings = HashMap::<String, Mapping>::new();
    let mut seeds = vec![];
    for (idx,cat) in categories.enumerate() {
        if idx == 0 {
            let rex = Regex::new(r"seeds:\s+(.*)").unwrap();
            let caps = rex.captures(cat).unwrap();
            let seed_nums = caps.get(1).unwrap().as_str().split(' ');
            let mut seed_iter = seed_nums.into_iter();
            while let Some(seed) = seed_iter.next() {
                seeds.push((seed.parse::<usize>().unwrap(), seed_iter.next().unwrap().parse::<usize>().unwrap())); 
            }
            continue;
        }
        let m = Mapping::new(cat);
        mappings.insert(m.from.clone(), m);
        /*
        println!("{:?}", mappings.last().unwrap());
        
        println!("{}", mappings.last().unwrap().get(50));
        */
    }

    let final_values = get_final_values(seeds, mappings);


    //println!("Lowest location: {}", final_values.iter().min().unwrap());

}
