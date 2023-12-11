use std::cmp::{min,max};

const EXPANSION: usize = 1_000_000;

fn find_empty_rows_and_cols(mut lines: Vec<String>) -> (Vec<usize>, Vec<usize>) {
    let mut expand_columns = vec![];
    let mut expand_lines = vec![];
    for (idx, ch) in lines.first().unwrap().char_indices() {
        let gal_in_column = lines
            .iter()
            .fold(0, |acc,l| acc + ((l.chars().nth(idx).unwrap() == '#') as usize));
        if gal_in_column == 0 {
            expand_columns.push(idx);
        }
    }
    for (idx,line) in lines.iter().enumerate() {
        if line.chars().all(|ch| ch == '.') {
            expand_lines.push(idx);
        }
    }

    println!("Expand columns: {:?}", expand_columns);
    println!("Expand lines: {:?}", expand_lines);

    (expand_columns, expand_lines)
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


    let galaxies = find_galaxies(&lines);

    println!("{:?}", galaxies);

    let empty_cols_rows = find_empty_rows_and_cols(lines);

    let mut total_dist = 0usize;
    for (idx, &coord) in galaxies.iter().enumerate() {
        if idx == galaxies.len()-1 {
            break;
        }
        total_dist += galaxies
            .iter()
            .skip(idx)
            .fold(0, |acc,&this_coord| {
                let col_between = empty_cols_rows.0
                    .iter()
                    .filter(|&&col| 
                        col > min(coord.0, this_coord.0) && col < max(coord.0, this_coord.0)
                    ).count();
                let row_between = empty_cols_rows.1
                    .iter()
                    .filter(|&&row| 
                        row > min(coord.1, this_coord.1) && row < max(coord.1, this_coord.1)
                    ).count();

                //println!("Between {:?} and {:?} there are {} empty cols and {} empty rows", coord, this_coord, col_between, row_between);

                let dx = (this_coord.0 as isize - coord.0 as isize).abs() as usize;
                let dy = (this_coord.1 as isize - coord.1 as isize).abs() as usize;
                let interp_dx = dx - col_between + (col_between * EXPANSION);
                let interp_dy = dy - row_between + (row_between * EXPANSION);

                //println!("-> dx = {}, dy = {}, interp_dx = {}, interp_dy = {}", dx, dy, interp_dx, interp_dy);

                acc + interp_dx + interp_dy
            })
    }

    println!("Total distance: {}", total_dist);
}
