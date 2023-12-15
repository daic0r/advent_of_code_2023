use regex::Regex;

#[derive(Debug,PartialEq)]
enum Operation {
    Add(u32),
    Remove
}

fn hash(s: &str) -> u8 {
    let mut cur_val = 0u32;
    for ch in s.bytes() {
        cur_val += ch as u32;
        cur_val *= 17;
        cur_val %= 256;
    }
    assert!(cur_val < 256);
    cur_val as u8
}

#[derive(PartialEq,Debug)]
struct Step(String, Operation);

fn extract_info(rex: &Regex, s: &str) -> Step {

    let mut caps = rex.captures(s).unwrap();

    let label = caps.get(1).unwrap().as_str();
    let remove_op = caps.get(2).unwrap().as_str() == "-";
    let op;
    if remove_op {
        op = Operation::Remove;
    } else {
        let f_length = caps.get(5).unwrap().as_str().parse::<u32>().unwrap();
        op = Operation::Add(f_length);
    }

    Step(label.to_owned(), op)
}

fn main() {
    let contents = include_str!("../../input.txt").replace("\n", "");

    let steps = contents.split(',');

    let mut boxes: [Vec<(String, u32)>; 256] = std::array::from_fn(|_| Vec::new());

    let mut rex = Regex::new(r"([a-z]+)(-|((=)(\d+)))").unwrap();

    for step_str in steps {
        let step = extract_info(&rex, step_str);
        let box_number = hash(&step.0) as usize;
        println!("{:?} -> {}", step, box_number);
        let container = boxes.get_mut(box_number).unwrap();
        match step.1 {
            Operation::Remove => {
                if let Some(idx) = container.iter().position(|s| s.0 == step.0) {
                    container.remove(idx);
                }
                println!("Removed {} from {}", step.0, box_number);
            },
            Operation::Add(focal_length) => {
                let elem = container.iter_mut().find(|s| s.0 == step.0);
                if let Some(lens) = elem {
                    lens.1 = focal_length;
                    println!("Set focal length of {} to {} in {}", step.0, focal_length, box_number);
                } else {
                    println!("Adding lense {},{} in {}", step.0, focal_length, box_number);
                    container.push((step.0, focal_length)); 
                }
            }
        }
    }

    let sum: u32 = boxes
        .iter()
        .enumerate()
        .map(|(idx,container)|
            container
            .iter()
            .enumerate()
            .fold(0u32, |acc,(cont_idx,lense)| 
                acc + ((idx+1) as u32 * (cont_idx+1) as u32 * lense.1)
            )
        )
        .sum();

    println!("Sum = {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("ot=9"), 9);
    }

    #[test]
    fn test_extract() {
        let mut rex = Regex::new(r"([a-z]+)(-|((=)(\d+)))").unwrap();
        assert!(extract_info(&rex, "qm=3") == Step("qm".to_owned(), Operation::Add(3)));
        assert!(extract_info(&rex, "sdula-") == Step("sdula".to_owned(), Operation::Remove));
    }
}
