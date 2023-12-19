use std::collections::HashMap;
use std::hash::Hash;
use std::ops:: RangeInclusive;
use regex::Regex;
use std::fmt::Display;

#[derive(Debug)]
enum Rule {
    LessThan((Category,u32,String)),
    GreaterThan((Category,u32,String)),
    Accept,
    Reject
}

impl Rule {
    fn adapt_range(&self, coll: &mut PartCollection, coll_inv: &mut PartCollection) -> Option<&str> {
        use Rule::*;
        match self {
            Accept => return Some("A"),
            Reject=> return Some("R"),
            _ => {}
        };
        let (range, inv_range) = match self {
            LessThan((cat,val,to)) | GreaterThan((cat,val,to)) => {
                (coll.ranges.get_mut(cat).unwrap(), coll_inv.ranges.get_mut(cat).unwrap())
            },
            _ => panic!("Impossible")
        };
        let (my_range, my_inv_range, to) = match self {
            LessThan((cat,val,to)) => (1..=val-1, *val..=4000, to),
            GreaterThan((cat,val,to)) => (val+1..=4000, 1..=*val, to),
            _ => panic!("Impossible")
        };
        if range.start() < my_range.start() && range.end() < my_range.start() {
            return None;
        } else
        if range.start() > my_range.end() && range.end() > my_range.end() {
            return None;
        }
        *inv_range = RangeInclusive::new(*std::cmp::max(range.start(), my_inv_range.start()),
            *std::cmp::min(range.end(), my_inv_range.end()));
        *range = RangeInclusive::new(*std::cmp::max(range.start(), my_range.start()),
            *std::cmp::min(range.end(), my_range.end()));

        Some(to)
    }
}

    impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        if value.len() == 1 {
            match value {
                "A" => return Rule::Accept,
                "R" => return Rule::Reject,
                _ => panic!("Ooops")
            }
        }
        let rex = Regex::new(r"([x,m,a,s])([<>])(\d+):([a-z]+|A|R)").unwrap();
        let cap = rex.captures(value).unwrap();
        let cat_str = cap.get(1).unwrap().as_str();
        let op_str = cap.get(2).unwrap().as_str();
        let val = cap.get(3).unwrap().as_str().parse::<u32>().unwrap();
        let next = cap.get(4).unwrap().as_str();

        let op_val_tup = (Category::from(cat_str), val, next.to_owned());
        match op_str {
            "<" => Rule::LessThan(op_val_tup),
            ">" => Rule::GreaterThan(op_val_tup),
            _ => panic!("Invalid operator string")
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Category {
    X,
    M,
    A,
    S
}

impl From<&str> for Category {
    fn from(value: &str) -> Self {
        if value.len() != 1 {
            panic!("Wrong length for Category");
        }
        match value {
            "x" => Category::X,
            "m" => Category::M,
            "a" => Category::A,
            "s" => Category::S,
            _ => panic!("Invalid Category string")
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    final_dest: String
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let rex = Regex::new(r"([a-z]+)\{(.+)\}").unwrap();
        let cap = rex.captures(value).unwrap();
        let name = cap.get(1).unwrap().as_str();
        let rules = cap.get(2).unwrap().as_str();
        let rule_parts = rules
            .split(",")
            .collect::<Vec<&str>>();
        
        Self {
            name: name.to_owned(),
            rules: rule_parts
                .iter()
                .take(rule_parts.len()-1)
                .map(|&s| Rule::from(s))
                .collect(),
            final_dest: rule_parts.last().unwrap().to_string()
        }
    }
}

impl Hash for Workflow {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Display for PartCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rng_x = &self.ranges[&Category::X];
        let rng_m = &self.ranges[&Category::M];
        let rng_a = &self.ranges[&Category::A];
        let rng_s = &self.ranges[&Category::S];
        f.write_fmt(format_args!("x: [{},{}], m: [{},{}], a: [{},{}], s: [{},{}]",
            rng_x.start(), rng_x.end(),
            rng_m.start(), rng_m.end(),
            rng_a.start(), rng_a.end(),
            rng_s.start(), rng_s.end()))
    }
}

#[derive(Clone)]
struct PartCollection {
    ranges: HashMap<Category, std::ops::RangeInclusive<u32>>
}

impl PartCollection {
    fn new() -> Self {
        let mut ranges = HashMap::new();
        ranges.insert(Category::X, 1..=4000);
        ranges.insert(Category::M, 1..=4000);
        ranges.insert(Category::A, 1..=4000);
        ranges.insert(Category::S, 1..=4000);
        Self {
            ranges
        }
    }

    fn get_combination_count(&self) -> u128 {
        self.ranges.values().fold(1, |acc,x| acc * (x.end() - x.start() + 1) as u128)
    }
}

fn fit_ranges(workflows: &HashMap<String, Workflow>) -> Vec<PartCollection> {
    let mut ret = vec![];

    let mut the_stack = vec![ ("in", "", PartCollection::new()) ];
    while !the_stack.is_empty() {
        let (wf_name, from, coll) = the_stack.pop().unwrap();
        println!("{}->{}, {}", from, wf_name, coll);
        if wf_name == "R" {
            continue;
        } else 
        if wf_name == "A" {
            ret.push(coll); 
            continue;
        }

        let wf = workflows.get(wf_name).unwrap();
        // inv_coll is the "inverted range" of what is passed into the next
        // processing state, i.e. if s < 400 is passed onto the next stage,
        // we only have s >= 400 for the next the rule. inv_coll is that s >= 400
        let mut inv_coll = coll.clone();
        for rule in &wf.rules {
            let mut this_coll = inv_coll.clone();
            let nxt = rule.adapt_range(&mut this_coll, &mut inv_coll);
            if let Some(nxt) = nxt {
                the_stack.push((nxt, wf_name, this_coll));
            }
        }
        the_stack.push((&wf.final_dest, wf_name, inv_coll.clone()));
    }
    ret
}

fn main() {
    let contents = include_str!("../../input.txt");

    let idx_separator = contents.lines().position(|l| l.is_empty()).unwrap();

    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    contents
        .lines()
        .take(idx_separator)
        .map(Workflow::from)
        .for_each(|w| { workflows.insert(w.name.clone(), w); });
        
    for w in &workflows {
        println!("{:?}", w);
    }

    println!();

    let combos = fit_ranges(&workflows);
    println!("Got {} ranges", combos.len()); 
    let sum = combos.iter().fold(0, |acc,x| acc + x.get_combination_count());
    println!("Number of combinations = {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_combinations() {
        let coll = PartCollection::new();
        assert_eq!(coll.get_combination_count(), 256000000000000);
    }
}
