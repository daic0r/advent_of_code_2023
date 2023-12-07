#![feature(const_option)]
//
// A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2.
#[derive(PartialEq,Debug)]
enum HandKind {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7
}
struct Hand {
    cards: Vec<char>,
    kind: HandKind
}

impl Hand {
    // ALL OF THESE ASSUME A SORTED SLICE!!
    fn is_five_of_a_kind(hand: &[char]) -> bool {
        hand.iter().filter(|c| *c == hand.first().unwrap()).count() == 5
    }
    fn is_four_of_a_kind(hand: &[char]) -> bool {
        hand.iter().filter(|c| *c == hand.first().unwrap()).count() == 4 ||
        hand.iter().filter(|c| *c == hand.last().unwrap()).count() == 4
    }
    fn is_three_of_a_kind(hand: &[char]) -> bool {
        hand.iter().filter(|c| *c == hand.first().unwrap()).count() == 3 ||
        hand.iter().filter(|c| *c == hand.last().unwrap()).count() == 3 ||
        hand.iter().filter(|c| *c == hand.get(1).unwrap()).count() == 3
    }
    fn is_full_house(hand: &[char]) -> bool {
        let count_first_group = hand.iter().filter(|c| *c == hand.first().unwrap()).count();
        let count_second_group = hand.iter().filter(|c| *c == hand.last().unwrap()).count();
        (count_first_group == 3 && count_second_group == 2) || (count_first_group == 2 && count_second_group == 3)
    }
    fn is_two_pair(hand: &[char]) -> bool {
        let count_first_group = hand.iter().filter(|c| *c == hand.first().unwrap()).count();
        let count_last_group = hand.iter().filter(|c| *c == hand.last().unwrap()).count();
        if count_first_group == 2 && count_last_group == 2 {
            return true;
        }
        else
        if (count_first_group == 2 || count_last_group == 2) && hand.iter().filter(|c| *c == hand.get(2).unwrap()).count() == 2 {
            return true;
        }
        false
    }
    fn is_one_pair(hand: &[char]) -> bool {
        hand.iter().filter(|c| *c == hand.get(1).unwrap()).count() == 2 ||
        hand.iter().filter(|c| *c == hand.get(2).unwrap()).count() == 2 ||
        hand.iter().filter(|c| *c == hand.get(3).unwrap()).count() == 2 ||
        hand.iter().filter(|c| *c == hand.get(4).unwrap()).count() == 2
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
        ret.cards.sort_by_key(|c| 14-points(*c));
        if Hand::is_five_of_a_kind(&ret.cards) {
            ret.kind = HandKind::FiveOfAKind;
        }
        else
        if Hand::is_four_of_a_kind(&ret.cards) {
            ret.kind = HandKind::FourOfAKind;
        }
        if Hand::is_full_house(&ret.cards) {
            ret.kind = HandKind::FullHouse;
        }
        else
        if Hand::is_three_of_a_kind(&ret.cards) {
            ret.kind = HandKind::ThreeOfAKind;
        }
        else
        if Hand::is_two_pair(&ret.cards) {
            ret.kind = HandKind::TwoPair;
        }
        else
        if Hand::is_one_pair(&ret.cards) {
            ret.kind = HandKind::OnePair;
        }
        ret
    }
}

const fn points(c: char) -> u32 {
    match c {
        '2'..='9' => c.to_digit(10).unwrap(),
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_from_str() {
        let hand = Hand::new("32T3K");
        assert_eq!(hand.cards, vec![ 'K', 'T', '3', '3', '2' ]);
    }

    #[test]
    fn test_5_of_a_kind() {
        let hand = Hand::new("AAAAA");
        assert_eq!(hand.kind, HandKind::FiveOfAKind);
    }

    #[test]
    fn test_4_of_a_kind() {
        let hand = Hand::new("AAAAK");
        assert_eq!(hand.kind, HandKind::FourOfAKind);
        let hand = Hand::new("QQKQQ");
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
}
