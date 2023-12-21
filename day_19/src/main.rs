mod parsing;
mod problem_two;

use std::collections::HashMap;
use std::fmt::Display;
use std::result;

fn main() {
    let result_one = problem_one();
    println!("Problem one: {}", result_one);

    let result_two = problem_two::problem_two();
    println!("Problem two: {}", result_two);

}

fn problem_one() -> u64 {
    let input = include_str!("problem_text");
    get_rating_numbers(input)
}

fn get_rating_numbers(input: &str) -> u64 {
    let (rem, filters) = parsing::parse_filters_rows(input).unwrap();
    let (_, parts) = parsing::parse_parts(rem.trim()).unwrap();

    parts
        .into_iter()
        .filter(|part| get_final_bin(part, &filters) == Bin::Accept)
        .map(|part| part.total())
        .sum()
}

/// Returns the final bin that the part ends up in, this is either Accept or Reject
///
/// If the part ends up in a label bin, this function will loop until it finds
/// an Accept or Reject bin
fn get_final_bin(part: &Part, filters: &HashMap<Bin, FilterList>) -> Bin {
    let starting_filter = filters.get(&Bin::new("in")).unwrap();
    let mut current_filter = starting_filter;

    loop {
        let next_bin = part.get_resulting_bin(current_filter);
        if next_bin == Bin::Accept || next_bin == Bin::Reject {
            return next_bin;
        }
        current_filter = filters.get(&next_bin).unwrap();
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn new(x: u64, m: u64, a: u64, s: u64) -> Part {
        Part { x, m, a, s }
    }

    /// Returns the bin that the part ends up in after this filter
    fn get_resulting_bin(&self, filter_list: &FilterList) -> Bin {
        filter_list
            .0
            .iter()
            .find_map(|filter| {
                if filter.test_part(self) {
                    Some(filter.bin())
                } else {
                    None
                }
            })
            .unwrap_or(Bin::Reject)
    }

    fn total(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
enum Bin {
    Accept,
    Reject,
    Label(String),
}

impl Bin {
    fn new(s: &str) -> Bin {
        match s {
            "A" => Bin::Accept,
            "R" => Bin::Reject,
            x => Bin::Label(x.to_string()),
        }
    }
}

impl Display for Bin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bin::Accept => write!(f, " ACC"),
            Bin::Reject => write!(f, " REJ"),
            Bin::Label(s) => write!(f, "{:>4}", s),
        }
    }
}

#[derive(PartialEq, Debug)]
struct FilterList(Vec<Filter>);

#[derive(PartialEq, Debug)]
enum Filter {
    GreaterThan(u64, PartType, Bin),
    LessThan(u64, PartType, Bin),
    Unconditional(Bin),
}

impl Filter {
    fn new_greater_than(value: u64, part_type: PartType, bin: Bin) -> Filter {
        Filter::GreaterThan(value, part_type, bin)
    }

    fn new_less_than(value: u64, part_type: PartType, bin: Bin) -> Filter {
        Filter::LessThan(value, part_type, bin)
    }

    fn new_unconditional(bin: Bin) -> Filter {
        Filter::Unconditional(bin)
    }

    fn bin(&self) -> Bin {
        match self {
            Filter::GreaterThan(_, _, bin) => bin.clone(),
            Filter::LessThan(_, _, bin) => bin.clone(),
            Filter::Unconditional(bin) => bin.clone(),
        }
    }

    /// Returns true if the part passes the filter
    fn test_part(&self, part: &Part) -> bool {
        match self {
            Filter::GreaterThan(value, part_type, _bin) => match part_type {
                PartType::X => part.x > *value,
                PartType::M => part.m > *value,
                PartType::A => part.a > *value,
                PartType::S => part.s > *value,
            },
            Filter::LessThan(value, part_type, _bin) => match part_type {
                PartType::X => part.x < *value,
                PartType::M => part.m < *value,
                PartType::A => part.a < *value,
                PartType::S => part.s < *value,
            },
            Filter::Unconditional(_bin) => true,
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum PartType {
    X,
    M,
    A,
    S,
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use test_case::test_case;

    pub(crate) fn filter_string() -> &'static str {
        indoc! {"
            px{a<2006:qkq,m>2090:A,rfg}
            pv{a>1716:R,A}
            lnx{m>1548:A,A}
            rfg{s<537:gd,x>2440:R,A}
            qs{s>3448:A,lnx}
            qkq{x<1416:A,crn}
            crn{x>2662:A,R}
            in{s<1351:px,qqz}
            qqz{s>2770:qs,m<1801:hdj,R}
            gd{a>3333:R,R}
            hdj{m>838:A,pv}
        "}
    }

    #[test]
    fn test_parse_filter() {
        let filter_str = filter_string();
        let (_res, filter_dict) = parsing::parse_filters_rows(filter_str).unwrap();
        let starting_filter = filter_dict.get(&Bin::new("in")).unwrap();

        let part = Part::new(787, 2655, 1222, 2876);

        let resulting_bin = part.get_resulting_bin(starting_filter);
        let expected_bin = Bin::Label("qqz".to_string());

        assert_eq!(resulting_bin, expected_bin);

        let filter = filter_dict.get(&expected_bin).unwrap();
        let expected_bin = Bin::Label("qs".to_string());

        let resulting_bin = part.get_resulting_bin(filter);

        assert_eq!(resulting_bin, expected_bin);

        let filter = filter_dict.get(&expected_bin).unwrap();
        let expected_bin = Bin::Label("lnx".to_string());

        let resulting_bin = part.get_resulting_bin(filter);

        assert_eq!(resulting_bin, expected_bin);

        let filter = filter_dict.get(&expected_bin).unwrap();
        let expected_bin = Bin::Accept;

        let resulting_bin = part.get_resulting_bin(filter);

        assert_eq!(resulting_bin, expected_bin);
    }

    #[test_case(787, 2655, 1222, 2876, Bin::Accept)]
    #[test_case(1679, 44, 2067, 496, Bin::Reject)]
    #[test_case(2036, 264, 79, 2244, Bin::Accept)]
    fn test_final_bins(x: u64, m: u64, a: u64, s: u64, expected_bin: Bin) {
        let part = Part::new(x, m, a, s);
        let filter_str = filter_string();

        let (_res, filter_dict) = parsing::parse_filters_rows(filter_str).unwrap();
        let final_bin = get_final_bin(&part, &filter_dict);

        assert_eq!(final_bin, expected_bin);
    }

    #[test]
    fn sample_one() {
        let input = indoc! {"
            px{a<2006:qkq,m>2090:A,rfg}
            pv{a>1716:R,A}
            lnx{m>1548:A,A}
            rfg{s<537:gd,x>2440:R,A}
            qs{s>3448:A,lnx}
            qkq{x<1416:A,crn}
            crn{x>2662:A,R}
            in{s<1351:px,qqz}
            qqz{s>2770:qs,m<1801:hdj,R}
            gd{a>3333:R,R}
            hdj{m>838:A,pv}

            {x=787,m=2655,a=1222,s=2876}
            {x=1679,m=44,a=2067,s=496}
            {x=2036,m=264,a=79,s=2244}
            {x=2461,m=1339,a=466,s=291}
            {x=2127,m=1623,a=2188,s=1013}
        "};

        assert_eq!(get_rating_numbers(input), 19114);
    }
}
