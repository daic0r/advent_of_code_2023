use std::cell::RefCell;

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

fn calc_load(map: &Vec<RefCell<Vec<char>>>) -> usize {
    map
        .iter()
        .rev()
        .enumerate()
        .map(|(line_idx,l)| l.borrow().iter().filter(|&&ch| ch == 'O').count() * (line_idx + 1))
        .sum()
}

fn main() {
    let mut map = include_str!("../../input.txt")
        .lines()
        .map(|l| RefCell::new(l.chars().collect::<Vec<char>>()))
        .collect::<Vec<RefCell<Vec<char>>>>(); 

    tilt(&mut map);

    for line in map.iter().map(|l| l.borrow().iter().collect::<String>()) {
        println!("{line}");
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
            .fold(String::new(), |mut acc,x| {
                if !acc.is_empty() {
                    acc.push('\n');
                }
                acc.push_str(&x);
                acc
            }));

        assert_eq!(136, calc_load(&map));
    }
}
