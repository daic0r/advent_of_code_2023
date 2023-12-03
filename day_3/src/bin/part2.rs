use std::borrow::BorrowMut;
use std::cmp::{max,min};
use termion::{color, cursor};
use termion::raw::IntoRawMode;
use std::io::{stdout, Write};


#[allow(dead_code)]
#[derive(Debug,Clone)]
struct Number {
    value: i32,
    line: usize,
    col_range: (usize, usize)
}

#[allow(dead_code)]
#[derive(Debug)]
struct Symbol {
    line: usize,
    col: usize,
    numbers: Option<Vec<Number>>
}

#[allow(dead_code)]
#[derive(Debug)]
struct Schematic {
    data: Vec<String>,
    numbers: Vec<Vec<Number>>,
    symbols: Vec<Vec<Symbol>>
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
        match in_number {
            true => {
                if let Some(num) = &mut num {
                    num.col_range.1 = s.len();
                    num.value = buf.unwrap().parse().unwrap();
                }
                ret.push(num.unwrap());
            },
            _ => {}
        }

        ret
    }

    pub fn new(data: Vec<String>) -> Self {
        let mut ret = Schematic { data, numbers: vec![], symbols: vec![] };
        for (idx,line) in ret.data.iter().enumerate() {
            let mut line_nums = Schematic::scan_line_numbers(line, idx);
            ret.numbers.push(line_nums);
            let mut line_symbols = vec![];
            for (col_idx,ch) in line.chars().enumerate() {
                if !ch.is_ascii_digit() && ch != '.' {
                    line_symbols.push(Symbol{ 
                        line: idx, 
                        col: col_idx, 
                        numbers: None
                    });
                    if ch == '*' {
                        line_symbols.last_mut().unwrap().numbers = Some(vec![]);
                    }
                }
            }
            ret.symbols.push(line_symbols);
        }
        assert_eq!(ret.symbols.len(), ret.data.len());
        assert_eq!(ret.numbers.len(), ret.data.len());
        ret
    }
    fn check_adj_line(sym: &Symbol, adj_line: Option<&Vec<Number>>) -> Option<Vec<Number>> {
        if let Some(adj_line) = adj_line {
            let iter = adj_line.iter()
                .filter(|num| sym.col >= max(0, (num.col_range.0 as i32)-1) as usize && sym.col <= num.col_range.1)
                .map(|num| num.clone())
                .collect();
            return Some(iter);
        }
        None
    }

    fn find_gears(&mut self) -> Vec<&Symbol> {
        let mut ret: Vec<&Symbol> = vec![];
        for sym in self.symbols.iter_mut().flatten().filter(|sym| sym.numbers.is_some()) {
            // Check current line
            let cur_line = &self.numbers[sym.line];
            for adj_num in cur_line.iter().filter(|num| (num.col_range.1 == sym.col) || num.col_range.0 == sym.col+1) {
                sym.numbers.as_mut().unwrap().push(adj_num.clone());
            }
            // Check above
            let mut numbers_above: Option<&Vec<Number>> = None;
            if sym.line > 0 {
                numbers_above = Some(&self.numbers[sym.line-1]);
            }
            let mut nums = Self::check_adj_line(sym, numbers_above);
            if let Some(nums) = &mut nums {
                sym.numbers.as_mut().unwrap().append(nums);
            }
            // Check below
            let mut numbers_below: Option<&Vec<Number>> = None;
            if sym.line < self.data.len()-1 {
                numbers_below = Some(&self.numbers[sym.line+1]);
            }
            let mut nums = Self::check_adj_line(sym, numbers_below);
            if let Some(nums) = &mut nums {
                sym.numbers.as_mut().unwrap().append(nums);
            }
            if sym.numbers.as_mut().unwrap().len() != 2 {
                println!("{:?}", sym.numbers.as_ref().unwrap());
            }
            if sym.numbers.as_mut().unwrap().len() == 2 {
                ret.push(sym);
            }
        }
        ret
    }

    /*
    pub fn print(&self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        for (idx,line) in self.data.iter().enumerate() {
            for num in &self.numbers[idx] {
                if self.is_number_adjacent_to_symbol(num) {
                    print!("{}", color::Fg(color::Green));
                } else {
                    print!("{}", color::Fg(color::Red));
                }
                print!("{}{}", cursor::Goto(num.col_range.0 as u16, num.line as u16), num.value);
            }
            for symbol in &self.symbols[idx] {
                print!("{}", color::Fg(color::Yellow));
                print!("{}{}", cursor::Goto(symbol.col as u16, symbol.line as u16), self.data[symbol.line].chars().nth(symbol.col).unwrap());
            }
            println!();
        }
    }
    */
}

fn main() {
    let data = include_str!("../../input.txt");
    let mut schem = Schematic::new(data.split('\n').map(|l| l.to_string()).collect());
    let gears = schem.find_gears();
    for gear in gears.iter() {
        println!("{:?}", gear.numbers.as_ref().unwrap());
    }
    let sum = gears.iter().fold(0, |acc,x| acc+x.numbers.as_ref().unwrap().iter().fold(1, |acc,x| acc*x.value));
    println!("Sum = {}", sum);
    //schem.print();
    // println!("{:?}", schem.numbers[0]);
    // println!("Determined that {} numbers are relevant.", rel_numbers.len());
    // println!("Sum = {}", rel_numbers.iter().fold(0, |acc,x| acc+x.value));
}
