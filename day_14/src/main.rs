#![feature(iter_intersperse)]

use ndarray::prelude::*;
use ndarray::Array;
use num_traits::Zero;
use std::ops::Add;

fn main() {
    let result = problem_one();
    println!("Problem 1: {}", result);
}

fn problem_one() -> usize {
    let input = include_str!("problem_text");
    let grid = parse_input(input);
    shift_north(&grid)
}

fn shift_north(arr: &Array2<Stone>) -> usize {
    arr.axis_iter(Axis(1))
        .map(|col| score_series(&col.to_vec()))
        .sum()
}

fn score_series(vec: &[Stone]) -> usize {
    compress_series(vec)
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, &x)| {
            if x == Stone::Round {
                acc + (i+1)
            } else {
                acc
            }
        })
}
fn compress_series(vec: &[Stone]) -> Vec<Stone> {
    vec.split(|&x| x == Stone::Cube)
        .map(compress_split)
        .intersperse(vec![Stone::Cube])
        .flatten()
        .collect()
}

fn compress_split(vec: &[Stone]) -> Vec<Stone> {
    let n_round = vec.iter().filter(|&&x| x == Stone::Round).count();
    let n_elements = vec.len();

    (0..n_elements)
        .map(|i| {
            if i < n_round {
                Stone::Round
            } else {
                Stone::Ground
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Stone {
    Round,
    Cube,
    Ground,
}

impl Stone {
    fn to_char(&self) -> char {
        match self {
            Stone::Round => 'O',
            Stone::Cube => '#',
            Stone::Ground => '·',
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            'O' => Stone::Round,
            '#' => Stone::Cube,
            '.' => Stone::Ground,
            _ => {
                eprintln!("Invalid character: {}", c);
                panic!("Invalid character")
            }
        }
    }
}

impl Zero for Stone {
    fn zero() -> Self {
        Stone::Ground
    }

    fn is_zero(&self) -> bool {
        *self == Stone::Ground
    }
}

impl Add for Stone {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Stone::Round, Stone::Round) => Stone::Round,
            (Stone::Cube, Stone::Cube) => Stone::Cube,
            _ => Stone::Ground,
        }
    }
}

fn array_to_string(array: &Array2<Stone>) -> String {
    let mut string = String::new();

    for row in array.rows() {
        for stone in row {
            string.push(stone.to_char());
        }
        string.push('\n');
    }

    string
}

fn parse_input(input: &str) -> Array2<Stone> {
    let n_rows = input.lines().count();
    let n_cols = input.lines().next().unwrap().chars().count();

    let mut grid = Array::zeros((n_rows, n_cols));

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[[i, j]] = match c {
                'O' => Stone::Round,
                '#' => Stone::Cube,
                '.' => Stone::Ground,
                _ => panic!("Invalid character"),
            }
        }
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use std::vec;

    fn sample_input() -> &'static str {
        indoc! {"\
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        "}
    }

    #[test]
    fn test_parse_input() {
        let input = sample_input();
        let input_str = array_to_string(&parse_input(input));

        println!("{}", input_str);

        let expected_string = indoc! {"\
            O····#····
            O·OO#····#
            ·····##···
            OO·#O····O
            ·O·····O#·
            O·#··O·#·#
            ··O··#O··O
            ·······O··
            #····###··
            #OO··#····
        "};
        assert_eq!(input_str, expected_string);
    }

    #[test]
    fn test_compress_split() {
        let input_str = "OO.O.O.";
        let input = input_str.chars().map(Stone::from_char).collect::<Vec<_>>();
        let compressed_str = compress_split(&input)
            .iter()
            .map(Stone::to_char)
            .collect::<String>();

        let expected_str = "OOOO···";
        assert_eq!(compressed_str, expected_str);
    }

    #[test]
    fn test_compress_series() {
        let input_str = "OO.O.O.#O.O#...O#";
        let input = input_str.chars().map(Stone::from_char).collect::<Vec<_>>();
        let compressed_str = compress_series(&input)
            .iter()
            .map(Stone::to_char)
            .collect::<String>();

        let expected_str = "OOOO···#OO·#O···#";
        assert_eq!(compressed_str, expected_str);
    }

    #[test]
    fn test_compress_series_alt() {
        let input_str = "OO.O.O.#O.O#...O";
        let input = input_str.chars().map(Stone::from_char).collect::<Vec<_>>();

        let compressed_vecs = compress_series(&input);

        let expected_str = "OOOO...#OO.#O...";
        let expected = expected_str
            .chars()
            .map(Stone::from_char)
            .collect::<Vec<_>>();

        assert_eq!(expected, compressed_vecs)
    }

    #[test]
    fn test_score_columns() {
        let input_str = "OO.O.O..##";
        let input = input_str.chars().map(Stone::from_char).collect::<Vec<_>>();

        let score = score_series(&input);
        let score_expected = 10 + 9 + 8 + 7;

        assert_eq!(score, score_expected);

        let input_str = "....O#.O#.";
        let input = input_str.chars().map(Stone::from_char).collect::<Vec<_>>();

        let score = score_series(&input);
        let score_expected = 14;

        assert_eq!(score, score_expected);
    }

    #[test]
    fn score_array() {
        let input = parse_input(sample_input());
        let score = shift_north(&input);

        assert_eq!(score, 136);
    }
}
