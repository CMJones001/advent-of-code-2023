use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::character::complete::{alphanumeric1, multispace1};
use num::integer::lcm;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, pair, separated_pair};
use nom::IResult;
use nom::Parser;
use nom_supreme::ParserExt;
use std::collections::HashMap;

fn main() {
    problem_one();
    problem_two();
}

fn problem_one() {
    let input = include_str!("problem_text");
    let result = count_steps(input);
    println!("Result: {:?}", result);
}

fn problem_two() {
    let input = include_str!("problem_text");
    let result = parallel_cycles(input);
    println!("Result: {:?}", result);
}

fn count_steps(input: &str) -> u64 {
    let (_, (instructions, network)) = parse_input(input).unwrap();
    let mut start = Element::new("AAA");

    let mut steps = 0;
    for instruction in instructions.repeat(1000) {
        steps += 1;

        match instruction {
            Instruction::L => {
                let either = network.get(&start).unwrap();
                start = either.left.clone();
            }
            Instruction::R => {
                let either = network.get(&start).unwrap();
                start = either.right.clone();
            }
        };

        if start == Element::new("ZZZ") {
            break;
        }
    }

    steps
}

/// This iterates through networks with multiple starting points in parallel.
///
/// However, for the full input this would take an extremely long time to run, so instead
/// we note that the each starting point has a cycle length (some multiple of the instruction set
/// length) and we can use that to calculate the number of steps.
///
/// The number of steps is the LCM of the cycle lengths.
fn parallel_cycles(input: &str) -> u64 {
    let (_, (instructions, network)) = parse_input(input).expect("Parse Error");

    let mut starting_points: Vec<_> = network.keys().filter(|&key| key.0.ends_with('A')).collect();
    let mut steps = 0;

    let mut lasts = [0u64; 6];
    let mut cycle = [0u64; 6];

    for instruction in instructions.repeat(1000) {
        steps += 1;

        starting_points
            .iter_mut()
            .for_each(|start| match instruction {
                Instruction::L => {
                    let either = network.get(start).expect("No key found");
                    let left = &either.left;
                    *start = left;
                }
                Instruction::R => {
                    let either = network.get(start).expect("No key found");
                    let right = &either.right;
                    *start = right;
                }
            });

        if starting_points.iter().any(|start| start.0.ends_with('Z')) {
            starting_points.iter().enumerate().for_each(|(i, start)| {
                if start.0.ends_with('Z') {
                    let delta = steps - lasts[i];
                    cycle[i] = delta;
                    lasts[i] = steps;
                }
            });
        }

        if starting_points.iter().all(|start| start.0.ends_with('Z')) {
            // In the trivial case, we can just return the number of steps
            println!("{steps}: starting_points: {:?}", starting_points);
            return steps;
        }

        if cycle.iter().all(|&x| x != 0) {
            // In the non-trivial case, we can break once we know the cycle lengths
            println!("Cycle array filled: {:?}", cycle);
            break;
        }
    }

    // The total cycle is the LCM of the individual cycles
    cycle.into_iter().reduce(lcm).unwrap()
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Instruction {
    L,
    R,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Element(String);

impl Element {
    fn new(input: &str) -> Element {
        Element(input.to_string())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Either {
    left: Element,
    right: Element,
}

fn triplet_parser(input: &str) -> IResult<&str, Element> {
    alphanumeric1.map(Element::new).parse(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Instruction>, HashMap<Element, Either>)> {
    let network = parse_network;
    let instructions = parse_instructions;

    let mut parser = separated_pair(instructions, multispace1, network);
    parser.parse(input)
}

fn parse_network(input: &str) -> IResult<&str, HashMap<Element, Either>> {
    let pair_parse = separated_pair(triplet_parser, pair(tag(","), space1), triplet_parser);
    let either_parser =
        delimited(tag("("), pair_parse, tag(")")).map(|(left, right)| Either { left, right });

    let line_parse = separated_pair(triplet_parser, tag(" = "), either_parser);
    let parser = separated_list1(multispace1, line_parse);

    parser
        .map(|elements| {
            elements
                .into_iter()
                .map(|(key, value)| (key, value))
                .collect()
        })
        .parse(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    let left = tag("L").value(Instruction::L);
    let right = tag("R").value(Instruction::R);
    many1(left.or(right)).parse(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    #[test]
    fn test_triplet_parser() {
        match triplet_parser("abc") {
            Ok((rest, instruction)) => {
                assert_eq!(rest, "");
                assert_eq!(instruction, Element::new("abc"));
            }
            Err(e) => panic!("Parse Error: {:?}", e),
        }
    }

    #[test]
    fn test_network_parse() {
        let input = "abc = (def, ghi)";
        let expected = Element::new("abc");
        let expected_either = Either {
            left: Element::new("def"),
            right: Element::new("ghi"),
        };
        let expected_map = {
            let mut map = HashMap::new();
            map.insert(expected, expected_either);
            map
        };

        match parse_network(input) {
            Ok((rest, map)) => {
                assert_eq!(rest, "");
                assert_eq!(map, expected_map);
            }
            Err(e) => panic!("Parse Error: {:?}", e),
        }
    }

    #[test]
    fn test_parse_instructions() {
        let input = "LRLR";
        let expected = vec![
            Instruction::L,
            Instruction::R,
            Instruction::L,
            Instruction::R,
        ];

        match parse_instructions(input) {
            Ok((rest, instructions)) => {
                assert_eq!(rest, "");
                assert_eq!(instructions, expected);
            }
            Err(e) => panic!("Parse Error: {:?}", e),
        }
    }

    #[test]
    fn test_sample_input() {
        let sample_input = indoc! {"
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
            "
        };

        let expected_count = 2;
        let actual_count = count_steps(sample_input);

        assert_eq!(actual_count, expected_count);
    }

    #[test]
    fn test_sample_looping() {
        let sample_input = indoc! {"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
            "
        };

        let expected_count = 6;
        let actual_count = count_steps(sample_input);

        assert_eq!(actual_count, expected_count);
    }

    #[test]
    fn test_par_lookup() {
        let sample_input = indoc! {"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
            "};

        let expected_count = 6;
        let actual_count = parallel_cycles(sample_input);

        assert_eq!(actual_count, expected_count);
    }
}
