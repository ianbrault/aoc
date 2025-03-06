/*
** src/puzzles/aoc2023/day7.rs
*/

use super::Solution;
use crate::types::Counter;
use crate::utils;

use log::debug;

use std::cmp::Ordering;

const HAND_SIZE: usize = 5;

#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("Card::From<char>: invalid character: {}", value),
        }
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Joker => 'J',
                Self::Two => '2',
                Self::Three => '3',
                Self::Four => '4',
                Self::Five => '5',
                Self::Six => '6',
                Self::Seven => '7',
                Self::Eight => '8',
                Self::Nine => '9',
                Self::Ten => 'T',
                Self::Jack => 'J',
                Self::Queen => 'Q',
                Self::King => 'K',
                Self::Ace => 'A',
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_card_counts(counter: Counter<&Card>) -> Self {
        let sorted = counter.sorted().collect::<Vec<_>>();
        let (_, top_count) = sorted[0];
        let &(_, next_count) = sorted.get(1).unwrap_or(&(&&Card::Joker, 0));
        match top_count {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                // full house or three of a kind?
                if next_count == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                // two pair or one pair?
                if next_count == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            _ => HandType::HighCard,
        }
    }
}

#[derive(Eq, PartialEq)]
struct Hand {
    cards: [Card; HAND_SIZE],
    bid: u32,
    hand_type: HandType,
}

impl Hand {
    fn get_type(cards: &[Card]) -> HandType {
        let counter = Counter::from_iter(cards.iter());
        HandType::from_card_counts(counter)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let comparison = self.hand_type.cmp(&other.hand_type);
        if matches!(comparison, Ordering::Equal) {
            let mut i = 0;
            let mut cmp = Ordering::Equal;
            while i < self.cards.len() && cmp == Ordering::Equal {
                cmp = self.cards[i].cmp(&other.cards[i]);
                i += 1;
            }
            cmp
        } else {
            comparison
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let (card_str, bid_str) = utils::split(value, " ").unwrap();
        let cards: [Card; HAND_SIZE] = card_str
            .chars()
            .map(Card::from)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let bid = bid_str.parse().unwrap();
        let hand_type = Self::get_type(&cards);
        Self {
            cards,
            bid,
            hand_type,
        }
    }
}

impl std::fmt::Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let card_strings = self
            .cards
            .iter()
            .map(|card| format!("{:?}", card))
            .collect::<Vec<_>>();
        write!(f, "{} {}", card_strings.join(""), self.bid)
    }
}

#[derive(Eq, PartialEq)]
struct JokerHand {
    cards: [Card; HAND_SIZE],
    bid: u32,
    hand_type: HandType,
}

impl JokerHand {
    fn get_type(cards: &[Card]) -> HandType {
        let mut counter = Counter::from_iter(cards.iter());
        let joker_count = counter.remove(&Card::Joker).unwrap_or(0);

        // check what the best card is
        let top_count = counter.max();
        let mut card_types = counter
            .sorted()
            .filter(|&(_, count)| count == top_count)
            .map(|(&card_type, _)| card_type.clone())
            .collect::<Vec<_>>();
        card_types.sort();
        let card_type = card_types.last().map_or(Card::Ace, |card| card.clone());

        if joker_count > 0 {
            // add the joker count to the count of the best card
            counter.add_many(&card_type, joker_count);
        }
        HandType::from_card_counts(counter)
    }
}

impl Ord for JokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        let comparison = self.hand_type.cmp(&other.hand_type);
        if matches!(comparison, Ordering::Equal) {
            let mut i = 0;
            let mut cmp = Ordering::Equal;
            while i < self.cards.len() && cmp == Ordering::Equal {
                cmp = self.cards[i].cmp(&other.cards[i]);
                i += 1;
            }
            cmp
        } else {
            comparison
        }
    }
}

impl PartialOrd for JokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&str> for JokerHand {
    fn from(value: &str) -> Self {
        let (card_str, bid_str) = utils::split(value, " ").unwrap();
        let cards: [Card; HAND_SIZE] = card_str
            .chars()
            .map(Card::from)
            // replace jacks with jokers
            .map(|card| {
                if matches!(card, Card::Jack) {
                    Card::Joker
                } else {
                    card
                }
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let bid = bid_str.parse().unwrap();
        let hand_type = Self::get_type(&cards);
        Self {
            cards,
            bid,
            hand_type,
        }
    }
}

impl std::fmt::Debug for JokerHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let card_strings = self
            .cards
            .iter()
            .map(|card| format!("{:?}", card))
            .collect::<Vec<_>>();
        write!(f, "{} {}", card_strings.join(""), self.bid)
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // In Camel Cards, you get a list of hands, and your goal is to order them based on the
    // strength of each hand. A hand consists of five cards. The relative strength of each card
    // follows the order where A is the highest and 2 is the lowest.
    let mut hands = input.split('\n').map(Hand::from).collect::<Vec<_>>();
    hands.sort();
    debug!("hands: {:?}", hands);

    // Part A: Find the rank of every hand in your set. What are the total winnings?
    let winnings = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum::<u32>();
    solution.set_part_a(winnings);

    // Part B: Now J cards are jokers, wildcards that can act like whatever card would make the
    // hand the strongest type possible. To balance this, J cards are now the weakest individual
    // cards, weaker even than 2. Using the new joker rule, find the rank of every hand in your
    // set. What are the new total winnings?
    let mut joker_hands = input.split('\n').map(JokerHand::from).collect::<Vec<_>>();
    joker_hands.sort();
    debug!("joker hands: {:?}", joker_hands);
    let joker_winnings = joker_hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum::<u32>();
    solution.set_part_b(joker_winnings);

    solution
}
