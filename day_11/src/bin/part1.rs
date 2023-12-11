fn expand_space(mut lines: Vec<String>) -> Vec<String> {
    let mut expand_columns = vec![];
    let mut expand_lines = vec![];
    for (idx, ch) in lines.first().unwrap().char_indices() {
        let gal_in_column = lines
            .iter()
            .fold(0, |acc,l| acc + ((l.chars().nth(idx).unwrap() == '#') as usize));
        if gal_in_column == 0 {
            expand_columns.push(idx + expand_columns.len());
        }
    }
    for (idx,line) in lines.iter().enumerate() {
        if line.chars().all(|ch| ch == '.') {
            expand_lines.push(idx + expand_lines.len());
        }
    }
    for idx in &expand_lines {
        lines.insert(*idx, lines.iter().nth(*idx).unwrap().clone());
    }
    for line in &mut lines {
        for &idx in &expand_columns {
            line.insert(idx, '.');
        }
    }

    lines
}

fn find_galaxies(v: &Vec<String>) -> Vec<(usize,usize)> {
    let mut ret = vec![];

    for (idx,line) in v.iter().enumerate() {
        let mut line_galaxies = line
            .char_indices()
            .filter(|(_, ch)| *ch == '#')
            .map(move |(col_idx,_)| {
                (col_idx, idx)
            }).collect::<Vec<(usize,usize)>>();
        ret.append(&mut line_galaxies);
    }

    ret
}

fn main() {
    let mut lines = include_str!("../../input.txt").lines().map(|l| l.to_string()).collect::<Vec<String>>();

    lines = expand_space(lines);

    for line in &lines {
        println!("{}", line);
    }

    let galaxies = find_galaxies(&lines);

    println!("{:?}", galaxies);

    let mut total_dist = 0usize;
    for (idx, coord) in galaxies.iter().enumerate() {
        if idx == galaxies.len()-1 {
            break;
        }
        total_dist += galaxies
            .iter()
            .skip(idx)
            .fold(0, |acc,&this_coord| 
                acc + ((this_coord.0 as isize - coord.0 as isize).abs() as usize) +
                      ((this_coord.1 as isize - coord.1 as isize).abs() as usize));
    }

    println!("Total distance: {}", total_dist);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_expand() {
        let lines = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let expected = r"....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";
        
        let lines = lines.lines().map(|l| l.to_string()).collect::<Vec<String>>();
        let expected = expected.lines().map(|l| l.to_string()).collect::<Vec<String>>();
        assert_eq!(expand_space(lines), expected);
    }

    #[test]
    fn test_find_galaxies() {
        let expected = r"....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";
        let expected = expected.lines().map(|l| l.to_string()).collect::<Vec<String>>();

        let galaxies = find_galaxies(&expected);

        assert_eq!(galaxies, [(4, 0), (9, 1), (0, 2), (8, 5), (1, 6), (12, 7), (9, 10), (0, 11), (5, 11)]);
    }
}
