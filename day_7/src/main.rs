use std::collections::{BTreeMap};
use std::fmt::Display;

fn main() {
    let input = include_str!("problem_text");
    let total = get_bet_total(input);

    println!("Total: {}", total);
}

fn parse_input(input: &str) -> Vec<(Hand, u32)> {
    input
        .lines()
        .map(|line| {
            let (hand_str, bid) = line.split_at(5);
            let hand = Hand::from_string(hand_str);

            (hand, bid.trim().parse::<u32>().unwrap())
        }).collect()
}

fn get_bet_total(input: &str) -> u32 {
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Hand {
    HighCard(Card, Card, Card, Card, Card),
    OnePair(Card, Card, Card, Card),
    TwoPair(Card, Card, Card),
    ThreeOfAKind(Card, Card, Card),
    FullHouse(Card, Card),
    FourOfAKind(Card, Card),
    FiveOfAKind(Card),
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Hand::HighCard(a, b, c, d, e) => write!(f, "{} {} {} {} {}: High", a, b, c, d, e),
            Hand::OnePair(a, b, c, d) => write!(f, "{a} {a} {b} {c} {d}: One pair"),
            Hand::TwoPair(a, b, c) => write!(f, "{a} {a} {b} {b} {c}: Two pair"),
            Hand::ThreeOfAKind(a, b, c) => write!(f, "{a} {a} {a} {b} {c}: Three of a kind"),
            Hand::FullHouse(a, b) => write!(f, "{a} {a} {a} {b} {b}: Full house"),
            Hand::FourOfAKind(a, b) => write!(f, "{a} {a} {a} {a} {b}: Four of a kind"),
            Hand::FiveOfAKind(a) => write!(f, "{a} {a} {a} {a} {a}: Five of a kind"),
        }
    }
}

impl Hand {
    fn from_string(hand: &str) -> Hand {
        let cards = hand.chars().map(parse_card).collect::<Vec<_>>();

        Hand::from_cards(cards)
    }

    fn from_cards(mut cards: Vec<Card>) -> Hand {
        cards.sort();
        cards.reverse();

        let counter = count_cards(&cards);

        let num_unique_cards = counter.len();
        if num_unique_cards == 1 {
            // If all cards are the same, we have a five of a kind
            return Hand::FiveOfAKind(cards[0]);
        } else if num_unique_cards == 2 {
            // If there are only two unique cards, we have either a four of a kind or a full house
            return if let Some(four_match) = find_count(4, &counter) {
                // If there are four of one card, we have a four of a kind
                let secondary = cards.iter().find(|&&card| card != four_match).unwrap();
                Hand::FourOfAKind(four_match, *secondary)

            } else if let Some(three_match) = find_count(3, &counter) {
                // If there are three of one card, we have a full house
                let secondary = cards.iter().find(|&&card| card != three_match).unwrap();
                Hand::FullHouse(three_match, *secondary)
            } else {
                panic!("Invalid number of unique cards: {}", num_unique_cards);
            };

        } else if num_unique_cards == 3 {
            // If there are three unique cards, we have either a three of a kind or a two pair
            return if let Some(three_match) = find_count(3, &counter) {
                let remaining_cards = cards
                    .iter()
                    .filter(|&&card| card != three_match)
                    .collect::<Vec<_>>();
                Hand::ThreeOfAKind(three_match, *remaining_cards[0], *remaining_cards[1])

            } else {
                // This is a bit dirty
                let remaining_card = find_count(1, &counter).unwrap();
                let first_pair = find_count(2, &counter).unwrap();
                let second_pair = cards
                    .iter()
                    .find(|&&card| card != first_pair && card != remaining_card)
                    .unwrap();
                Hand::TwoPair(first_pair, *second_pair, remaining_card)
            };
        } else if num_unique_cards == 4 {
            // If there are four unique cards, we have a one pair
            let pair = find_count(2, &counter).unwrap();
            let remaining_cards = cards
                .iter()
                .filter(|&&card| card != pair)
                .collect::<Vec<_>>();
            return Hand::OnePair(
                pair,
                *remaining_cards[0],
                *remaining_cards[1],
                *remaining_cards[2],
            );
        } else if num_unique_cards == 5 {
            return Hand::HighCard(cards[0], cards[1], cards[2], cards[3], cards[4]);
        } else {
            panic!("Invalid number of unique cards: {}", num_unique_cards);
        }
    }
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

        let hand = Hand::from_string(sample_input);
        let hand_expected = Hand::OnePair(Card::Unit(3), Card::King, Card::Ten, Card::Unit(2));

        assert_eq!(hand_expected, hand);
    }

    #[test]
    fn test_parse_hands_2() {
        let sample_input = "T55J5";
        let hand = Hand::from_string(sample_input);
        let hand_expected = Hand::ThreeOfAKind(Card::Unit(5), Card::Jack, Card::Ten);

        assert_eq!(hand_expected, hand);
    }

    #[test]
    fn test_parse_hands_3() {
        let sample_input = "776KK";
        let hand = Hand::from_string(sample_input);

        let hand_expected = Hand::TwoPair(Card::King, Card::Unit(7), Card::Unit(6));
        assert_eq!(hand_expected, hand)
    }

    #[test]
    fn test_parse_hands_4() {
        let sample_input = "KTJJT";
        let hand = Hand::from_string(sample_input);

        let hand_expected = Hand::TwoPair(Card::Jack, Card::Ten, Card::King);
        assert_eq!(hand_expected, hand)
    }

    #[test]
    fn test_parse_hands_5() {
        let sample_input = "QQQJA";
        let hand = Hand::from_string(sample_input);

        let hand_expected = Hand::ThreeOfAKind(Card::Queen, Card::Ace, Card::Jack);
        assert_eq!(hand_expected, hand)
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

        let expected = vec![
            (Hand::OnePair(Card::Unit(3), Card::King, Card::Ten, Card::Unit(2)), 765),
            (Hand::ThreeOfAKind(Card::Unit(5), Card::Jack, Card::Ten), 684),
            (Hand::TwoPair(Card::King, Card::Unit(7), Card::Unit(6)), 28),
            (Hand::TwoPair(Card::Jack, Card::Ten, Card::King), 220),
            (Hand::ThreeOfAKind(Card::Queen, Card::Ace, Card::Jack), 483),
        ];

        assert_eq!(expected, bets);
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

        let total = get_bet_total(sample_input);
        let total_expected = 6440;

        assert_eq!(total_expected, total);
    }
}
