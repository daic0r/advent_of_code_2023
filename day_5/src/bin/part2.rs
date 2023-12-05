use std::collections::{BTreeMap,HashMap};
use regex::{Regex,Captures};
use std::ops::Bound::*;
use std::thread;
use std::sync::{Arc,Mutex};

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
}

fn process_seed_range(thread: usize, seed_rng: (usize,usize), mappings: &HashMap::<String,Mapping>) -> usize {
    let mut final_values = vec![];
    for seed in seed_rng.0..seed_rng.0+seed_rng.1 {
        let mut src_cat = "seed";
        //let mut mapping = mappings.iter().find(|m| m.from == src_cat);
        let mut mapping = mappings.get(src_cat);
        let mut src = seed;
        while let Some(m) = mapping {
            src = m.get(src);
            src_cat = m.to.as_str();
            mapping = mappings.get(src_cat);
            //mapping = mappings.iter().find(|m| m.from == src_cat);
        }
        final_values.push(src);
        if (seed-seed_rng.0) % 100_000 == 0 {
            let percent = ((seed-seed_rng.0) as f64 / seed_rng.1 as f64) * 100.0;
            println!("Thread {}: {}%", thread, percent);
        }
    }
    final_values.iter().min().unwrap().clone()
}

fn get_final_values(seeds: Vec<(usize,usize)>, mappings: HashMap::<String,Mapping>) -> Vec<usize> {
    let final_values = Arc::new(Mutex::new(vec![]));
    println!("{:?}", seeds);
    let mappings = Arc::new(mappings);
    for seed_rng in seeds {
        let mut handles = vec![];
        let mut rng_begin = seed_rng.0;
        let mut thread_nr = 0usize;
        while rng_begin < seed_rng.0 + seed_rng.1 {
            let mut len = 10_000_000;
            if rng_begin+len > seed_rng.0 + seed_rng.1 {
                len = seed_rng.0 + seed_rng.1 - rng_begin;
            }
            let final_values = Arc::clone(&final_values);
            let mappings = Arc::clone(&mappings);
            println!("Starting thread {} for range {}-{}", thread_nr, rng_begin, rng_begin+len);
            handles.push(thread::spawn(move || {
                let final_value = process_seed_range(thread_nr, (rng_begin, len), &mappings);
                final_values.lock().unwrap().push(final_value);
            }));
            rng_begin = rng_begin + len;
            thread_nr += 1;
        }
        let mut num_joined = 0;
        let len = &handles.len();
        for handle in handles {
            let _ = handle.join();
            num_joined += 1;
            let percent = ((num_joined as f64) / (*len as f64)) * 100.0;
            println!("\r{}% of threads finished.", percent);
        }
    }

    let vec = final_values.lock().unwrap().clone();
    vec
}

fn main() {
    let content = include_str!("../../input.txt");
    
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

    //println!("{:?}", &final_values);

    println!("Lowest location: {}", final_values.iter().min().unwrap());

}
