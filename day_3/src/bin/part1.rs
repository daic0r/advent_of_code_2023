use std::borrow::BorrowMut;
use std::cmp::{max,min};
use termion::{color, cursor};
use termion::raw::IntoRawMode;
use std::io::{stdout, Write};


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
                    line_symbols.push(Symbol{ line: idx, col: col_idx });
                }
            }
            ret.symbols.push(line_symbols);
        }
        assert_eq!(ret.symbols.len(), ret.data.len());
        assert_eq!(ret.numbers.len(), ret.data.len());
        ret
    }

    fn is_number_adjacent_to_symbol(&self, num: &Number) -> bool {
        if num.col_range.0 > 0 && self.symbols[num.line].iter().any(|sym| sym.col == num.col_range.0-1) {
            return true;
        }
        if self.symbols[num.line].iter().any(|sym| sym.col == num.col_range.1) {
            return true;
        }
        let check_adj_line = |adj_line: Option<&Vec<Symbol>>| {
            if let Some(adj_line) = adj_line {
                if adj_line.iter().any(|sym| sym.col >= max(0, (num.col_range.0 as i32)-1) as usize && sym.col <= num.col_range.1) {
                    return true;
                }
            }
            false
        };
        let mut symbols_above: Option<&Vec<Symbol>> = None;
        if num.line > 0 {
            symbols_above = Some(&self.symbols[num.line-1]);
        }
        if check_adj_line(symbols_above) {
            return true;
        }
        let mut symbols_below: Option<&Vec<Symbol>> = None;
        if num.line < self.data.len()-1 {
            symbols_below = Some(&self.symbols[num.line+1]);
        }
        if check_adj_line(symbols_below) {
            return true;
        }
        false
    }

    pub fn get_relevant_numbers(&self) -> Vec<&Number> {
        self.numbers.iter()
            .flatten()
            .filter(|num| self.is_number_adjacent_to_symbol(num))
            .collect()
    }

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
}


fn main() {
    let data = include_str!("../../input.txt");
    let schem = Schematic::new(data.split('\n').map(|l| l.to_string()).collect());
    let rel_numbers = schem.get_relevant_numbers();
    for num in &rel_numbers {
        println!("{:?}", num);
    }
    //schem.print();
    println!("{:?}", schem.numbers[0]);
    println!("Determined that {} numbers are relevant.", rel_numbers.len());
    println!("Sum = {}", rel_numbers.iter().fold(0, |acc,x| acc+x.value));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = include_str!("../../input2.txt");
        let schem = Schematic::new(data.split('\n').map(|l| l.to_string()).collect());
        let rel_numbers = schem.get_relevant_numbers();
        assert_eq!(rel_numbers.len(), 8);
        assert_eq!(rel_numbers.iter().fold(0, |acc,x| acc+x.value), 4361);
    }
}
