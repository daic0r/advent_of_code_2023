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

fn main() {

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
}
