use std::{cell::RefCell, borrow::BorrowMut};
use std::hash::{Hasher,Hash};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;

fn map_builder(mut acc: String, x: String) -> String {
    if !acc.is_empty() {
        acc.push('\n');
    }
    acc.push_str(&x);
    acc
}

fn string_from_map(map: &Vec<RefCell<Vec<char>>>) -> String {
    map.iter().map(|l| l.borrow().iter().collect::<String>())
        .fold(String::new(), map_builder)
}

fn transpose(map: Vec<RefCell<Vec<char>>>) -> Vec<RefCell<Vec<char>>> {
    let ret = map.clone();
    for (row, l) in map.iter().enumerate() {
        for (col, ch) in l.borrow().iter().enumerate() {
            *ret.get(col).unwrap().borrow_mut().get_mut(map.len() - 1 - row).unwrap() = *ch;
        }
    }
    ret
}

fn tilt(map: &mut Vec<RefCell<Vec<char>>>) {
    for row in 1..map.len()-1 {
        for (idx, line) in map.iter().skip(row).enumerate() {
            for (col, ch) in line.borrow_mut().iter_mut().enumerate() {
                if *ch != 'O' {
                    continue;
                }
                let mut dst_row: isize = (idx + row - 1) as isize;
                while dst_row >= 0 && *map.get(dst_row as usize).unwrap().borrow().get(col).unwrap() == '.' {
                    dst_row -= 1;
                }
                // We went one beyond due to the final += 1 above
                dst_row += 1;
                if dst_row as usize == row + idx {
                    continue;
                }
                *map.get(dst_row as usize).unwrap().borrow_mut().get_mut(col).unwrap() = 'O';
                *ch = '.';
            }
        }
    }
}

fn cycle(mut map: Vec<RefCell<Vec<char>>>) -> Vec<RefCell<Vec<char>>> {
    for _ in 0..4 {
        tilt(&mut map);
        map = transpose(map);
    }
    map
}

fn calc_load(map: &Vec<RefCell<Vec<char>>>) -> usize {
    map
        .iter()
        .rev()
        .enumerate()
        .map(|(line_idx,l)| l.borrow().iter().filter(|&&ch| ch == 'O').count() * (line_idx + 1))
        .sum()
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn main() {
    let mut map = include_str!("../../input.txt")
        .lines()
        .map(|l| RefCell::new(l.chars().collect::<Vec<char>>()))
        .collect::<Vec<RefCell<Vec<char>>>>(); 

    println!("ORIGINAL MAP:");
    for line in map.iter().map(|l| l.borrow().iter().collect::<String>()) {
        println!("{line}");
    }

    println!();
    let mut hashes = HashSet::<(u64, usize)>::new();
    let mut cycle_cnt = 0usize;
    let mut tmp = map.clone();
    let cycle_len;
    let cycle_start;
    println!("Trying to detect cycle...");
    loop {
        tmp = cycle(tmp);
        cycle_cnt += 1;
        let hash = calculate_hash(&string_from_map(&tmp));
        if let Some(hash) = hashes.iter().find(|(h, _)| *h == hash) {
            cycle_len = cycle_cnt - hash.1;
            cycle_start = hash.1;
            println!("Cycle of length {} starting at {} detected.", cycle_len, cycle_start);
            break;
        }
        hashes.insert((hash, cycle_cnt));
    };

    let effective_cnt = (1_000_000_000 - cycle_start) % cycle_len;

    println!("Number of times to run after entering cycle = {}", effective_cnt);

    for i in 0..cycle_start+effective_cnt {
        map = cycle(map);
    }

    println!("Load = {}", calc_load(&map));

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input_str = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let mut map = input_str
            .lines()
            .map(|l| RefCell::new(l.chars().collect::<Vec<char>>()))
            .collect::<Vec<RefCell<Vec<char>>>>(); 

        let mut copy_map = map.clone();
        for i in 0..4 {
            copy_map = transpose(copy_map);
        }
        assert_eq!(copy_map, map);
        tilt(&mut map);

        let exp_result = r"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";
        assert_eq!(exp_result, string_from_map(&map));

        assert_eq!(136, calc_load(&map));
    }

    #[test]
    fn test_cycle() {
        let input_str = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let mut map = input_str
            .lines()
            .map(|l| RefCell::new(l.chars().collect::<Vec<char>>()))
            .collect::<Vec<RefCell<Vec<char>>>>(); 

        let expect_cycle_1 = r".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";
        let expect_cycle_2 = r".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O";
        let expect_cycle_3 = r".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O";

        map = cycle(map);
        assert_eq!(expect_cycle_1, string_from_map(&map));
        map = cycle(map);
        assert_eq!(expect_cycle_2, string_from_map(&map));
        map = cycle(map);
        assert_eq!(expect_cycle_3, string_from_map(&map));
    }
}
