use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{space0, space1, line_ending};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, separated_pair};
use nom::IResult;
use nom_supreme::parser_ext::ParserExt;

fn main() {
    problem_one();
    problem_two();
}

fn problem_one() {
    let input = include_str!("./problem_text");
    let id_sum = get_id_total(input);
    println!("The sum of the valid game ids is {}", id_sum);
}

fn problem_two() {
    let input = include_str!("./problem_text");
    let power_sum = get_power_total(input);
    println!("The sum of the valid game powers is {}", power_sum);
}

fn get_id_total(input: &str) -> u32 {
    let (_, games) = parse_file(input).unwrap();

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    games
        .into_iter()
        .filter(|game| game.is_game_valid(max_red, max_green, max_blue))
        .map(|game| game.id)
        .sum()
}

fn get_power_total(input: &str) -> u32 {
    let (_, games) = parse_file(input).unwrap();

    games
        .into_iter()
        .map(|game| game.get_max_counts().get_power())
        .sum()
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}


impl Game {
    /// Return true if all rounds are valid
    fn is_game_valid(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        self.rounds
            .iter()
            .all(|round| round.is_round_valid(max_red, max_green, max_blue))
    }

    /// Returns the minimum number of dice of each colour required in this game
    fn get_max_counts(&self) -> Counts {
        self.rounds
            .iter()
            .fold(Counts::new(), |acc, round| acc.update(round))
    }
}

#[derive(Debug, PartialEq)]
struct Round {
    blue: u32,
    red: u32,
    green: u32,
}

impl Round {
    fn is_round_valid(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        self.red <= max_red && self.green <= max_green && self.blue <= max_blue
    }
}

#[derive(Debug, PartialEq)]
struct Counts {
    blue: u32,
    red: u32,
    green: u32,
}

impl Counts {
    fn new() -> Self {
        Counts {
            blue: 0,
            red: 0,
            green: 0,
        }
    }

    /// Returns a new `Counts` with the number of each colour updated to the maximum of the two
    fn update(&self, round: &Round) -> Self {
        let Counts {
            blue: b1,
            red: r1,
            green: g1,
        } = self;
        let Round {
            blue: b2,
            red: r2,
            green: g2,
        } = round;

        Counts {
            blue: *b1.max(b2),
            red: *r1.max(r2),
            green: *g1.max(g2),
        }
    }

    fn get_power(&self) -> u32 {
        self.blue * self.red * self.green
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
enum Colour {
    Blue,
    Red,
    Green,
}

fn parse_file(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(line_ending, parse_game)(input)
}

fn parse_game(line: &str) -> IResult<&str, Game> {
    let id_parser = delimited(tag("Game "), nom::character::complete::u32, tag(": "));
    let round_parser = separated_list1(pair(tag(";"), space1), parse_round);

    map(pair(id_parser, round_parser), |(id, rounds)| Game {
        id,
        rounds,
    })(line)
}

fn parse_round(line: &str) -> IResult<&str, Round> {
    let colours = alt((
        tag("blue").value(Colour::Blue),
        tag("red").value(Colour::Red),
        tag("green").value(Colour::Green),
    ));
    let colour_parser = separated_pair(nom::character::complete::u32, space1, colours);
    let list_sep = tag(",").terminated(space0);
    let colour_list_parser = separated_list1(list_sep, colour_parser);

    map(colour_list_parser, |v| {
        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;

        // This would also work if we have multiple versions of the same colour
        // in the same round, although this doesn't seem to happen.
        v.into_iter().for_each(|(count, colour)| match colour {
            Colour::Blue => blue += count,
            Colour::Red => red += count,
            Colour::Green => green += count,
        });

        Round { blue, red, green }
    })(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Round {
        fn from_tuple(blue: u32, red: u32, green: u32) -> Self {
            Round { blue, red, green }
        }
    }

    #[test]
    fn test_parse_round() {
        let input = "3 blue, 4 red";
        let expected = Round {
            blue: 3,
            red: 4,
            green: 0,
        };
        let (rem, actual) = parse_round(input).unwrap();
        assert_eq!(rem, "");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_game() {
        let line_one = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = Game {
            id: 1,
            rounds: vec![
                Round::from_tuple(3, 4, 0),
                Round::from_tuple(6, 1, 2),
                Round::from_tuple(0, 0, 2),
            ],
        };

        let (rem, actual) = parse_game(line_one).unwrap();
        assert_eq!(rem, "");
        assert_eq!(expected, actual);

        let line_four = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let expected = Game {
            id: 4,
            rounds: vec![
                Round::from_tuple(6, 3, 1),
                Round::from_tuple(0, 6, 3),
                Round::from_tuple(15, 14, 3),
            ],
        };

        let (rem, actual) = parse_game(line_four).unwrap();
        assert_eq!(rem, "");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_game_is_valid() {
        let game_string = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let (_, game) = parse_game(game_string).unwrap();
        assert!(game.is_game_valid(12, 13, 14));
    }

    #[test]
    fn test_game_is_not_valid() {
        let game_string =
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let (rem, game) = parse_game(game_string).unwrap();

        assert_eq!(rem, "");
        assert!(!game.is_game_valid(12, 13, 14));
    }

    #[test]
    fn test_id_total() {
        let input = indoc::indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};
        let expected = 8;
        let actual = get_id_total(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_max_counts() {
        let game_string = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let (_, game) = parse_game(game_string).unwrap();
        let counts = Counts {
            blue: 6,
            red: 4,
            green: 2,
        };
        assert_eq!(counts, game.get_max_counts());
    }

    #[test]
    fn test_get_powers() {
        let input = indoc::indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};
        let expected = 2286;
        let actual = get_power_total(input);
        assert_eq!(expected, actual);
    }
}
