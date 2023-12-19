use std::collections::HashMap;
use std::hash::Hash;
use regex::Regex;

#[derive(Debug)]
enum Rule {
    LessThan((Category,u32,String)),
    GreaterThan((Category,u32,String)),
    Accept,
    Reject
}

impl Rule {
    fn process(&self, p: &Part) -> Option<&str> {
        use Rule::*;
        match self {
            Accept => return Some("A"),
            Reject => return Some("R"),
            LessThan((cat,val,to)) => if p.categories[cat] < *val {
                Some(to)
            } else {
                None
            }
            GreaterThan((cat,val,to)) => if p.categories[cat] > *val {
                Some(to)
            } else {
                None
            }
        }
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

#[derive(Debug, Hash, Eq, PartialEq)]
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
struct Part {
    categories: HashMap<Category, u32> 
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let mut categories = HashMap::new();
        value[1..value.len()-1].split(",")
            .map(|s| {
                let val = s[2..].parse::<u32>().unwrap();
                (Category::from(&s[0..1]), val)
            })
            .for_each(|(cat,val)| { categories.insert(cat, val); });
        Self {
            categories
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    final_dest: String
}

impl Workflow {
    fn process(&self, p: &Part) -> &str {
        for rule in &self.rules {
            if let Some(to) = rule.process(p) {
                return to;
            }
        }
        self.final_dest.as_str()
    }
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

fn main() {
    let contents = include_str!("../../input.txt");

    let idx_separator = contents.lines().position(|l| l.is_empty()).unwrap();

    println!("Separator at {}", idx_separator);

    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    contents
        .lines()
        .take(idx_separator)
        .map(Workflow::from)
        .for_each(|w| { workflows.insert(w.name.clone(), w); });
        
    for w in &workflows {
        println!("{:?}", w);
    }

    let parts = contents.lines()
        .skip(idx_separator+1)
        .map(Part::from)
        .collect::<Vec<Part>>();

    println!();

    for p in &parts {
        println!("{:?}", p);
    }

    println!();

    let mut accepted = vec![];
    for p in &parts {
        let mut cur_workflow = "in";
        while cur_workflow != "A" && cur_workflow != "R" {
            let workflow = workflows.get(cur_workflow).unwrap();
            cur_workflow = workflow.process(p);
        }
        if cur_workflow == "A" {
            accepted.push(p);
        }
    }

    let sum = accepted
        .iter()
        .map(|&p| p.categories.values().sum::<u32>())
        .sum::<u32>();

    println!("Sum of accepted parts: {}", sum);

}
