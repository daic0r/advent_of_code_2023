use std::{cell::RefCell, borrow::BorrowMut};

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
    for i in 0..4 {
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

fn main() {
    let mut map = include_str!("../../input2.txt")
        .lines()
        .map(|l| RefCell::new(l.chars().collect::<Vec<char>>()))
        .collect::<Vec<RefCell<Vec<char>>>>(); 

    for line in map.iter().map(|l| l.borrow().iter().collect::<String>()) {
        println!("{line}");
    }

    println!();
    map = cycle(map);

    for line in map.iter().map(|l| l.borrow().iter().collect::<String>()) {
        println!("{line}");
    }

    println!("Load = {}", calc_load(&map));

}

#[cfg(test)]
mod tests {
    use super::*;

    fn map_builder(mut acc: String, x: String) -> String {
        if !acc.is_empty() {
            acc.push('\n');
        }
        acc.push_str(&x);
        acc
    }

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
        assert_eq!(exp_result, map.iter().map(|l| l.borrow().iter().collect::<String>())
            .fold(String::new(), map_builder));

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
        assert_eq!(expect_cycle_1, map.iter().map(|l| l.borrow().iter().collect::<String>())
            .fold(String::new(), map_builder));
        map = cycle(map);
        assert_eq!(expect_cycle_2, map.iter().map(|l| l.borrow().iter().collect::<String>())
            .fold(String::new(), map_builder));
        map = cycle(map);
        assert_eq!(expect_cycle_3, map.iter().map(|l| l.borrow().iter().collect::<String>())
            .fold(String::new(), map_builder));
    }
}
