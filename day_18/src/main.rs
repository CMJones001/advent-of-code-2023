mod parser;
mod sparse_parser;
mod sparse_processing;

use ndarray::{s, Array2};
use std::cmp::Ordering;

use parser::parse;

fn main() {
    let problem_one_answer = problem_one();
    println!("Problem one answer: {}", problem_one_answer);

    let problem_one_expected = 48652;
    match problem_one_answer.cmp(&problem_one_expected) {
        Ordering::Greater => println!(
            "Problem one answer is too high, got {} expected {}",
            problem_one_answer, problem_one_expected
        ),
        Ordering::Less => println!(
            "Problem one answer is too low, got {} expected {}",
            problem_one_answer, problem_one_expected
        ),
        _ => {}
    }

    let problem_two_answer = sparse_processing::problem_two();
    println!("Problem two answer: {}", problem_two_answer);

    let problem_two_expected = 45757884535661;
    match problem_two_answer.cmp(&problem_two_expected) {
        Ordering::Greater => println!(
            "Problem two answer is too high, got {} expected {}",
            problem_two_answer, problem_two_expected
        ),
        Ordering::Less => println!(
            "Problem two answer is too low, got {} expected {}",
            problem_two_answer, problem_two_expected
        ),
        _ => {}
    }
}

fn problem_one() -> usize {
    let input = include_str!("problem_text");
    let inside_array = flood_fill_inside(input);

    let print = false;
    if print {
        let unicode = true;
        let inside_str = pretty_print_bool_arr(&inside_array, unicode);
        println!("Bricks inside:\n{}", inside_str);
    }

    let num_inside = inside_array.iter().filter(|&&b| b).count();
    num_inside
}

#[allow(clippy::reversed_empty_ranges)]
fn flood_fill_inside(input: &str) -> Array2<bool> {
    let path = get_boundary_points(input);
    let bricks = normalise_path(&path);
    let outline_array = create_outline_array(&bricks);

    // Pad the array with an empty row and column on each side
    // This ensures that the flood fill will fill the outside
    // of the array, and not the inside
    let expanded_shape = (outline_array.nrows() + 2, outline_array.ncols() + 2);
    let mut expanded_array = Array2::from_elem(expanded_shape, false);
    for ((x, y), &b) in outline_array.indexed_iter() {
        expanded_array[[x + 1, y + 1]] = b;
    }

    // Reduce the array back to its original size
    let outside_array = flood_fill(&expanded_array, 0, 0);
    // Clippy doesn't seem to like the indexing here, but I think this an abuse of notation by ndarray
    // so we disable the lint on this function
    let outside_array = outside_array.slice(s![1..-1, 1..-1]).to_owned();

    let inside_array = outside_array.mapv(|b| !b);
    inside_array | outline_array
}

fn get_boundary_points(input: &str) -> Vec<(i32, i32)> {
    let instructions = parse(input).unwrap();

    let path = instructions.iter().scan((0, 0), |(x, y), instruction| {
        let mut points = Vec::new();
        for _ in 0..instruction.distance {
            match instruction.direction {
                Direction::Up => *y += 1,
                Direction::Down => *y -= 1,
                Direction::Left => *x -= 1,
                Direction::Right => *x += 1,
            }
            points.push((*x, *y));
        }
        Some(points)
    });

    path.flatten().collect()
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Brick {
    x: usize,
    y: usize,
}

impl Brick {
    fn new(x: usize, y: usize) -> Brick {
        Brick { x, y }
    }
}

/// Convert a path of points into a list of bricks
///
/// This converts the i32 coordinates into usize coordinates, shifting them
/// so that the minimum x and y values are 0.
fn normalise_path(path: &[(i32, i32)]) -> Vec<Brick> {
    let min_x = path.iter().map(|(x, _)| x).min().unwrap();
    let min_y = path.iter().map(|(_, y)| y).min().unwrap();

    path.iter()
        .map(|(x, y)| Brick::new((x - min_x) as usize, (y - min_y) as usize))
        .collect()
}

fn create_outline_array(bricks: &[Brick]) -> Array2<bool> {
    let max_x = bricks.iter().map(|brick| brick.x).max().unwrap();
    let max_y = bricks.iter().map(|brick| brick.y).max().unwrap();

    let mut outline = Array2::from_elem((max_y + 1, max_x + 1), false);

    for brick in bricks {
        outline[[brick.y, brick.x]] = true;
    }

    outline
}

fn pretty_print_bool_arr(arr: &Array2<bool>, unicode: bool) -> String {
    let wall = if unicode { "█" } else { "#" };
    let space = if unicode { " " } else { "." };

    arr.outer_iter()
        .map(|row| {
            row.iter()
                .map(|&b| if b { wall } else { space })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn flood_fill(arr: &Array2<bool>, x: usize, y: usize) -> Array2<bool> {
    let mut stack = Vec::new();
    stack.push((x, y));

    let mut arr = arr.clone();

    while let Some((x, y)) = stack.pop() {
        // Skip if already filled
        if arr[[y, x]] {
            continue;
        }

        arr[[y, x]] = true;

        // Add neighbours to stack
        if x > 0 {
            stack.push((x - 1, y));
        }
        if x < arr.ncols() - 1 {
            stack.push((x + 1, y));
        }
        if y > 0 {
            stack.push((x, y - 1));
        }
        if y < arr.nrows() - 1 {
            stack.push((x, y + 1));
        }
    }

    arr
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Instruction {
    direction: Direction,
    distance: u32,
}

impl Instruction {
    fn new(direction: Direction, distance: u32) -> Instruction {
        Instruction {
            direction,
            distance,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    pub(crate) fn test_input() -> &'static str {
        indoc! {"
            R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)
        "}
    }

    #[test]
    fn test_create_outline() {
        let input = test_input();
        let path = get_boundary_points(input);
        let bricks = normalise_path(&path);

        let outline_arr = create_outline_array(&bricks);
        let outline = pretty_print_bool_arr(&outline_arr, false);

        let expected = indoc! {"
            .######
            .#....#
            ##..###
            #...#..
            ###.###
            ..#...#
            ..#...#
            ###...#
            #.....#
            #######
        "}
        .trim();
        assert_eq!(outline, expected);
    }

    #[test]
    fn test_flood_fill() {
        let input = test_input();
        let path = get_boundary_points(input);
        let bricks = normalise_path(&path);

        let outline_array = create_outline_array(&bricks);
        let filled = flood_fill(&outline_array, 2, 2);

        let expected = indoc! {"
            .######
            .######
            #######
            #####..
            #######
            ..#####
            ..#####
            #######
            #######
            #######
        "}
        .trim();

        let filled_str = pretty_print_bool_arr(&filled, false);
        assert_eq!(filled_str, expected);
    }
}
