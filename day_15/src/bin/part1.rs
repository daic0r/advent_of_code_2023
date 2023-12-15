fn hash(s: &str) -> u8 {
    let mut cur_val = 0u32;
    for ch in s.bytes() {
        cur_val += ch as u32;
        cur_val *= 17;
        cur_val %= 256;
    }
    assert!(cur_val < 256);
    cur_val as u8
}

fn main() {
    let contents = include_str!("../../input.txt").replace("\n", "");

    let sum = contents.split(',').fold(0u32, |acc,x| acc + (hash(x) as u32));

    println!("Sum = {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("ot=9"), 9);
    }
}
