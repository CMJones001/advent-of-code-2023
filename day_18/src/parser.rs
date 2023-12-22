use crate::{Direction, Instruction};
use nom::bytes::complete::tag;
use nom::character::complete::{hex_digit1, multispace0, multispace1, space1};
use nom::multi::separated_list1;
use nom::sequence::{delimited, tuple};
use nom::{IResult, Parser};
use nom_supreme::ParserExt;

pub fn parse(input: &str) -> Result<Vec<Instruction>, String> {
    match parse_list(input) {
        Ok((rem, instructions)) => {
            if rem.is_empty() {
                Ok(instructions)
            } else {
                Err(format!("Error: unparsed input: {}", rem))
            }
        }
        Err(e) => Err(format!("Error: {:?}", e)),
    }
}

fn parse_list(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(multispace1, parse_instruction)
        .terminated(multispace0)
        .parse(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    use nom::character::complete::u32 as u32_parser;
    let direction_parser = nom::branch::alt((
        tag("U").value(Direction::Up),
        tag("D").value(Direction::Down),
        tag("L").value(Direction::Left),
        tag("R").value(Direction::Right),
    ))
    .terminated(space1);

    let distance_parser = u32_parser.terminated(space1);

    let hex_parser = hex_digit1.map(|s| u32::from_str_radix(s, 16).unwrap());
    let colour_parser = delimited(tag("(#"), hex_parser, tag(")"));

    tuple((direction_parser, distance_parser, colour_parser))
        .map(|(direction, distance, colour)| Instruction::new(direction, distance, colour))
        .parse(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "R 6 (#70c710)";
        let expected = Instruction::new(Direction::Right, 6, 0x70c710);

        assert_eq!(parse_instruction(input), Ok(("", expected)));
    }

    #[test]
    fn test_parse_multiple() {
        let input = "R 6 (#70c710)\nD 5 (#0dc571)";
        let expected = vec![
            Instruction::new(Direction::Right, 6, 0x70c710),
            Instruction::new(Direction::Down, 5, 0x0dc571),
        ];
        assert_eq!(parse_list(input), Ok(("", expected)));
    }
}
