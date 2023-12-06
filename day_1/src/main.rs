#![feature(iter_map_windows)]
use itertools::repeat_n;
use itertools::Itertools;

fn main() {
    problem_one();
    problem_two();
}

fn problem_one() {
    let text = include_str!("problem-one");
    let sum = get_sum_of_numbers(text);
    println!("Sum of numbers: {}", sum);
}

fn get_sum_of_numbers(text: &str) -> u32 {
    text.lines()
        .map(get_values)
        .fold(0, |acc, (a, b)| acc + 10 * a + b)
}

fn problem_two() {
    let text = include_str!("problem-one");
    let sum = text.lines().map(parse_text_line).fold(0, |acc, (a, b)| acc + 10*a + b);
    println!("Sum of numbers: {}", sum);
}

fn parse_text_line(line: &str) -> (u32, u32) {
    let first_value = line
        .chars()
        .chain(repeat_n('x', 4))
        .tuple_windows()
        .find_map(|(a, b, c, d, e)| {
            if let Some(digit) = a.to_digit(10) {
                Some(digit)
            } else {
                let word: String = [a, b, c, d, e].iter().collect();
                if word.starts_with("one") {
                    Some(1)
                } else if word.starts_with("two") {
                    Some(2)
                } else if word.starts_with("three") {
                    Some(3)
                } else if word.starts_with("four") {
                    Some(4)
                } else if word.starts_with("five") {
                    Some(5)
                } else if word.starts_with("six") {
                    Some(6)
                } else if word.starts_with("seven") {
                    Some(7)
                } else if word.starts_with("eight") {
                    Some(8)
                } else if word.starts_with("nine") {
                    Some(9)
                } else {
                    None
                }
            }
        }).expect("No first value found");

    let last_value = line
        .chars()
        .rev()
        .chain(repeat_n('x', 4))
        .tuple_windows()
        .find_map(|(a, b, c, d, e)| {
            if let Some(digit) = a.to_digit(10) {
                Some(digit)
            } else {
                let word: String = [a, b, c, d, e].iter().collect();
                if word.starts_with("eno") {
                    Some(1)
                } else if word.starts_with("owt") {
                    Some(2)
                } else if word.starts_with("eerht") {
                    Some(3)
                } else if word.starts_with("ruof") {
                    Some(4)
                } else if word.starts_with("evif") {
                    Some(5)
                } else if word.starts_with("xis") {
                    Some(6)
                } else if word.starts_with("neves") {
                    Some(7)
                } else if word.starts_with("thgie") {
                    Some(8)
                } else if word.starts_with("enin") {
                    Some(9)
                } else {
                    None
                }
            }
        }).expect("No last value found");

    (first_value, last_value)
}

fn get_values(line: &str) -> (u32, u32) {
    let chars = line.chars();

    let first_char = chars.clone().find(|c| c.is_digit(10)).unwrap();
    let last_char = chars.rev().find(|c| c.is_digit(10)).unwrap();

    (
        first_char.to_digit(10).unwrap(),
        last_char.to_digit(10).unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_text_line_first() {
        assert_eq!(parse_text_line("1abc2").0, 1);
        assert_eq!(
            parse_text_line("one62").0,
            1,
            "Should parse 'one' correctly"
        );
        assert_eq!(
            parse_text_line("xone2").0,
            1,
            "Should parse 'one' correctly (start)"
        );
        assert_eq!(
            parse_text_line("xtwoc4").0,
            2,
            "Should parse 'two' correctly"
        );
        assert_eq!(
            parse_text_line("xeight9").0,
            8,
            "Should parse 'two' correctly"
        );
    }

    #[test]
    fn test_parse_text_line_both() {
        let lines = indoc! {"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "};

        let expected_values = vec![
            (2, 9),
            (8, 3),
            (1, 3),
            (2, 4),
            (4, 2),
            (1, 4),
            (7, 6),
        ];

        for (line, expected) in lines.lines().zip(expected_values) {
            assert_eq!(parse_text_line(line), expected, "Line: {}", line);
        }


    }

    #[test]
    fn test_get_values() {
        assert_eq!(get_values("1abc2"), (1, 2));
        assert_eq!(get_values("pqr3stu8vwx"), (3, 8));
        assert_eq!(get_values("a1b2c3d4e5f"), (1, 5));
        assert_eq!(get_values("treb7uchet"), (7, 7));
    }

    #[test]
    fn test_sum_of_text() {
        let text = indoc! {"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "};

        assert_eq!(get_sum_of_numbers(text), 142);
    }
}
