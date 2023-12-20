use crate::problem_one::Command;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::multi::separated_list1;
use nom::IResult;
use nom::Parser;
use nom_supreme::parser_ext::ParserExt;
use std::fmt::Display;

pub(crate) fn problem_two() -> u64 {
    let input = include_str!("problem_text");
    get_total_box_power(input)
}

fn get_total_box_power(input: &str) -> u64 {
    let entries = get_entries(input).unwrap();
    let lens_boxes = get_lens_boxes(entries);
    get_lens_box_power(lens_boxes)
}

#[derive(Debug, PartialEq)]
struct Entry {
    label: Command,
    operation: Operation,
    focal_length: Option<u64>,
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.operation {
            Operation::Add => write!(f, "{}={}", self.label, self.focal_length.unwrap()),
            Operation::Remove => write!(f, "{}-", self.label),
        }
    }
}

impl Entry {
    fn new(label: &str, operation: Operation, focal_length: Option<u64>) -> Entry {
        Entry {
            label: Command::new(label),
            operation,
            focal_length,
        }
    }

    fn get_box(&self) -> usize {
        self.label.execute() as usize
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operation {
    Add,
    Remove,
}

fn get_lens_boxes(entries: Vec<Entry>) -> Vec<Vec<Entry>> {
    let mut lens_boxes: Vec<Vec<Entry>> = (0..256).map(|_| Vec::new()).collect();

    for e in entries {
        let box_index = e.get_box();
        match e.operation {
            Operation::Add => {
                let box_ = &mut lens_boxes[box_index];
                let index = box_.iter().position(|x| x.label == e.label);

                if let Some(i) = index {
                    box_[i] = e;
                } else {
                    lens_boxes[box_index].push(e)
                }
            }
            Operation::Remove => {
                let box_ = &mut lens_boxes[box_index];
                box_.retain(|x| x.label != e.label);
            }
        }
    }

    lens_boxes
}

fn get_lens_box_power(lens_box: Vec<Vec<Entry>>) -> u64 {
    lens_box
        .iter()
        .enumerate()
        .map(|(i, box_)| {
            let power = box_power(box_);
            power * (1 + i as u64)
        })
        .sum()
}

fn box_power(box_: &[Entry]) -> u64 {
    box_.iter()
        .enumerate()
        .map(|(i, e)| {
            let power = e.focal_length.unwrap_or(0);
            power * (1 + i as u64)
        })
        .sum()
}

fn get_entries(input: &str) -> Result<Vec<Entry>, &str> {
    let (rem, entries) = input_parser(input).unwrap();
    if !rem.is_empty() {
        return Err(rem);
    }
    Ok(entries)
}

fn input_parser(input: &str) -> IResult<&str, Vec<Entry>> {
    let input = input.trim();
    let mut parser = separated_list1(tag(","), parse_entry);

    parser.parse(input)
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    let label_parser = alpha1;

    let operation_parser = alt((
        tag("=").value(Operation::Add),
        tag("-").value(Operation::Remove),
    ));

    let focal_length_parser = nom::combinator::opt(nom::character::complete::u64);

    let mut entry_parser =
        nom::sequence::tuple((label_parser, operation_parser, focal_length_parser))
            .map(|(label, operation, focal_length)| Entry::new(label, operation, focal_length));

    entry_parser.parse(input)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_entry() {
        let input = "rn=1";
        let (_, entry) = parse_entry(input).unwrap();
        let expected = Entry::new("rn", Operation::Add, Some(1));
        assert_eq!(entry, expected);

        assert_eq!(entry.get_box(), 0);
    }

    #[test]
    fn test_two_steps() {
        let input = "rn=1,cm-";
        let entries = get_entries(input).unwrap();

        let expected = vec![
            Entry::new("rn", Operation::Add, Some(1)),
            Entry::new("cm", Operation::Remove, None),
        ];

        assert_eq!(entries, expected);

        let lens_boxes = get_lens_boxes(entries);
        assert_eq!(lens_boxes[0].len(), 1);

        assert!(lens_boxes[0].contains(&Entry::new("rn", Operation::Add, Some(1))));
    }

    #[test]
    fn test_six_steps() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4";
        let entries = get_entries(input).unwrap();

        let expected = vec![
            Entry::new("rn", Operation::Add, Some(1)),
            Entry::new("cm", Operation::Remove, None),
            Entry::new("qp", Operation::Add, Some(3)),
            Entry::new("cm", Operation::Add, Some(2)),
            Entry::new("qp", Operation::Remove, None),
            Entry::new("pc", Operation::Add, Some(4)),
        ];

        assert_eq!(entries, expected);

        let lens_boxes = get_lens_boxes(entries);
        assert_eq!(lens_boxes[0].len(), 2);
        assert_eq!(lens_boxes[1].len(), 0);
        assert_eq!(lens_boxes[2].len(), 0);
        assert_eq!(lens_boxes[3].len(), 1);

        assert!(lens_boxes[0].contains(&Entry::new("rn", Operation::Add, Some(1))));
        assert!(lens_boxes[0].contains(&Entry::new("cm", Operation::Add, Some(2))));
        assert!(lens_boxes[3].contains(&Entry::new("pc", Operation::Add, Some(4))));
    }

    #[test]
    fn test_sample_two() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        let entries = get_entries(input).unwrap();
        let lens_boxes = get_lens_boxes(entries);

        let box_3 = &lens_boxes[3];
        println!(
            "{}",
            box_3
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );

        assert_eq!(box_3[0], Entry::new("ot", Operation::Add, Some(7)));
        assert_eq!(box_3[1], Entry::new("ab", Operation::Add, Some(5)));
        assert_eq!(box_3[2], Entry::new("pc", Operation::Add, Some(6)));

        let box_powers = lens_boxes
            .iter()
            .map(|box_| box_power(box_))
            .collect::<Vec<u64>>();

        assert_eq!(box_powers[0], 5, "Box 0");
        assert_eq!(box_powers[1], 0, "Box 1");
        assert_eq!(box_powers[2], 0, "Box 2");
        assert_eq!(box_powers[3] * 4, 28 + 40 + 72, "Box 3");
        assert_eq!(box_powers[4], 0, "Box 4");

        let lens_box_power = get_lens_box_power(lens_boxes);
        assert_eq!(lens_box_power, 145);
    }
}
