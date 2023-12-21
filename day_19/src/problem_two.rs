use crate::{Bin, Filter, FilterList, Part, PartType};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

fn process_filter_set(input: &str) -> HistMap {
    let (_, filters) = crate::parsing::parse_filters_rows(input).unwrap();
    let mut range = PartRange::from_range(1, 4000);
    let mut hist_map = HistMap::new();

    // Create an initial entry for the history map
    let mut filter = filters.get(&Bin::new("in")).unwrap();
    let mut range_map = filter.split_range(range);

    hist_map.extend(&range_map);
    println!("{}", hist_map);

    loop {
        let mut new_hist_map = HistMap::new();

        // Go through the current history map and split each range
        for (bin, ranges) in hist_map.map.iter() {
            match bin {
                Bin::Accept | Bin::Reject => {
                    // For the accept and reject bins, just copy the ranges
                    for range in ranges {
                        new_hist_map.insert(bin.clone(), *range);
                    }
                }
                _ => {
                    // For the label bins, split the ranges
                    let filter = filters.get(&bin).expect("Filter not found");
                    for range in ranges {
                        let mut range_map = filter.split_range(*range);
                        new_hist_map.extend(&range_map);
                    }
                }
            }
        }

        println!("{}", new_hist_map);

        // If everything is in the accept/reject bins then we are done
        if new_hist_map.count_non_empty_labels() == 0 {
            return new_hist_map
        }
        hist_map = new_hist_map.clone();
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartRange {
    fn from_max(val: u64) -> PartRange {
        PartRange {
            x: Range::from_upper(val),
            m: Range::from_upper(val),
            a: Range::from_upper(val),
            s: Range::from_upper(val),
        }
    }

    fn from_range(min: u64, max: u64) -> PartRange {
        PartRange {
            x: Range::new(max, min),
            m: Range::new(max, min),
            a: Range::new(max, min),
            s: Range::new(max, min),
        }
    }

    /// Replace the given range at the given part of the part range.
    fn replace_range(&self, range: Range, part_type: PartType) -> PartRange {
        let new = Self {
            x: self.x,
            m: self.m,
            a: self.a,
            s: self.s,
        };
        match part_type {
            PartType::X => Self {
                x: range,
                ..new
            },
            PartType::M => Self {
                m: range,
                ..new
            },
            PartType::A => Self {
                a: range,
                ..new
            },
            PartType::S => Self {
                s: range,
                ..new
            },
        }
    }

    /// Split the part range into accepted and rejected ranges, based on the given filter.
    ///
    /// If any part of the range is empty then the entire range is None.
    fn split(&self, filter: &Filter) -> (Option<PartRange>, Option<PartRange>) {
        match filter {
            Filter::GreaterThan(value, part_type, _bin) => {
                let (accepted, rejected) = match part_type {
                    PartType::X => self.x.greater_than(*value),
                    PartType::M => self.m.greater_than(*value),
                    PartType::A => self.a.greater_than(*value),
                    PartType::S => self.s.greater_than(*value),
                };

                let accepted = accepted.map(|range| self.replace_range(range, *part_type));
                let rejected = rejected.map(|range| self.replace_range(range, *part_type));

                (accepted, rejected)
            },
            Filter::LessThan(value, part_type, _bin) => {
                let (accepted, rejected) = match part_type {
                    PartType::X => self.x.less_than(*value),
                    PartType::M => self.m.less_than(*value),
                    PartType::A => self.a.less_than(*value),
                    PartType::S => self.s.less_than(*value),
                };

                let accepted = accepted.map(|range| self.replace_range(range, *part_type));
                let rejected = rejected.map(|range| self.replace_range(range, *part_type));

                (accepted, rejected)
            }
            Filter::Unconditional(_bin) => (Some(*self), None),
        }
    }

    fn get_combinations(&self) -> u64 {
        self.x.length() * self.m.length() * self.a.length() * self.s.length()
    }
}

impl Display for PartRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, m: {}, a: {}, s: {}", self.x, self.m, self.a, self.s)
    }
}


#[derive(PartialEq, Debug, Copy, Clone)]
struct Range {
    upper: u64,
    lower: u64,
}

impl Range {
    fn new(upper: u64, lower: u64) -> Range {
        Range { upper, lower }
    }

    fn from_upper(upper: u64) -> Range {
        Range { upper, lower: 0 }
    }

    fn length(&self) -> u64 {
        // Our ranges are inclusive, so we need to add 1
        // (1, 1) is a range of 1
        self.upper - self.lower + 1
    }

    /// Split the range into accepted and rejected ranges, based on being less than the given value.
    fn less_than(&self, value: u64) -> (Option<Self>, Option<Self>) {
        // If the new upper bound is less than the lower bound, then all values are rejected
        if value < self.lower {
            return (None, Some(*self));
        }

        // If the new upper bound is greater than the current upper bound, then all values are accepted
        if value > self.upper {
            return (Some(*self), None);
        }

        let mut accepted = self.clone();
        let mut rejected = self.clone();

        accepted.upper = value - 1;
        rejected.lower = value;

        (Some(accepted), Some(rejected))
    }

    ///
    fn greater_than(&self, value: u64) -> (Option<Self>, Option<Self>) {
        // If the new lower bound is greater than the upper bound, then all values are rejected
        if value > self.upper {
            return (None, Some(self.clone()));
        }

        // If the new lower bound is less than the current lower bound, then all values are accepted
        if value < self.lower {
            return (Some(self.clone()), None);
        }

        let mut accepted = self.clone();
        let mut rejected = self.clone();

        accepted.lower = value + 1;
        rejected.upper = value;

        (Some(accepted), Some(rejected))
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.lower == self.upper {
            write!(f, "{}", self.lower)
        } else {
            write!(f, "{:4}-{:4}", self.lower, self.upper)
        }
    }
}

pub(crate) fn problem_two() -> u64 {
    let input = include_str!("problem_text");
    let hist_map = process_filter_set(input);

    0
}

#[derive(Debug, Clone)]
struct HistMap {
    map: HashMap<Bin, Vec<PartRange>>,
}

impl HistMap {
    fn new() -> Self {
        let mut map = HashMap::new();
        Self { map }
    }

    fn insert(&mut self, bin: Bin, range: PartRange) {
        self.map.entry(bin).or_insert_with(Vec::new).push(range);
    }

    fn extend(&mut self, map: &HashMap<Bin, PartRange>) {
        for (bin, range) in map {
            self.insert(bin.clone(), *range);
        }
    }

    /// Count the number of non-empty labels in the map
    fn count_non_empty_labels(&self) -> usize {
        self.map.iter().filter(|(b, v)|
            match b {
                Bin::Accept => false,
                Bin::Reject => false,
                _ => v.len() > 0,
            }
        ).count()
    }
}

impl Display for HistMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (bin, ranges) in self.map.iter() {
            writeln!(f, "{}:", bin)?;
            for range in ranges {
                writeln!(f, " {}", range)?;
            }
        }
        writeln!(f, "--")?;
        Ok(())
    }
}


impl FilterList {
    /// Split the ranges into given bins based on the filters.
    fn split_range(&self, range: PartRange) -> HashMap<Bin, PartRange> {
        let mut remaining_range = range.clone();
        let mut map = HashMap::new();

        for filter in &self.0 {
            let (accepted, range) = remaining_range.split(filter);
            map.insert(filter.bin(), accepted.unwrap());
            if let Some(range) = range {
                remaining_range = range;
            } else {
                break;
            }
        }

        map
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    impl PartRange {
        fn contains_part(&self, part: &Part) -> bool {
            self.x.contains(part.x) && self.m.contains(part.m) && self.a.contains(part.a) && self.s.contains(part.s)
        }
    }

    impl Range {
        fn contains(&self, value: u64) -> bool {
            value >= self.lower && value <= self.upper
        }

    }

    /// We should be able to recreate part one of the problem using this method
    #[test_case(787, 2655, 1222, 2876, Bin::Accept)]
    #[test_case(1679, 44, 2067, 496, Bin::Reject)]
    #[test_case(2036, 264, 79, 2244, Bin::Accept)]
    #[test_case(2461, 1339, 466, 291, Bin::Reject)]
    #[test_case(2172, 1623, 2188, 1013, Bin::Accept)]
    fn test_sample_one(x: u64, m: u64, a: u64, s: u64, expected: Bin) {
        let range = PartRange::from_range(1,4000);
        let filter_string = crate::test::filter_string();

        let output = process_filter_set(filter_string);
        let accepted = output.map.get(&Bin::Accept).unwrap();

        let part = Part::new(x, m, a, s);
        let part_accepted = accepted.iter().any(|r| r.contains_part(&part));

        if expected == Bin::Accept {
            assert!(part_accepted, "Part should be accepted");
        } else {
            assert!(!part_accepted, "Part should be rejected");
        }
    }

    /// Test part one, by seeing if the part not in the rejected range
    #[test_case(787, 2655, 1222, 2876, Bin::Accept)]
    #[test_case(1679, 44, 2067, 496, Bin::Reject)]
    #[test_case(2036, 264, 79, 2244, Bin::Accept)]
    #[test_case(2461, 1339, 466, 291, Bin::Reject)]
    #[test_case(2172, 1623, 2188, 1013, Bin::Accept)]
    fn test_sample_one_inv(x: u64, m: u64, a: u64, s: u64, expected: Bin) {
        let range = PartRange::from_range(1,4000);
        let filter_string = crate::test::filter_string();

        let output = process_filter_set(filter_string);
        let rejected = output.map.get(&Bin::Reject).unwrap();

        let part = Part::new(x, m, a, s);
        let part_rejected = rejected.iter().any(|r| r.contains_part(&part));

        if expected == Bin::Accept {
            assert!(!part_rejected, "Part should be accepted");
        } else {
            assert!(part_rejected, "Part should be rejected");
        }
    }

    #[test]
    fn test_bisect_range() {
        let (_, filter_range) =
            crate::parsing::parse_filter_list("a<2006:qkq,m>2090:A,rfg").unwrap();
        let range = PartRange::from_range(0,4000);

        let map = filter_range.split_range(range);

        let mut expected = HashMap::new();
        expected.insert(
            Bin::new("qkq"),
            PartRange {
                x: Range::new(4000, 0),
                m: Range::new(4000, 0),
                a: Range::new(2005, 0),
                s: Range::new(4000, 0),
            },
        );
        expected.insert(
            Bin::Accept,
            PartRange {
                x: Range::new(4000, 0),
                m: Range::new(4000, 2091),
                a: Range::new(4000, 2006),
                s: Range::new(4000, 0),
            },
        );
        expected.insert(
            Bin::new("rfg"),
            PartRange {
                x: Range::new(4000, 0),
                m: Range::new(2090, 0),
                a: Range::new(4000, 2006),
                s: Range::new(4000, 0),
            },
        );

        let keys = expected.keys().collect::<Vec<_>>();
        for key in keys {
            assert_eq!(map[&key], expected[&key], "Key: {:?}", key);
        }
    }

    #[test]
    fn test_redundant_filter() {
        let range = PartRange::from_max(4000);

        // Create a sample range that has x >= 500
        let (upper_range, _lower_range) =
            range.split(&Filter::new_greater_than(499, PartType::X, Bin::new("qkq")));
        let upper_range = upper_range.unwrap();

        assert_eq!(
            upper_range,
            PartRange {
                x: Range::new(4000, 500),
                m: Range::new(4000, 0),
                a: Range::new(4000, 0),
                s: Range::new(4000, 0)
            },
            "Upper range"
        );

        // Now we do a redundant split of x >= 300
        let filter = Filter::new_greater_than(300, PartType::X, Bin::new("qkq"));
        let (accepted, rejected) = upper_range.split(&filter);

        // The new range should be the same as before
        assert_eq!(
            accepted.unwrap(),
            PartRange {
                x: Range::new(4000, 500),
                m: Range::new(4000, 0),
                a: Range::new(4000, 0),
                s: Range::new(4000, 0)
            },
            "Accepted range should be the same"
        );

        // The rejection is None
        assert_eq!(rejected, None, "No rejection after redundant split");
    }

    #[test]
    fn test_complete_filter() {
        let range = PartRange::from_max(4000);

        // Create a sample range that has x >= 500
        let (upper_range, _lower_range) =
            range.split(&Filter::new_greater_than(499, PartType::X, Bin::new("qkq")));
        let upper_range = upper_range.unwrap();


        // Now we do a complete filter of x <= 300
        let filter = Filter::new_less_than(300, PartType::X, Bin::new("qkq"));
        let (accepted, rejected) = upper_range.split(&filter);

        assert_eq!(accepted, None, "No accepted range");

        // The rejection is the same as the original range
        assert_eq!(
            rejected.unwrap(),
            PartRange {
                x: Range::new(4000, 500),
                m: Range::new(4000, 0),
                a: Range::new(4000, 0),
                s: Range::new(4000, 0)
            },
            "Rejection should be the same"
        );

    }

    // In this filter, both outcomes go to the same bin
    #[test]
    fn test_tautological_filter() {
        let range = PartRange::from_range(1, 4000);
        let filter_string = "m>1548:A,A";

        let (_, filter_range) =
            crate::parsing::parse_filter_list(filter_string).unwrap();

        let map = filter_range.split_range(range);

        let mut expected = HashMap::new();

        expected.insert(
            Bin::new("A"),
            PartRange {
                x: Range::new(4000, 1),
                m: Range::new(4000, 1549),
                a: Range::new(4000, 1),
                s: Range::new(4000, 1),
            },
        );


        assert!(false)


    }

    // #[test]
    fn test_filter_set() {
        let range = PartRange::from_range(1,4000);
        let filter_string = crate::test::filter_string();

        let output = process_filter_set(filter_string);
        let accepted = output.map.get(&Bin::Accept).unwrap();
        let rejected = output.map.get(&Bin::Reject).unwrap();

        let rejected_score = rejected.iter().map(|r| r.get_combinations()).sum::<u64>();
        let rejected_score = 3999_u64.pow(4) - rejected_score;

        println!("Rejected score: {}", rejected_score);

        for a in accepted {
            println!("{}", a);
        }

        let combinations = accepted.iter().map(|a| a.get_combinations()).sum::<u64>();
        assert_eq!(combinations, 167409079868000, "Score")
    }
}
