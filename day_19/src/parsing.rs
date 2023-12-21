use crate::{Bin, Filter, FilterList, Part, PartType};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace0, multispace1};
use nom::multi::separated_list1;
use nom::sequence::{delimited, tuple};
use nom::{IResult, Parser};
use nom_supreme::ParserExt;
use std::collections::HashMap;

/// Parse a list of parts, each of the form `{x=787,m=2655,a=1222,s=2876}`
pub(crate) fn parse_parts(input: &str) -> IResult<&str, Vec<Part>> {
    use nom::character::complete::u64;
    let x_parser = u64.preceded_by(tag("x="));
    let m_parser = u64.preceded_by(tag("m="));
    let a_parser = u64.preceded_by(tag("a="));
    let s_parser = u64.preceded_by(tag("s="));

    let tuple_parser = delimited(
        tag("{"),
        tuple((
            x_parser.terminated(tag(",")),
            m_parser.terminated(tag(",")),
            a_parser.terminated(tag(",")),
            s_parser,
        )),
        tag("}"),
    );

    let part_line_parser = tuple_parser.map(|(x, m, a, s)| Part::new(x, m, a, s));

    let mut parser = separated_list1(multispace1, part_line_parser).terminated(multispace0);
    parser.parse(input)
}

/// Parse a list of filter rows, each of the form `px{a<2006:qkq,m>2090:A,rfg}`
pub(crate) fn parse_filters_rows(input: &str) -> IResult<&str, HashMap<Bin, FilterList>> {
    let bin_parser = alpha1.map(Bin::new);
    let filter_brace = delimited(tag("{"), parse_filter_list, tag("}"));

    let line_parser = tuple((bin_parser, filter_brace)).map(|(bin, filter_list)| {
        let mut map = HashMap::new();
        map.insert(bin, filter_list);
        map
    });

    let mut parser = separated_list1(multispace1, line_parser).map(|lines| {
        lines.into_iter().fold(HashMap::new(), |mut acc, line| {
            acc.extend(line);
            acc
        })
    });

    parser.parse(input)
}

/// Parse a filter list of the form `a<2006:qkq,m>2090:A,rfg`
pub(crate) fn parse_filter_list(input: &str) -> IResult<&str, FilterList> {
    let filter_parser = alt((parse_operator, parse_unconditional));
    let mut parser = separated_list1(tag(","), filter_parser).map(FilterList);
    parser.parse(input)
}

/// Parse a filter of the form `rfg` or `A`
fn parse_unconditional(input: &str) -> IResult<&str, Filter> {
    let label_parser = alpha1;

    let mut parser = label_parser.map(|label| Filter::new_unconditional(Bin::new(label)));

    parser.parse(input)
}

/// Parse a filter of the form `a<2006:qkq`
fn parse_operator(input: &str) -> IResult<&str, Filter> {
    let part_type_parser = alt((
        tag("x").value(PartType::X),
        tag("m").value(PartType::M),
        tag("a").value(PartType::A),
        tag("s").value(PartType::S),
    ));
    let relation_parser = alt((tag(">"), tag("<")));
    let value_parser = nom::character::complete::u64;

    let bin_parser = alpha1.preceded_by(tag(":"));

    let mut parser = tuple((part_type_parser, relation_parser, value_parser, bin_parser)).map(
        |(part, relation, value, _bin)| {
            let bin = Bin::new(_bin);
            match relation {
                ">" => Filter::new_greater_than(value, part, bin),
                "<" => Filter::new_less_than(value, part, bin),
                _ => panic!("Unknown relation"),
            }
        },
    );

    parser.parse(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_filter() {
        let input = "a<2006:qkq";

        let (res, filter) = parse_operator(input).expect("Failed to parse filter");
        if !res.is_empty() {
            panic!("Did not parse entire input");
        }

        let expected = Filter::new_less_than(2006, PartType::A, Bin::new("qkq"));

        assert_eq!(filter, expected);
    }

    #[test]
    fn test_parse_filter_list() {
        let input = "a<2006:qkq,m>2090:A,rfg";

        let (_res, result) = parse_filter_list(input).unwrap();
        let expected = FilterList(vec![
            Filter::new_less_than(2006, PartType::A, Bin::new("qkq")),
            Filter::new_greater_than(2090, PartType::M, Bin::Accept),
            Filter::new_unconditional(Bin::new("rfg")),
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_filter_commands() {
        let input = indoc! {"
            px{a<2006:qkq,m>2090:A,rfg}
            pv{a>1716:R,A}"
        };

        let (_res, result) = parse_filters_rows(input).unwrap();
        assert_eq!(_res, "");

        let mut expected = HashMap::new();
        expected.insert(
            Bin::new("px"),
            FilterList(vec![
                Filter::new_less_than(2006, PartType::A, Bin::new("qkq")),
                Filter::new_greater_than(2090, PartType::M, Bin::Accept),
                Filter::new_unconditional(Bin::new("rfg")),
            ]),
        );
        expected.insert(
            Bin::new("pv"),
            FilterList(vec![
                Filter::new_greater_than(1716, PartType::A, Bin::Reject),
                Filter::new_unconditional(Bin::Accept),
            ]),
        );

        let keys = vec![Bin::new("px"), Bin::new("pv")];

        for key in keys {
            assert_eq!(
                result.get(&key),
                expected.get(&key),
                "Key {:?} did not match",
                key
            );
        }
    }

    #[test]
    fn parse_parts_test() {
        let input = indoc! {"
            {x=787,m=2655,a=1222,s=2876}
            {x=1679,m=44,a=2067,s=496}
        "};

        let (_res, result) = parse_parts(input).unwrap();
        assert_eq!(_res, "");

        let expected = vec![
            Part::new(787, 2655, 1222, 2876),
            Part::new(1679, 44, 2067, 496),
        ];

        assert_eq!(result, expected);
    }
}
