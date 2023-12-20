#![feature(iter_intersperse)]

use std::hash::{Hasher, Hash};
use std::collections::{HashMap};
use ndarray::prelude::*;
use ndarray::Array;
use num_traits::Zero;
use std::ops::Add;

fn main() {
    let result = problem_one();
    println!("Problem 1: {}", result);

    let result = problem_two();
    println!("Problem 2: {}", result);
}

fn problem_one() -> usize {
    let input = include_str!("problem_text");
    let grid = parse_input(input);

    score_array(&shift_north(&grid))
}

fn problem_two() -> usize {
    let input = include_str!("problem_text");
    let grid = parse_input(input);

    get_cycle_shift_score(&grid, 1_000_000_000)
}

fn score_array(arr: &Array2<Stone>) -> usize {
    arr.axis_iter(Axis(1))
        .map(|col| score_series(&col.to_vec()))
        .sum()
}

fn score_series(vec: &[Stone]) -> usize {
    vec
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


/// Get the score of the grid after `target_cycle` cycles
///
/// As target cycle can be very large, we need to find the period of the cycle and
/// then interpolate the score from that.
fn get_cycle_shift_score(arr: &Array2<Stone>, target_cycle: u64) -> usize {
    let mut array = arr.clone();
    let mut count = 0;

    let mut hash_dict = HashMap::new();
    let mut cycle_length: Option<u64> = None;

    // The cycle may take some steps to appear, but once we have a repeat hash we
    // know this must be a cycle.

    // It's not enough to just use the score as the hash, as we can have different
    // configurations with the same score.

    loop {
        count += 1;
        array = cycle_shift(&array);

        if let Some(cycle_length) = cycle_length {
            // Once we have a cycle length, we can interpolate the score
            // At most we will have to do another cycle length steps
            //
            // This requires a few more steps, but the other solution is to create a second hash
            // map to store the score at each cycle length, which complicates the code a bit.
            // Or to store the hash of the array directly, both of which require more memory.
            if target_cycle % cycle_length == count % cycle_length {
                return score_array(&array);
            }
        } else {
            // If the cycle length is not known, we need to find it
            let array_hash = hash_array(&array);

            if let Some(cycle_start) = hash_dict.get(&array_hash) {
                // As the output of shifting the array depends only on the previous state,
                // we can find the cycle length by finding the first repeat
                cycle_length = Some(count - cycle_start);
                println!("Cycle length {} found at {}", cycle_length.unwrap(), count);
            } else {
                hash_dict.insert(array_hash, count);
            }
        }
    }
}

fn hash_array(arr: &Array2<Stone>) -> usize {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    arr.hash(&mut hasher);
    hasher.finish() as usize
}
fn cycle_shift(arr: &Array2<Stone>) -> Array2<Stone> {
    let shifted_north = shift_north(arr);
    let shifted_west = shift_west(&shifted_north);
    let shifted_south = shift_south(&shifted_west);
    let shifted_east = shift_east(&shifted_south);

    shifted_east
}
fn shift_north(arr: &Array2<Stone>) -> Array2<Stone> {
    let mut new_arr = arr.clone();

    new_arr.axis_iter_mut(Axis(1))
        .for_each(|mut col| {
            col.assign(&Array::from_iter(compress_series(&col.to_vec(), false)))
        });

    new_arr
}

fn shift_south(arr: &Array2<Stone>) -> Array2<Stone> {
    let mut new_arr = arr.clone();

    new_arr.axis_iter_mut(Axis(1))
        .for_each(|mut col| {
            col.assign(&Array::from_iter(compress_series(&col.to_vec(), true)))
        });

    new_arr
}

fn shift_east(arr: &Array2<Stone>) -> Array2<Stone> {
    let mut new_arr = arr.clone();

    new_arr.axis_iter_mut(Axis(0))
        .for_each(|mut row| {
            row.assign(&Array::from_iter(compress_series(&row.to_vec(), true)))
        });

    new_arr
}

fn shift_west(arr: &Array2<Stone>) -> Array2<Stone> {
    let mut new_arr = arr.clone();

    new_arr.axis_iter_mut(Axis(0))
        .for_each(|mut row| {
            row.assign(&Array::from_iter(compress_series(&row.to_vec(), false)))
        });

    new_arr
}

fn compress_series(vec: &[Stone], reverse: bool) -> Vec<Stone> {
    // Can we make this more general for arrays?
    vec.split(|&x| x == Stone::Cube)
        .map(|x| compress_split(x, reverse))
        .intersperse(vec![Stone::Cube])
        .flatten()
        .collect()
}

fn compress_split(vec: &[Stone], reverse: bool) -> Vec<Stone> {
    let n_round = vec.iter().filter(|&&x| x == Stone::Round).count();
    let n_elements = vec.len();


    let iter = (0..n_elements)
        .map(|i| {
            if i < n_round {
                Stone::Round
            } else {
                Stone::Ground
            }
        });

    if reverse {
        iter.rev().collect()
    } else {
        iter.collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
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
            Stone::Ground => '.',
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
    use test_case::test_case;

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
        "};
        assert_eq!(input_str, expected_string);
    }

    #[test]
    fn test_compress_split() {
        let input_str = "OO.O.O.";
        let input = input_str.chars().map(Stone::from_char).collect::<Vec<_>>();
        let compressed_str = compress_split(&input, false)
            .iter()
            .map(Stone::to_char)
            .collect::<String>();

        let expected_str = "OOOO...";
        assert_eq!(compressed_str, expected_str);
    }

    #[test_case("OO.O.O.#O.O#...O", "OOOO...#OO.#O..."; "Simple test case")]
    #[test_case("OO.O.O.#O.O#...O#", "OOOO...#OO.#O...#"; "With termination")]
    #[test_case("#OO.O.O.#O.O#...O#", "#OOOO...#OO.#O...#"; "With termination and start")]
    fn test_compress_series(input_str: &str, expected_str: &str) {
        let input = input_str.chars().map(Stone::from_char).collect::<Vec<_>>();
        let compressed_str = compress_series(&input, false)
            .iter()
            .map(Stone::to_char)
            .collect::<String>();

        assert_eq!(compressed_str, expected_str);
    }

    #[test_case("OO.O.O.#O.O#...O", "...OOOO#.OO#...O"; "Simple test case")]
    #[test_case("#OO.O.O.#O.O#...O", "#...OOOO#.OO#...O"; "With start")]
    #[test_case("#OO.O.O.#O.O#...O#", "#...OOOO#.OO#...O#"; "With start and termination")]
    fn test_compress_series_rev(input_str: &str, expected_str: &str) {
        let input = input_str.chars().map(Stone::from_char).collect::<Vec<_>>();

        let compressed_str = compress_series(&input, true)
            .iter()
            .map(Stone::to_char)
            .collect::<String>();

        assert_eq!(compressed_str, expected_str);
    }

    #[test_case("OO.O.O..##", 10 + 9 + 8 + 7; "Sample col 0")]
    #[test_case("....O#.O#.", 14; "Sample col 3")]
    fn test_score_columns(input_str: &str, expected_score: usize) {
        let input = input_str.chars().map(Stone::from_char).collect::<Vec<_>>();

        let score = score_series(&compress_series(&input, false));

        assert_eq!(score, expected_score);
    }

    #[test]
    fn test_shift_north() {
        let shifted_str = array_to_string(&shift_north(&parse_input(sample_input())));
        let expected_str = indoc! {"\
            OOOO.#.O..
            OO..#....#
            OO..O##..O
            O..#.OO...
            ........#.
            ..#....#.#
            ..O..#.O.O
            ..O.......
            #....###..
            #....#....
        " };

        assert_eq!(shifted_str, expected_str);
    }

    #[test]
    fn test_score_array() {
        let input = parse_input(sample_input());
        let score = score_array(&shift_north(&input));

        assert_eq!(score, 136);
    }

    #[test]
    fn test_cycle_shift() {
        let input = parse_input(sample_input());
        let shifted = cycle_shift(&input);
        let shifted_str = array_to_string(&shifted);

        let expected_str = indoc! {"\
            .....#....
            ....#...O#
            ...OO##...
            .OO#......
            .....OOO#.
            .O#...O#.#
            ....O#....
            ......OOOO
            #...O###..
            #..OO#....
        " };

        assert_eq!(shifted_str, expected_str, "Cycle 1");

        let shifted = cycle_shift(&shifted);
        let shifted_str = array_to_string(&shifted);

        let expected_str = indoc!{"
            .....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #..OO###..
            #.OOO#...O
        "};
        assert_eq!(shifted_str, expected_str, "Cycle 2");

    }

    #[test]
    fn test_problem_two() {
        let sample_array = parse_input(sample_input());

        let score = get_cycle_shift_score(&sample_array, 1_000_000_000);

        assert_eq!(score, 64);
    }
}
