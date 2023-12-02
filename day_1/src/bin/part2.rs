use std::fs::File;
use std::io::{BufRead, BufReader};
use phf::phf_map;
use std::cmp::max;

static DIGITS: phf::Map<&str,u32> = phf_map! {
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
};

fn value_from_line(line: &str) -> u32 {
    let dig1_pos = line.find(|c: char| c.is_ascii_digit());
    let dig1_text_pos = DIGITS.into_iter().filter_map(|(k,v)| {
        let pos = line.find(k);
        if pos.is_some() {
            Some((pos,v.clone()))
        } else {
            None
        }
    }).min_by_key(|(k,v)| k.unwrap().clone());
    let dig1;
    if dig1_text_pos.is_none() || (dig1_pos.is_some() &&  dig1_text_pos.unwrap().0 > dig1_pos) {
        dig1 = line.chars().nth(dig1_pos.unwrap()).unwrap().to_digit(10).unwrap();
    } else {
        dig1 = dig1_text_pos.unwrap().1;
    }

    let dig2_pos = line.rfind(|c: char| c.is_ascii_digit());
    let dig2_text_pos = DIGITS.into_iter().filter_map(|(k,v)| {
        let pos = line.rfind(k);
        if pos.is_some() {
            Some((pos,v.clone()))
        } else {
            None
        }
    }).max_by_key(|(k,v)| k.unwrap().clone());
    let dig2;
    if dig2_text_pos.is_none() || (dig2_pos.is_some() && dig2_text_pos.unwrap().0 < dig2_pos) {
        dig2 = line.chars().nth(dig2_pos.unwrap()).unwrap().to_digit(10).unwrap();
    } else {
        dig2 = dig2_text_pos.unwrap().1;
    }

    (10 * dig1) + dig2
}

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(file);
    let sum = reader.lines().fold(0, |acc,line| acc+value_from_line(&line.unwrap()));
    println!("Sum: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_from_line() {
        //4nineeightseven2
        assert_eq!(value_from_line("4nineeightseven2"), 42);
        //7pqrstsixteen
        assert_eq!(value_from_line("7pqrstsixteen"), 76);
        //9qb95oneightsf
        assert_eq!(value_from_line("9qb95oneightsf"), 98);
    }
}
