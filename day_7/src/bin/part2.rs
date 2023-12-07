#![feature(const_option)]

use std::cmp::Ordering;
//
// A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2.
#[derive(PartialEq,PartialOrd, Ord, Eq, Debug, Clone)]
enum HandKind {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7
}

#[derive(Clone, Debug)]
struct Hand {
    cards: Vec<char>,
    kind: HandKind
}

impl Hand {
    fn is_joker(c: char) -> bool {
        c == 'J'
    }
    fn joker_cnt(s: &[char]) -> usize {
        s.iter().filter(|c| **c == 'J').count()
    }
    // ALL OF THESE ASSUME A SORTED SLICE!!
    fn determine_jokered_kind(hand: &[char]) -> HandKind {
        match hand.len() {
            0..=1 => HandKind::FiveOfAKind,
            2 => {
                if hand.first().unwrap() == hand.last().unwrap() {
                    HandKind::FiveOfAKind
                } else {
                    HandKind::FourOfAKind
                }
            },
            3 => {
                if hand.iter().filter(|c| *c == hand.first().unwrap()).count() == 3 {
                    HandKind::FiveOfAKind
                }
                else
                // 2 identical
                if hand.first().unwrap() == hand.get(1).unwrap() || hand.last().unwrap() == hand.get(1).unwrap() {
                    HandKind::FourOfAKind
                }
                else {
                    HandKind::ThreeOfAKind
                }
            },
            4 => {
                if hand.iter().filter(|c| *c == hand.first().unwrap()).count() == 4 {
                    HandKind::FiveOfAKind
                }
                else
                // 3 identical
                if hand.first().unwrap() == hand.get(2).unwrap() || hand.last().unwrap() == hand.get(1).unwrap() {
                    HandKind::FourOfAKind
                }
                else
                // 2 identical
                if hand.first().unwrap() == hand.get(1).unwrap() || hand.last().unwrap() == hand.get(2).unwrap() {
                    HandKind::ThreeOfAKind
                }
                else {
                    HandKind::OnePair
                }
            },
            _ => HandKind::HighCard
        }
    }
    fn is_five_of_a_kind(hand: &[char]) -> bool {
        let jc = Self::joker_cnt(hand);
        if jc == 0 {
            hand.iter().filter(|c| *c == hand.first().unwrap()).count() == 5
        }
        else {
            Self::determine_jokered_kind(&hand[..hand.len()-jc]) == HandKind::FiveOfAKind
        }
    }
    fn is_four_of_a_kind(hand: &[char]) -> bool {
        let jc = Self::joker_cnt(hand);
        if jc == 0 {
            hand.iter().filter(|c| *c == hand.first().unwrap()).count() == 4 ||
            hand.iter().filter(|c| *c == hand.last().unwrap()).count() == 4
        }
        else {
            Self::determine_jokered_kind(&hand[..hand.len()-jc]) == HandKind::FourOfAKind
        }
    }
    fn is_three_of_a_kind(hand: &[char]) -> bool {
        let jc = Self::joker_cnt(hand);
        if jc == 0 {
            hand.iter().filter(|c| *c == hand.first().unwrap() ||Self::is_joker(**c)).count() == 3 ||
                hand.iter().filter(|c| *c == hand.last().unwrap() ||Self::is_joker(**c)).count() == 3 ||
                hand.iter().filter(|c| *c == hand.get(1).unwrap() ||Self::is_joker(**c)).count() == 3
        }
        else {
            Self::determine_jokered_kind(&hand[..hand.len()-jc]) == HandKind::ThreeOfAKind
        }
    }
    fn is_full_house(hand: &[char]) -> bool {
        let jc = Self::joker_cnt(hand);
        if jc > 0 {
            return false;
        }
        let count_first_group = hand.iter().filter(|c| *c == hand.first().unwrap() ||Self::is_joker(**c)).count();
        let count_second_group = hand.iter().filter(|c| *c == hand.last().unwrap() ||Self::is_joker(**c)).count();
        (count_first_group == 3 && count_second_group == 2) || (count_first_group == 2 && count_second_group == 3)
    }
    fn is_two_pair(hand: &[char]) -> bool {
        let jc = Self::joker_cnt(hand);
        if jc > 0 {
            return false;
        }
        let count_first_group = hand.iter().filter(|c| *c == hand.first().unwrap() ||Self::is_joker(**c)).count();
        let count_last_group = hand.iter().filter(|c| *c == hand.last().unwrap() ||Self::is_joker(**c)).count();
        if count_first_group == 2 && count_last_group == 2 {
            return true;
        }
        else
        if (count_first_group == 2 || count_last_group == 2) && hand.iter().filter(|c| *c == hand.get(2).unwrap() ||Self::is_joker(**c)).count() == 2 {
            return true;
        }
        false
    }
    fn is_one_pair(hand: &[char]) -> bool {
        let jc = Self::joker_cnt(hand);
        if jc == 0 {
            hand.iter().filter(|c| *c == hand.get(1).unwrap() || Self::is_joker(**c)).count() == 2 ||
                hand.iter().filter(|c| *c == hand.get(2).unwrap() || Self::is_joker(**c)).count() == 2 ||
                hand.iter().filter(|c| *c == hand.get(3).unwrap() || Self::is_joker(**c)).count() == 2 ||
                hand.iter().filter(|c| *c == hand.get(4).unwrap() || Self::is_joker(**c)).count() == 2
        }
        else {
            Self::determine_jokered_kind(&hand[..hand.len()-jc]) == HandKind::OnePair
        }
    }

    fn new(s: &str) -> Self {
        assert_eq!(s.len(), 5);
        let mut ret = Hand {
            cards: vec![],
            kind: HandKind::HighCard
        };
        for ch in s.chars() {
            ret.cards.push(ch);
        }
        let mut sorted = ret.cards.clone();
        sorted.sort_by_key(|c| 14-points(*c, false));

        let cnt = Self::joker_cnt(&sorted);
        if cnt > 0 {
            assert!(&sorted[sorted.len()-cnt..].iter().all(|c| *c == 'J'));
        }

        if Hand::is_five_of_a_kind(&sorted) {
            ret.kind = HandKind::FiveOfAKind;
        }
        else
        if Hand::is_four_of_a_kind(&sorted) {
            ret.kind = HandKind::FourOfAKind;
        }
        else
        if Hand::is_full_house(&sorted) {
            ret.kind = HandKind::FullHouse;
        }
        else
        if Hand::is_three_of_a_kind(&sorted) {
            ret.kind = HandKind::ThreeOfAKind;
        }
        else
        if Hand::is_two_pair(&sorted) {
            ret.kind = HandKind::TwoPair;
        }
        else
        if Hand::is_one_pair(&sorted) {
            ret.kind = HandKind::OnePair;
        }
        ret
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.kind == other.kind
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.kind.partial_cmp(&other.kind) {
            Some(ord) if ord != Ordering::Equal => Some(ord),
            _ => {
                for (ch, ch2) in self.cards.iter().zip(other.cards.iter()) {
                    if points(*ch, true) > points(*ch2, true) {
                        return Some(Ordering::Greater);
                    }
                    else if points(*ch, true) < points(*ch2, true) {
                        return Some(Ordering::Less);
                    }
                }
                Some(Ordering::Equal)
            }
        }
    }
}

impl Eq for Hand {

}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind.cmp(&other.kind) {
            ord if ord != Ordering::Equal => ord,
            _ => {
                for (ch, ch2) in self.cards.iter().zip(other.cards.iter()) {
                    if points(*ch, true) > points(*ch2, true) {
                        println!("{} > {}", ch, ch2);
                        return Ordering::Greater;
                    }
                    else if points(*ch, true) < points(*ch2, true) {
                        println!("{} < {}", ch, ch2);
                        return Ordering::Less;
                    }
                }
                Ordering::Equal
            }
        }
    }
}

const fn points(c: char, for_sorting: bool) -> u32 {
    match c {
        'J' if !for_sorting => 1,
        'J' if for_sorting => 11,
        '2'..='9' => c.to_digit(10).unwrap(),
        'T' => 10,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0
    }
}

fn main() {
    let contents = include_str!("../../input.txt");
    let lines = contents.split('\n').filter(|l| !l.is_empty());

    let mut hands_and_bids = vec![];
    for line in lines {
        let mut splits = line.split_whitespace();
        let hand = Hand::new(splits.next().unwrap());
        let bid = splits.next().unwrap().parse::<u32>().unwrap();
        hands_and_bids.push((hand,bid));
    }
    hands_and_bids.sort_by_key(|h| h.0.clone());
    for (idx, (hand, bid)) in hands_and_bids.iter().enumerate() {
        println!("{:?} {} {}*{}", hand.kind, String::from_iter(&hand.cards), bid, idx+1);
    }
    let sum = hands_and_bids
        .iter()
        .enumerate()
        .fold(0, |acc,x| acc+((x.0 as u32 + 1) * x.1.1));
    println!("Sum = {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_from_str() {
        let hand = Hand::new("32T3K");
        assert_eq!(hand.cards, vec![ '3', '2', 'T', '3', 'K' ]);
    }

    #[test]
    fn test_5_of_a_kind() {
        let hand = Hand::new("AAAAA");
        assert_eq!(hand.kind, HandKind::FiveOfAKind);
        let hand = Hand::new("AJJJJ");
        assert_eq!(hand.kind, HandKind::FiveOfAKind);
    }

    #[test]
    fn test_4_of_a_kind() {
        let hand = Hand::new("AAAAK");
        assert_eq!(hand.kind, HandKind::FourOfAKind);
        let hand = Hand::new("QQKQQ");
        assert_eq!(hand.kind, HandKind::FourOfAKind);
        let hand = Hand::new("T55J5");
        let mut v = hand.cards.clone();
        v.sort_by_key(|c| 14-points(*c, false));
        assert_eq!(v, vec![ 'T', '5', '5', '5', 'J' ]);
        assert_eq!(hand.kind, HandKind::FourOfAKind);

        let hand = Hand::new("QJJQ2");
        assert_eq!(hand.kind, HandKind::FourOfAKind);
    }

    #[test]
    fn test_3_of_a_kind() {
        let hand = Hand::new("2232K");
        assert_eq!(hand.kind, HandKind::ThreeOfAKind);
        let hand = Hand::new("4K5KK");
        assert_eq!(hand.kind, HandKind::ThreeOfAKind);
        let hand = Hand::new("34544");
        assert_eq!(hand.kind, HandKind::ThreeOfAKind);
    }

    #[test]
    fn test_full_house() {
        let hand = Hand::new("5K5K5");
        assert_eq!(hand.kind, HandKind::FullHouse);
        let hand = Hand::new("K5K5K");
        assert_eq!(hand.kind, HandKind::FullHouse);
    }

    #[test]
    fn test_two_pair() {
        let hand = Hand::new("22344");
        assert_eq!(hand.kind, HandKind::TwoPair);
        let hand = Hand::new("22334");
        assert_eq!(hand.kind, HandKind::TwoPair);
        let hand = Hand::new("23344");
        assert_eq!(hand.kind, HandKind::TwoPair);
    }

    #[test]
    fn test_pair() {
        let hand = Hand::new("22KQA");
        assert_eq!(hand.kind, HandKind::OnePair);
        let hand = Hand::new("K22QA");
        assert_eq!(hand.kind, HandKind::OnePair);
        let hand = Hand::new("KQ22A");
        assert_eq!(hand.kind, HandKind::OnePair);
        let hand = Hand::new("KQA22");
        assert_eq!(hand.kind, HandKind::OnePair);
    }

    #[test]
    fn test_order() {
        let hand = Hand::new("AAAAA");
        let hand2 = Hand::new("AAAAK");
        assert!(hand > hand2);
        let hand = Hand::new("AAAAA");
        let hand2 = Hand::new("KKKKK");
        assert!(hand > hand2);

        let hand = Hand::new("33332");
        let hand2 = Hand::new("2AAAA");
        assert_eq!(hand.kind, HandKind::FourOfAKind);
        assert_eq!(hand2.kind, HandKind::FourOfAKind);
        assert!(hand > hand2);

        let hand = Hand::new("JKKK2");
        let hand2 = Hand::new("QQQQ2");
        assert_eq!(hand.kind, HandKind::FourOfAKind);
        assert_eq!(hand2.kind, HandKind::FourOfAKind);
        assert!(hand < hand2);

        assert!(HandKind::FiveOfAKind > HandKind::FourOfAKind);
        assert!(HandKind::FourOfAKind > HandKind::FullHouse);
        assert!(HandKind::FullHouse > HandKind::ThreeOfAKind);
        assert!(HandKind::ThreeOfAKind > HandKind::TwoPair);
        assert!(HandKind::TwoPair > HandKind::OnePair);
        assert!(HandKind::OnePair > HandKind::HighCard);

        assert!(HandKind::FiveOfAKind > HandKind::HighCard);
        assert!(HandKind::FiveOfAKind > HandKind::OnePair);
        assert!(HandKind::FiveOfAKind > HandKind::TwoPair);
        assert!(HandKind::FiveOfAKind > HandKind::ThreeOfAKind);
        assert!(HandKind::FiveOfAKind > HandKind::FullHouse);
        assert!(HandKind::FiveOfAKind > HandKind::FourOfAKind);

        assert!(HandKind::FourOfAKind > HandKind::HighCard);
        assert!(HandKind::FourOfAKind > HandKind::OnePair);
        assert!(HandKind::FourOfAKind > HandKind::TwoPair);
        assert!(HandKind::FourOfAKind > HandKind::ThreeOfAKind);
        assert!(HandKind::FourOfAKind > HandKind::FullHouse);

        assert!(HandKind::FullHouse > HandKind::HighCard);
        assert!(HandKind::FullHouse > HandKind::OnePair);
        assert!(HandKind::FullHouse > HandKind::TwoPair);
        assert!(HandKind::FullHouse > HandKind::ThreeOfAKind);

        assert!(HandKind::ThreeOfAKind > HandKind::OnePair);
        assert!(HandKind::ThreeOfAKind > HandKind::TwoPair);
        assert!(HandKind::ThreeOfAKind > HandKind::HighCard);
        
        assert!(HandKind::TwoPair > HandKind::OnePair);
        assert!(HandKind::TwoPair > HandKind::HighCard);

        assert!(HandKind::OnePair > HandKind::HighCard);
    }
}
