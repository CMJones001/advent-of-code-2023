use nom::bytes::complete::tag;
use nom::character::complete::{space0, space1};
use nom::multi::separated_list1;
use nom::sequence::{delimited, tuple};
use nom::IResult;
use nom::Parser;
use nom_supreme::parser_ext::ParserExt;
use std::collections::HashSet;

fn main() {
    let score = problem_one();
    println!("Problem One: {}", score);
}

fn problem_one() -> u32 {
    let input = include_str!("problem_text");
    get_total_score(input)
}

fn get_total_score(lines: &str) -> u32 {
    let cards = parse_lines(lines);
    cards.iter().map(|card| card.score()).sum()
}

#[derive(Debug, PartialEq)]
struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    collected_numbers: Vec<u32>,
}

impl Card {
    fn from_line(line: &str) -> Option<Card> {
        parse_card(line).map(|(_, card)| card).ok()
    }

    fn score(&self) -> u32 {
        // This is a bit inefficient, if we knew that there are no repeats in the collected numbers
        // then we could do everything with sets.
        let n_matches = self
            .collected_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count() as u32;

        if n_matches == 0 {
            0
        } else {
            2u32.pow(n_matches - 1)
        }
    }
}

fn parse_lines(lines: &str) -> Vec<Card> {
    lines
        .lines()
        .map(|line| Card::from_line(line))
        .collect::<Option<Vec<_>>>()
        .expect("Failed to parse lines")
}

fn parse_card(line: &str) -> IResult<&str, Card> {
    use nom::character::complete::u32;
    let id_parser = delimited(tag("Card").terminated(space1), u32, tag(":").terminated(space0));

    let winning_numbers_parser = separated_list1(space1, u32);
    let separator_parser = delimited(space1, tag("|"), space1);
    let collected_numbers_parser = separated_list1(space1, u32);

    let mut parser = tuple((
        id_parser,
        winning_numbers_parser,
        separator_parser,
        collected_numbers_parser,
    ))
    .map(|(id, winning_numbers, _, collected_numbers)| Card {
        id,
        winning_numbers: winning_numbers.into_iter().collect(),
        collected_numbers,
    });

    parser.parse(line)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_card() {
        let input = "Card 1: 1 2 3 4 5 | 1 2 3 4 5";
        let expected = Card {
            id: 1,
            winning_numbers: [1, 2, 3, 4, 5].iter().cloned().collect(),
            collected_numbers: [1, 2, 3, 4, 5].to_vec(),
        };
        let (rem, card) = parse_card(input).unwrap();
        assert_eq!(card, expected);
        assert_eq!(rem, "");
    }

    #[test]
    fn test_example_line() {
        let input = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let expected = Card {
            id: 2,
            winning_numbers: [13, 32, 20, 16, 61].iter().cloned().collect(),
            collected_numbers: [61, 30, 68, 82, 17, 32, 24, 19].to_vec(),
        };

        match parse_card(input) {
            Ok((rem, card)) => {
                assert_eq!(card, expected);
                assert_eq!(rem, "");
            }
            Err(e) => {
                println!("Parse Error: {:?}", e);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_card_score() {
        let input = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let expected_score = 2;

        let card = Card::from_line(input).unwrap();
        assert_eq!(card.score(), expected_score);

        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let expected_score = 8;

        let card = Card::from_line(input).unwrap();
        assert_eq!(card.score(), expected_score);
    }

    #[test]
    fn test_card_score_no_matches() {
        let input = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let expected_score = 0;

        let card = Card::from_line(input).unwrap();
        assert_eq!(card.score(), expected_score);
    }

    #[test]
    fn test_parse_lines() {
        let input = indoc!{"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};

        let expected_score = 13;
        let total_score = get_total_score(input);
        assert_eq!(total_score, expected_score);
    }
}
