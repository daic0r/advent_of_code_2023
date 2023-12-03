use std::borrow::BorrowMut;


#[allow(dead_code)]
#[derive(Debug)]
struct Number {
    value: i32,
    line: usize,
    col_range: (usize, usize)
}

#[allow(dead_code)]
#[derive(Debug)]
struct Symbol {
    line: usize,
    col: usize
}

#[allow(dead_code)]
#[derive(Debug)]
struct Schematic {
    data: Vec<String>,
    numbers: Vec<Number>
}

impl Schematic {
    fn scan_line_numbers(s: &str, line_num: usize) -> Vec<Number> {
        let mut ret = vec![];

        let mut num: Option<Number> = None;
        let mut in_number = false;
        let mut buf: Option<String> = None;
        for (idx, ch) in s.chars().enumerate() {
            match ch {
                '0'..='9' => match in_number {
                    false => {
                        in_number = true;
                        num = Some(Number{ value: -1, line: line_num, col_range: (idx,0) });
                        buf = Some(String::from(ch));
                    },
                    true => {
                        buf.as_mut().unwrap().push(ch);
                    }
                },
                _ => match in_number {
                    true => {
                        if let Some(num) = &mut num {
                            num.col_range.1 = idx;
                            num.value = buf.unwrap().parse().unwrap();
                            buf = None;
                        }
                        ret.push(num.unwrap());
                        num = None;
                        in_number = false;
                    },
                    _ => {}
                }
            }
        }

        ret
    }

    pub fn new(data: Vec<String>) -> Self {
        let mut ret = Schematic { data, numbers: vec![] };
        for (idx,line) in ret.data.iter().enumerate() {
            let mut line_nums = Schematic::scan_line_numbers(line, idx);
            ret.numbers.append(&mut line_nums);
        }
        ret
    }
}


fn main() {
    let data = include_str!("../../input2.txt");
    let schem = Schematic::new(data.split('\n').map(|l| l.to_string()).collect());
    println!("{:?}", schem);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = include_str!("../../input2.txt");
        let schem = Schematic::new(data.split('\n').map(|l| l.to_string()).collect());
    }
}
