#[derive(Debug)]
#[allow(dead_code)]
struct Arrangement {
    data: String,
    layout: Vec<usize>
}

impl Arrangement {
    fn new(s: &str) -> Self {
        let mut spl = s.split(' ');
        Self {
            data: spl.next().unwrap().to_string(),
            layout: spl.next().unwrap().split(',').map(|s| s.parse::<usize>().unwrap()).collect()
        }
    }
}

fn find_possible_permutations(
    data: &str, 
) -> Vec<String>
{
    if data.len() == 1 {
        match data.chars().nth(0).unwrap() {
            '?' => return vec![ "#".to_string(), ".".to_string() ],
            '#' => return vec![ "#".to_string() ],
            '.' => return vec![ '.'.to_string() ],
            _ => panic!("Cannot happen")
        }
    }
    let mut ret: Vec<String> = Vec::new();
    match data.chars().nth(0).unwrap() {
        '?' => {
            for s in find_possible_permutations(&data[1..]) {
                let mut new_str = String::from('#');
                new_str.push_str(&s);
                ret.push(new_str);
                let mut new_str = String::from('.');
                new_str.push_str(&s);
                ret.push(new_str);
            }
        },
        '#' => {
            for s in find_possible_permutations(&data[1..]) {
                let mut new_str = String::from('#');
                new_str.push_str(&s);
                ret.push(new_str);
            }
        },
        '.' => {
            for s in find_possible_permutations(&data[1..]) {
                let mut new_str = String::from('.');
                new_str.push_str(&s);
                ret.push(new_str);
            }
        },
        _ => panic!("Cannot happen")
    }

    ret
}

fn permutation_is_group(perm: &str, len: usize, need_beg_sep: bool, need_end_sep: bool) -> bool {
    if need_beg_sep && perm.chars().nth(0).unwrap() != '.' {
        return false;
    }
    if need_end_sep && perm.chars().nth(perm.len()-1).unwrap() != '.' {
        return false;
    }
    let mut idx_start = 0usize;
    while idx_start < perm.len() &&  perm.chars().nth(idx_start).unwrap() == '.' {
        idx_start += 1;
    }
    let mut idx_end = perm.len() - 1;
    while idx_end > 0 && perm.chars().nth(idx_end).unwrap() == '.' {
        idx_end -= 1;
    }
    perm[idx_start..idx_end].chars().all(|c| c == '#')
}

fn main() {
    let lines = include_str!("../../input2.txt").lines().collect::<Vec<&str>>();
    let mut arrangements = lines.iter().map(|l| Arrangement::new(l)).collect::<Vec<Arrangement>>();

    for a in arrangements {
        println!("{:?}", a.data);
        for p in find_possible_permutations(&a.data) {
            println!("  {:?}", p);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_arrangement() {
        let content = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    
        let v = content.lines().map(|l| Arrangement::new(l)).collect::<Vec<Arrangement>>();
     
        assert_eq!(v.len(), 6);
        assert_eq!(v[0].data, "???.###");
        assert_eq!(v[0].layout, vec![1,1,3]);
        assert_eq!(v[1].data, ".??..??...?##.");
        assert_eq!(v[1].layout, vec![1,1,3]);
        assert_eq!(v[2].data, "?#?#?#?#?#?#?#?");
        assert_eq!(v[2].layout, vec![1,3,1,6]);
        assert_eq!(v[3].data, "????.#...#...");
        assert_eq!(v[3].layout, vec![4,1,1]);
        assert_eq!(v[4].data, "????.######..#####.");
        assert_eq!(v[4].layout, vec![1,6,5]);
        assert_eq!(v[5].data, "?###????????");
        assert_eq!(v[5].layout, vec![3,2,1]);
    }

    #[test]
    fn test_find_possible_permutations() {
        let perm = find_possible_permutations("???");
        assert_eq!(perm.len(), 8);
        assert_eq!(perm[0], "###");
        assert_eq!(perm[1], ".##");
        assert_eq!(perm[2], "#.#");
        assert_eq!(perm[3], "..#");
        assert_eq!(perm[4], "##.");
        assert_eq!(perm[5], ".#.");
        assert_eq!(perm[6], "#..");
        assert_eq!(perm[7], "...");
    }

    #[test]
    fn test_permutation_is_group() {
        assert_eq!(permutation_is_group("###", 3, false, false), true);
        assert_eq!(permutation_is_group("###", 3, true, false), false);
        assert_eq!(permutation_is_group("###", 3, false, true), false);
        assert_eq!(permutation_is_group("###", 3, true, true), false);

    }

}
