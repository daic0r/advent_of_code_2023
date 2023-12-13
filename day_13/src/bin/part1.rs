fn find_reflection_idx(pattern: &Vec<&str>, horizontal: bool) -> Option<(usize,usize)> {
    let mut ret: Option<(usize, usize)> = None;
    if horizontal {
        let cnt = pattern.len();
        for (idx,&line) in pattern.iter().enumerate() {
            if idx == cnt - 1 {
                break; 
            }
            if *line == **pattern.get(idx+1).unwrap(){
                println!("horz: {} == {}", idx, idx+1);
                let mut cmp_idx = 1usize;
                let mut invalid = false;
                while cmp_idx+idx+1 < cnt && cmp_idx <= idx {
                    if *pattern.get(idx+1+cmp_idx).unwrap() != *pattern.get(idx-cmp_idx).unwrap() {
                        println!("horz: {} != {}", idx+1+cmp_idx, idx-cmp_idx);
                        invalid = true;
                        break;
                    }
                    if cmp_idx == idx {
                        cmp_idx += 1;
                        break;
                    }
                    cmp_idx += 1;
                }
                if invalid {
                    continue;
                }
                ret = Some((idx, cmp_idx));
                break;
            }
        }
    } else {
        let cnt = pattern.first().unwrap().len();
        let num_lines = pattern.len();
        for idx in 0..cnt {
            if idx == cnt - 1 {
                break; 
            }
            let ident_count = pattern.iter()
                .enumerate()
                .filter(|&(_, &l)| l.chars().nth(idx) == l.chars().nth(idx+1))
                .count();
            if ident_count == num_lines
            {
                let mut cmp_idx = 1usize;
                let mut invalid = false;
                while cmp_idx+idx+1 < cnt && cmp_idx <= idx {
                    if pattern.iter()
                                .enumerate()
                                .filter(|&(_, &l)| l.chars().nth(idx-cmp_idx) == l.chars().nth(idx+1+cmp_idx))
                                .count() != num_lines
                    {
                        println!("vert: {} != {}", idx+1+cmp_idx, idx-cmp_idx);
                        invalid = true;
                        break;
                    }
                    if cmp_idx == idx {
                        cmp_idx += 1;
                        break;
                    }
                    cmp_idx += 1;
                }
                if invalid {
                    continue;
                }
                ret = Some((idx, cmp_idx));
                break;
            }
        }

    }

    ret
}

fn main() {
    let patterns = include_str!("../../input.txt")
        .split("\n\n")
        .map(|pat_str| pat_str.lines().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut sum = 0usize;
    for pattern in patterns {
        for line in &pattern {
            println!("{}", line);
        }

        let ref_horz = find_reflection_idx(&pattern, true);
        let ref_vert = find_reflection_idx(&pattern, false);
        if let (Some(vert), Some(horz)) = (ref_vert, ref_horz) {
            panic!("Both reflections found: {:?}, {:?}", vert, horz);
        } else {
            if let Some(vert) = ref_vert {
                println!("Axis: vertical");
                let val = vert.0 + 1; 
                println!("{:?}, left: {}", vert, val);
                sum += val;
            }
            else
            if let Some(horz) = ref_horz {
                println!("Axis: horizontal");
                let val = (horz.0 + 1) * 100;
                println!("{:?}, above: {}", horz, val);
                sum += val;
            }
            else {
                panic!("No reflection found");
            }
        }
        println!("---------------------");
    }
    println!("Sum = {}", sum);
}
