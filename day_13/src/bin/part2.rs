fn cmp_str_tolerant(a: &str, b: &str, was_tolerant: &mut bool) -> bool {
    let ret = a.chars().zip(b.chars()).filter(|&(a,b)| a != b).count();
    if ret == 1 {
        *was_tolerant = true;
    }
    ret <= 1
}

fn cmp_columns_tolerant(v: &Vec<&str>, col_a: usize, col_b: usize, was_tolerant: &mut bool) -> bool {
    let cnt = v.iter()
            .enumerate()
            .filter(|&(_, &l)| l.chars().nth(col_a) == l.chars().nth(col_b))
            .count();
    if cnt == v.len() - 1 {
        *was_tolerant = true;
    }
    cnt >= v.len() - 1
}

fn find_reflection_idx(pattern: &Vec<&str>, horizontal: bool) -> Option<(usize,usize)> {
    let mut ret: Option<(usize, usize)> = None;
    if horizontal {
        let cnt = pattern.len();
        for (idx,&line) in pattern.iter().enumerate() {
            if idx == cnt - 1 {
                break; 
            }
            let mut was_tolerant = false;
            if cmp_str_tolerant(line, *pattern.get(idx+1).unwrap(), &mut was_tolerant) {
                println!("horz: {} == {}", idx, idx+1);
                let mut cmp_idx = 1usize;
                let mut invalid = false;
                while cmp_idx+idx+1 < cnt && cmp_idx <= idx {
                    if !cmp_str_tolerant(pattern.get(idx+1+cmp_idx).unwrap(), *pattern.get(idx-cmp_idx).unwrap(), &mut was_tolerant) {
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
                if invalid || !was_tolerant {
                    continue;
                }
                ret = Some((idx, cmp_idx));
                break;
            }
        }
    } else {
        let cnt = pattern.first().unwrap().len();
        for idx in 0..cnt {
            if idx == cnt - 1 {
                break; 
            }
            let mut was_tolerant = false;
            if cmp_columns_tolerant(pattern, idx, idx+1, &mut was_tolerant)
            {
                let mut cmp_idx = 1usize;
                let mut invalid = false;
                while cmp_idx+idx+1 < cnt && cmp_idx <= idx {
                    if !cmp_columns_tolerant(pattern, idx-cmp_idx, idx+1+cmp_idx, &mut was_tolerant)
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
                if invalid || !was_tolerant {
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
        if let (Some(_), Some(_)) = (ref_vert, ref_horz) {
            panic!("Both reflections found");
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
