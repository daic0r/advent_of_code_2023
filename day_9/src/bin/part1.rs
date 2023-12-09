use core::fmt;
use std::cell::RefCell;

#[derive(Debug, Clone)]
struct Sequence {
    values: Vec<i32>
}

impl Sequence {
    fn new(s: &str) -> Self {
        let mut ret = Self {
            values: vec![]
        };
        ret.values = s.split_whitespace().map(|str| str.parse::<i32>().unwrap()).collect();
        ret
    }

    fn derive(&self) -> Self {
        Self {
            values: self.values.windows(2).map(|arr| arr.last().unwrap() - arr.first().unwrap()).collect()
        }
    }

    fn extrapolate(&self) -> Self {
        let mut ders = vec![ RefCell::new(self.clone()) ];

        // Derive until we get a sequence of all 0's
        while !ders.last().unwrap().borrow().values.iter().all(|&n| n == 0) {
            let new_val = RefCell::new(ders.last().unwrap().borrow().derive());
            ders.push(new_val);
        }

        // Extrapolate new value
        for (idx,seq) in ders.iter().rev().skip(1).enumerate() {
            let prev_seq = ders.iter().rev().nth(idx).unwrap();
            let new_val = prev_seq.borrow().values.last().unwrap()
                    + seq.borrow().values.last().unwrap();
            seq.borrow_mut().values.push(new_val);
        }

        ders.into_iter().nth(0).unwrap().into_inner()
    }
}

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.values)
    }
}

fn main() {
   let lines = include_str!("../../input.txt").split('\n').filter(|l| !l.is_empty());

   let sum = lines.map(|l| *Sequence::new(l).extrapolate().values.last().unwrap()).sum::<i32>();

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
        let seq = Sequence::new("0 3 6 9 12 15");
        let der_seq = seq.derive();
        assert_eq!(der_seq.values.len(), 5);
        assert_eq!(der_seq.values, vec![3, 3, 3, 3, 3]);
    }
    #[test]
    fn test_extrapolate() {
        let seq = Sequence::new("0 3 6 9 12 15");
        let new = seq.extrapolate();
        assert_eq!(new.values.len(), seq.values.len() + 1);
        assert_eq!(new.values, vec![ 0, 3, 6, 9, 12, 15, 18 ]);

        let seq = Sequence::new("1 3 6 10 15 21");
        let new = seq.extrapolate();
        assert_eq!(new.values.len(), seq.values.len() + 1);
        assert_eq!(new.values, vec![ 1, 3, 6, 10, 15, 21, 28 ]);

        let seq = Sequence::new("10 13 16 21 30 45");
        let new = seq.extrapolate();
        assert_eq!(new.values.len(), seq.values.len() + 1);
        assert_eq!(new.values, vec![ 10, 13, 16, 21, 30, 45, 68 ]);
    }
}
