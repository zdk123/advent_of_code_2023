use std::collections::HashMap;
use std::cmp::Ordering;
use core::cmp::PartialEq;

fn main() {
    let input = include_str!("./input7.txt");
    let output = part1(input);
    println!("{}", output);
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum CardValue {
    A,
    K,
    Q,
    J,
    T,
    _9,
    _8,
    _7,
    _6,
    _5,
    _4,
    _3,
    _2,
}

impl From<char> for CardValue {
    fn from(ch: char) -> Self {
        match ch.to_ascii_uppercase() {
            'A' => CardValue::A,
            'K' => CardValue::K,
            'Q' => CardValue::Q,
            'J' => CardValue::J,
            'T' => CardValue::T,
            '9' => CardValue::_9,
            '8' => CardValue::_8,
            '7' => CardValue::_7,
            '6' => CardValue::_6,
            '5' => CardValue::_5,
            '4' => CardValue::_4,
            '3' => CardValue::_3,
            '2' => CardValue::_2,
            _ => panic!("Invalid card value: {}", ch),
        }
    }
}


#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [CardValue; 5],
    hand_type: HandType,
}
impl Hand {
    fn new(card_values: &str) -> Self {
        if card_values.len() != 5 {
            panic!("Invalid number of card values");
        }

        let mut cards = [CardValue::A; 5];
        for (index, ch) in card_values.chars().enumerate() {
            cards[index] = match ch.to_ascii_uppercase() {
                'A' => CardValue::A,
                'K' => CardValue::K,
                'Q' => CardValue::Q,
                'J' => CardValue::J,
                'T' => CardValue::T,
                '9' => CardValue::_9,
                '8' => CardValue::_8,
                '7' => CardValue::_7,
                '6' => CardValue::_6,
                '5' => CardValue::_5,
                '4' => CardValue::_4,
                '3' => CardValue::_3,
                '2' => CardValue::_2,
                _ => panic!("Invalid card value: {}", ch),
            };
        }

        let hand_type = Hand::determine_hand_type(&cards);

        Hand { cards, hand_type }
    }

    fn determine_hand_type(cards: &[CardValue; 5]) -> HandType {
        let mut card_count = HashMap::new();

        for &card in cards.iter() {
            *card_count.entry(card).or_insert(0) += 1;
        }

        let max_count = card_count.values().cloned().max().unwrap();

        match max_count {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if card_count.len() == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if card_count.len() == 3 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            1 => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}


impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare based on HandType
        let type_ordering = self.hand_type.cmp(&other.hand_type);
        if type_ordering != Ordering::Equal {
            return type_ordering;
        }

        // Compare based on cards
        for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            let card_ordering = self_card.cmp(other_card);
            if card_ordering != Ordering::Equal {
                return card_ordering;
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn part1(input: &str) -> u32 {

    let mut hands: Vec<_> = input
                    .lines()
                    .map(|line| {
                        let (card, bid) = line.split_once(' ').unwrap();
                        (
                            Hand::new(
                                card
                            ),
                            bid.parse::<u32>().unwrap(),
                        )
                    })
                    .collect();
    
    hands.sort_by(|(a, _), (b, _)| a.cmp(b));


    let winnings: u32 = hands.iter().rev().zip(1u32..).map(|((_, bid), rank)| rank*bid).sum();

    return winnings;
    
}


