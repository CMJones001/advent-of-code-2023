use std::collections::{BTreeMap};
use std::fmt::{Display, Formatter};

fn main() {
    let input = include_str!("problem_text");
    let total = get_bet_total_naive(input);

    println!("Total: {}", total);
}

fn parse_input(input: &str) -> Vec<(HandNaive, u32)> {
    input
        .lines()
        .map(|line| {
            let (hand_str, bid) = line.split_at(5);
            let hand = HandNaive::from_string(hand_str);

            (hand, bid.trim().parse::<u32>().unwrap())
        }).collect()
}

fn get_bet_total_naive(input: &str) -> u32 {
    let mut bets = parse_input(input);

    bets.sort_by(|(hand_a, _), (hand_b, _)| hand_a.cmp(hand_b));
    for (hand, bid) in bets.iter() {
        println!("{bid:4}: {hand}");
    }
    bets.iter().enumerate().map(|(pos, (_, bid))| (pos as u32 +1) * bid).sum()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
enum Card {
    Unit(u8),
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Card::Unit(n) => write!(f, "{}", n),
            Card::Ten => write!(f, "T"),
            Card::Jack => write!(f, "J"),
            Card::Queen => write!(f, "Q"),
            Card::King => write!(f, "K"),
            Card::Ace => write!(f, "A"),
        }
    }
}

fn parse_card(card: char) -> Card {
    match card {
        'A' => Card::Ace,
        'K' => Card::King,
        'Q' => Card::Queen,
        'J' => Card::Jack,
        'T' => Card::Ten,
        x => Card::Unit(x.to_digit(10).unwrap() as u8),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct HandNaive {
    hand_type: HandType,
    cards: Vec<Card>,
}

impl Display for HandNaive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {:?}", self.cards, self.hand_type)
    }
}

impl HandNaive {
    fn from_string(hand: &str) -> HandNaive {
        let cards = hand.chars().map(parse_card).collect::<Vec<_>>();

        HandNaive::from_cards(cards)
    }

    fn from_cards(cards: Vec<Card>) -> HandNaive {
        let counter = count_cards(&cards);

        let num_unique_cards = counter.len();

        if num_unique_cards == 1 {
            // If there is only one unique card, then it must be a five of a kind
            HandNaive {
                hand_type: HandType::FiveOfAKind,
                cards,
            }
        } else if num_unique_cards == 2 {
            if find_count(4, &counter).is_some() {
                // If there are two unique cards, and one of them has a count of 4, then it
                // must be a four of a kind
                HandNaive {
                    hand_type: HandType::FourOfAKind,
                    cards,
                }
            } else {
                // Otherwise, it must be a full house
                HandNaive {
                    hand_type: HandType::FullHouse,
                    cards,
                }
            }
        } else if num_unique_cards == 3 {
            if find_count(3, &counter).is_some() {
                // If there are three unique cards, and one of them has a count of 3, then it
                // must be a three of a kind
                HandNaive {
                    hand_type: HandType::ThreeOfAKind,
                    cards,
                }
            } else {
                // Otherwise, it must be a two pair
                HandNaive {
                    hand_type: HandType::TwoPair,
                    cards,
                }
            }
        } else if num_unique_cards == 4 {
            // If there are four unique cards, then it must be a one pair
            HandNaive {
                hand_type: HandType::OnePair,
                cards,
            }
        } else if num_unique_cards == 5 {
            // If there are five unique cards, then it must be a high card
            HandNaive {
                hand_type: HandType::HighCard,
                cards,
            }
        } else {
            panic!("Invalid number of unique cards: {}", num_unique_cards);
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn find_count(count: u32, counter: &BTreeMap<Card, u32>) -> Option<Card> {
    counter
        .iter()
        .rev()
        .find(|(_, &c)| c == count)
        .map(|(&card, _)| card)
}

fn count_cards(cards: &[Card]) -> BTreeMap<Card, u32> {
    cards.iter().fold(BTreeMap::new(), |mut acc, card| {
        *acc.entry(*card).or_insert(0) += 1;
        acc
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn card_from_string() {
        let sample_input = "32T3K";

        let hand_expected = vec![
            Card::Unit(3),
            Card::Unit(2),
            Card::Ten,
            Card::Unit(3),
            Card::King,
        ];
        let hand_actual = sample_input
            .chars()
            .map(super::parse_card)
            .collect::<Vec<_>>();

        assert_eq!(hand_expected, hand_actual);
    }

    #[test]
    fn test_card_sorting() {
        let sample_input = "32T3K";

        let hand_expected = vec![
            Card::King,
            Card::Ten,
            Card::Unit(3),
            Card::Unit(3),
            Card::Unit(2),
        ];
        let mut hand_actual = sample_input
            .chars()
            .map(super::parse_card)
            .collect::<Vec<_>>();

        hand_actual.sort();
        hand_actual.reverse();

        assert_eq!(hand_expected, hand_actual);
    }

    #[test]
    fn test_card_sorting_2() {
        let sample_input = "K677K";

        let hand_expected = vec![
            Card::King,
            Card::King,
            Card::Unit(7),
            Card::Unit(7),
            Card::Unit(6),
        ];
        let mut hand_actual = sample_input
            .chars()
            .map(super::parse_card)
            .collect::<Vec<_>>();

        hand_actual.sort();
        hand_actual.reverse();

        assert_eq!(hand_expected, hand_actual);
    }

    #[test]
    fn test_counter() {
        let sample_input = "K3T3K";

        let mut hand_actual = sample_input
            .chars()
            .map(super::parse_card)
            .collect::<Vec<_>>();

        hand_actual.sort();
        hand_actual.reverse();

        let counter = count_cards(&hand_actual);

        let counter_expected =
            BTreeMap::from_iter(vec![(Card::King, 2), (Card::Ten, 1), (Card::Unit(3), 2)]);

        assert_eq!(counter_expected, counter);
    }

    #[test]
    fn test_counter_2() {
        let sample_input = "K677K";

        let mut hand_actual = sample_input
            .chars()
            .map(super::parse_card)
            .collect::<Vec<_>>();

        hand_actual.sort();
        hand_actual.reverse();

        let counter = count_cards(&hand_actual);

        let counter_expected =
            BTreeMap::from_iter(vec![(Card::King, 2), (Card::Unit(7), 2), (Card::Unit(6), 1)]);

        for (k, v) in counter.iter() {
            println!("{:?}: {}", k, v);
        }
        assert_eq!(counter_expected, counter);
    }

    #[test]
    fn test_parse_hands() {
        let sample_input = "32T3K";

        let hand = HandNaive::from_string(sample_input);
        assert_eq!(hand.hand_type, HandType::OnePair);
    }

    #[test]
    fn test_parse_hands_2() {
        let sample_input = "T55J5";
        let hand = HandNaive::from_string(sample_input);

        assert_eq!(hand.hand_type, HandType::ThreeOfAKind);
    }

    #[test]
    fn test_parse_hands_3() {
        let sample_input = "776KK";
        let hand = HandNaive::from_string(sample_input);

        assert_eq!(hand.hand_type, HandType::TwoPair)
    }

    #[test]
    fn test_parse_hands_4() {
        let sample_input = "KTJJT";
        let hand = HandNaive::from_string(sample_input);

        assert_eq!(hand.hand_type, HandType::TwoPair);
    }

    #[test]
    fn test_parse_hands_5() {
        let sample_input = "QQQJA";
        let hand = HandNaive::from_string(sample_input);
        assert_eq!(hand.hand_type, HandType::ThreeOfAKind);
    }

    #[test]
    fn test_parse_input() {
        let sample_input = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};

        let bets = parse_input(sample_input);
        assert_eq!(bets.len(), 5);
    }

    #[test]
    fn get_bet_total_test() {
        let sample_input = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};

        let total = get_bet_total_naive(sample_input);
        let total_expected = 6440;

        assert_eq!(total_expected, total);
    }
}
