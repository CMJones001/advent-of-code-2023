use nom::character::complete::space1;
use nom::combinator::{map, value};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;

fn main() {
    println!("Hello, world!");
}

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[derive(Debug, PartialEq)]
struct Round {
    blue: u32,
    red: u32,
    green: u32,
}

fn parse_line(line: &str) -> IResult<&str, Game> {
    todo!()
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Colour {
    Blue,
    Red,
    Green,
}

fn parse_round(line: &str) -> IResult<&str, Round> {
    use nom::bytes::complete::tag;
    let blue_parser = separated_pair(
        nom::character::complete::u32,
        space1,
        value(Colour::Blue, tag("blue")),
    );
    let red_parser = separated_pair(
        nom::character::complete::u32,
        space1,
        value(Colour::Red, tag("red")),
    );
    let green_parser = separated_pair(
        nom::character::complete::u32,
        space1,
        value(Colour::Green, tag("green")),
    );

    let mut parser = separated_list1(
        nom::bytes::complete::tag(", "),
        nom::branch::alt((
            blue_parser,
            red_parser,
            green_parser,
        )),
    );

    map(
        parser,
        |mut v| {
            let mut blue = 0;
            let mut red = 0;
            let mut green = 0;

            while let Some((count, colour)) = v.pop() {
                match colour {
                    Colour::Blue => blue += count,
                    Colour::Red => red += count,
                    Colour::Green => green += count,
                }
            }
            Round { blue, red, green }
        },
    )(line)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}