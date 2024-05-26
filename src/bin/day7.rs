use std::cmp::Ordering;

use itertools::Itertools;
use winnow::{
    ascii::{dec_int, space1},
    prelude::*,
    token::any,
};

fn main() {
    let input = std::fs::read_to_string("inputs/day7").unwrap();
    let hands = input
        .trim()
        .lines()
        .map(|s| Hand::parser.parse(s).unwrap())
        .collect_vec();
    println!("Part 1: {}", solve_part1(hands.clone()));
    println!("Part 2: {}", solve_part2(hands));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn parser(i: &mut &str) -> PResult<Card> {
        any.verify_map(|c| match c {
            '2' => Some(Card::N2),
            '3' => Some(Card::N3),
            '4' => Some(Card::N4),
            '5' => Some(Card::N5),
            '6' => Some(Card::N6),
            '7' => Some(Card::N7),
            '8' => Some(Card::N8),
            '9' => Some(Card::N9),
            'T' => Some(Card::T),
            'J' => Some(Card::J),
            'Q' => Some(Card::Q),
            'K' => Some(Card::K),
            'A' => Some(Card::A),
            _ => None,
        })
        .parse_next(i)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand<C> {
    cards: [C; 5],
    bid: i32,
}

impl Hand<Card> {
    #[allow(unused)]
    fn new(cards: [Card; 5], bid: i32) -> Self {
        Self { cards, bid }
    }
    fn parser(i: &mut &str) -> PResult<Self> {
        let cp = Card::parser;
        let cards = (cp, cp, cp, cp, cp).parse_next(i)?.into();
        space1.parse_next(i)?;
        let bid = dec_int.parse_next(i)?;
        Ok(Hand { cards, bid })
    }
}

fn solve_part1(mut hands: Vec<Hand<Card>>) -> i32 {
    hands.sort_unstable_by_key(|h| (classify_by(group_cards, h.cards), h.cards));
    hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| (i as i32 + 1) * h.bid)
        .sum()
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Jokered<C>(C);

impl PartialOrd for Jokered<Card> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Jokered<Card> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.0, other.0) {
            (Card::J, Card::J) => Ordering::Equal,
            (Card::J, _) => Ordering::Less,
            (_, Card::J) => Ordering::Greater,
            (a, b) => a.cmp(&b),
        }
    }
}

fn solve_part2(mut hands: Vec<Hand<Card>>) -> i32 {
    hands.sort_unstable_by_key(|h| {
        (
            classify_by(group_with_jokers, h.cards),
            h.cards.map(Jokered),
        )
    });
    hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| (i as i32 + 1) * h.bid)
        .sum()
}

fn group_cards(mut cards: [Card; 5]) -> Vec<usize> {
    cards.sort_unstable();
    let mut counts = cards
        .into_iter()
        .dedup_with_count()
        .map(|(n, _card)| n)
        .collect_vec();
    counts.sort_unstable();
    counts
}

fn group_with_jokers(mut cards: [Card; 5]) -> Vec<usize> {
    cards.sort_unstable();
    let mut jokers = 0;
    let mut others = cards
        .into_iter()
        .dedup_with_count()
        .filter_map(|(n, c)| {
            if c == Card::J {
                jokers += n;
                None
            } else {
                Some(n)
            }
        })
        .collect_vec();
    if others.is_empty() {
        vec![jokers]
    } else {
        others.sort_unstable();
        *others.last_mut().unwrap() += jokers;
        others
    }
}

fn classify_by<C, F>(mut group_counts: F, cards: [C; 5]) -> HandType
where
    F: FnMut([C; 5]) -> Vec<usize>,
    C: Ord,
{
    let counts = group_counts(cards);
    match &counts[..] {
        [5] => HandType::FiveOfAKind,
        [1, 4] => HandType::FourOfAKind,
        [2, 3] => HandType::FullHouse,
        [1, 1, 3] => HandType::ThreeOfAKind,
        [1, 2, 2] => HandType::TwoPair,
        [1, 1, 1, 2] => HandType::OnePair,
        [1, 1, 1, 1, 1] => HandType::HighCard,
        _ => panic!("classify: unexpected grouping: {counts:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_classify() {
        use Card::*;
        assert_eq!(
            classify_by(group_cards, [N3, N2, T, N3, K]),
            HandType::OnePair
        );
        assert_eq!(
            classify_by(group_cards, [K, K, N6, N7, N7]),
            HandType::TwoPair
        );
        assert_eq!(classify_by(group_cards, [K, T, J, J, T]), HandType::TwoPair);
        assert_eq!(
            classify_by(group_cards, [T, N5, N5, J, N5]),
            HandType::ThreeOfAKind
        );
        assert_eq!(
            classify_by(group_cards, [K, K, K, K, K]),
            HandType::FiveOfAKind
        );
    }

    #[test]
    fn can_parse_hand() {
        use Card::*;
        let hands = INPUT
            .lines()
            .take(2)
            .map(|l| Hand::parser.parse(l))
            .collect_vec();
        let expected = [
            Ok(Hand::new([N3, N2, T, N3, K], 765)),
            Ok(Hand::new([T, N5, N5, J, N5], 684)),
        ];
        assert_eq!(hands, expected);
    }

    #[test]
    fn can_solve_part1() {
        let hands = INPUT
            .lines()
            .map(|s| Hand::parser.parse(s).unwrap())
            .collect_vec();
        assert_eq!(solve_part1(hands), 6440);
    }

    #[test]
    fn can_solve_part2() {
        let hands = INPUT
            .lines()
            .map(|s| Hand::parser.parse(s).unwrap())
            .collect_vec();
        assert_eq!(solve_part2(hands), 5905);
    }

    const INPUT: &str = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
}
