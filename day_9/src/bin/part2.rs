use core::fmt;
use std::{cell::RefCell, ops::Deref};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Sequence {
    values: VecDeque<i128>
}

impl Sequence {
    fn new(s: &str) -> Self {
        let mut ret = Self {
            values: VecDeque::new()
        };
        ret.values = s.split_whitespace().map(|str| str.parse::<i128>().unwrap()).collect();
        ret
    }

    fn derive(&mut self) -> Self {
        Self {
            values: self.values.make_contiguous().windows(2).map(|arr| arr.last().unwrap() - arr.first().unwrap()).collect()
        }
    }

    fn all_derivatives(&mut self) -> Vec<RefCell<Self>> {
        let mut ders = vec![ RefCell::new(self.clone()) ];

        // Derive until we get a sequence of all 0's
        while !ders.last().unwrap().borrow().values.iter().all(|&n| n == 0) {
            let new_val = RefCell::new(ders.last().unwrap().borrow_mut().derive());
            ders.push(new_val);
        }

        ders
    }

    fn extrapolate_impl(&mut self, 
        f: fn(&Sequence, &Sequence) -> i128,
        apply_new_val: fn(&mut Sequence, i128) -> ()
    ) -> Self {
        let ders = self.all_derivatives();
        // Extrapolate new value
        for (idx,seq) in ders.iter().rev().skip(1).enumerate() {
            let prev_seq = ders.iter().rev().nth(idx).unwrap();
            let new_val = f(prev_seq.borrow().deref(), seq.borrow().deref());
            apply_new_val(&mut seq.borrow_mut(), new_val);
        }

        let ret = ders.first().unwrap().borrow();
        println!("{}", ret);
        ret.clone()
    }

    #[allow(dead_code)]
    fn extrapolate(&mut self) -> Self {
        self.extrapolate_impl(
            |prev_seq, seq| prev_seq.values.back().unwrap() + seq.values.back().unwrap(),
            |seq, val| seq.values.push_back(val)
        )
    }
    fn extrapolate_back(&mut self) -> Self {
        self.extrapolate_impl(
            |prev_seq, seq| seq.values.front().unwrap() - prev_seq.values.front().unwrap() ,
            |seq, val| seq.values.push_front(val)
        )
    }
}

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.values)
    }
}

fn main() {
   let lines = include_str!("../../input.txt").split('\n').filter(|l| !l.is_empty());

   let sum = lines.map(|l| *Sequence::new(l).extrapolate_back().values.front().unwrap()).sum::<i128>();

   println!("Sum = {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sequence() {
        let seq = Sequence::new("0 3 6 9 12 15");
        assert_eq!(seq.values, vec![0, 3, 6, 9, 12, 15]); 
    }
    #[test]
    fn test_derive_sequence() {
        let mut seq = Sequence::new("0 3 6 9 12 15");
        let der_seq = seq.derive();
        assert_eq!(der_seq.values.len(), 5);
        assert_eq!(der_seq.values, vec![3, 3, 3, 3, 3]);
    }
    #[test]
    fn test_extrapolate() {
        let mut seq = Sequence::new("0 3 6 9 12 15");
        let new = seq.extrapolate_back();
        assert_eq!(new.values.len(), seq.values.len() + 1);
        assert_eq!(new.values, vec![ -3, 0, 3, 6, 9, 12, 15 ]);

        let mut seq = Sequence::new("1 3 6 10 15 21");
        let new = seq.extrapolate_back();
        assert_eq!(new.values.len(), seq.values.len() + 1);
        assert_eq!(new.values, vec![ 0, 1, 3, 6, 10, 15, 21 ]);

        let mut seq = Sequence::new("10 13 16 21 30 45");
        let new = seq.extrapolate_back();
        assert_eq!(new.values.len(), seq.values.len() + 1);
        assert_eq!(new.values, vec![ 5, 10, 13, 16, 21, 30, 45 ]);
    }
}
