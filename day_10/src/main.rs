#![allow(dead_code)]
use itertools::Itertools;
use std::cmp::{max, min};
use std::fmt::Display;

fn main() {
    problem_one();
    problem_two();
}

fn problem_one() {
    let input = include_str!("problem_text");
    let (steps, _) = walk_map(input);

    println!("Problem one: {steps}");
}

fn problem_two() {
    let input = include_str!("problem_text");
    let (_, filtered_map) = walk_map(input);

    let inside_count = map_inside(&filtered_map);
    println!("Problem two: {inside_count}");
}

fn walk_map(input: &str) -> (u32, Map) {
    let map = Map::parse_input(input);
    let mut steps = 1;

    let mut filtered_map = map.empty_like();
    let (mut new_direction, mut new_coord, mut new_pipe) = map.get_first_step();
    let first_direction = new_direction;

    loop {
        filtered_map.set(&new_coord, new_pipe);

        new_direction = new_pipe
            .exit(&new_direction.compliment())
            .expect("Unable to exit pipe");
        new_coord = new_coord
            .step(&new_direction)
            .expect("Attempting to step off map");
        new_pipe = map.get(&new_coord);
        steps += 1;

        if new_pipe == Pipe::Start {
            let replacement_start =
                Pipe::from_directions(&new_direction.compliment(), &first_direction);
            filtered_map.set(&new_coord, replacement_start);
            break;
        }
    }

    (get_farthest_distance(steps), filtered_map)
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    Outside,
    Inside,
    Wall,
}

impl State {
    fn flip(&self) -> Option<Self> {
        use State::{Inside, Outside, Wall};

        match self {
            Inside => Some(Outside),
            Outside => Some(Inside),
            Wall => None,
        }
    }

    fn to_char(&self) -> char {
        use State::{Inside, Outside, Wall};

        match self {
            Outside => ' ',
            Inside => 'I',
            Wall => '█',
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum WallHistory {
    None,
    North,
    South,
}

/// Count the number of cells that are inside the curve.
///
/// This is done by counting the number of wall transitions.
/// We count on a row by row basis. If a vertical wall is encountered, we
/// flip the state. If a corner is encountered, we check if the wall is
/// ending opposite the direction that it started. If so, we flip the state.
///
/// This also pretty prints the map with the inside regions annotated.
fn map_inside(filtered_map: &Map) -> usize {
    use State::Wall;
    let mut outside_map: Vec<char> = Vec::with_capacity(filtered_map.pipes.len());
    let mut history = WallHistory::None;

    for row in filtered_map.pipes.chunks(filtered_map.width) {
        let mut state = State::Outside;
        for pipe in row {
            match (pipe, history) {
                (Pipe::Vertical, _) => {
                    state = state.flip().unwrap();
                    outside_map.push(pipe.to_char());
                }
                (Pipe::BendNE | Pipe::BendNW, WallHistory::None) => {
                    // Entering a wall
                    history = WallHistory::North;
                    outside_map.push(pipe.to_char());
                }
                (Pipe::BendNE | Pipe::BendNW, WallHistory::South) => {
                    // Wall is ending on a switch
                    outside_map.push(pipe.to_char());
                    state = state.flip().unwrap();
                    history = WallHistory::None;
                }
                (Pipe::BendNE | Pipe::BendNW, WallHistory::North) => {
                    // Wall is ending, but not on a switch
                    outside_map.push(pipe.to_char());
                    history = WallHistory::None;
                }

                (Pipe::BendSE | Pipe::BendSW, WallHistory::None) => {
                    // Entering a wall
                    history = WallHistory::South;
                    outside_map.push(pipe.to_char());
                }
                (Pipe::BendSE | Pipe::BendSW, WallHistory::North) => {
                    // Wall is ending on a switch
                    outside_map.push(pipe.to_char());
                    state = state.flip().unwrap();
                    history = WallHistory::None;
                }
                (Pipe::BendSE | Pipe::BendSW, WallHistory::South) => {
                    // Wall is ending, but not on a switch
                    outside_map.push(pipe.to_char());
                    history = WallHistory::None;
                }
                (Pipe::Horizontal, _) => outside_map.push(pipe.to_char()),
                (Pipe::Ground, _) => outside_map.push(state.to_char()),
                (Pipe::Start, _) => {
                    panic!("Start pipe found in filtered map")
                }
            }
        }
    }

    for line in outside_map.chunks(filtered_map.width) {
        println!("{}", line.iter().join(""));
    }

    outside_map
        .iter()
        .filter(|c| **c == State::Inside.to_char())
        .count()
}

/// The furthest point is simply the ceiling of half the number of steps
fn get_farthest_distance(steps: u32) -> u32 {
    (steps + 1) / 2
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn compliment(&self) -> Self {
        use Direction::{East, North, South, West};

        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Coord {
    fn north(&self) -> Option<Self> {
        if self.y == 0 {
            return None;
        };

        Some(Coord {
            x: self.x,
            y: self.y - 1,
            width: self.width,
            height: self.height,
        })
    }

    fn south(&self) -> Option<Self> {
        if self.y + 1 >= self.height {
            return None;
        };

        Some(Coord {
            x: self.x,
            y: self.y + 1,
            width: self.width,
            height: self.height,
        })
    }

    fn west(&self) -> Option<Self> {
        if self.x == 0 {
            return None;
        };

        Some(Coord {
            x: self.x - 1,
            y: self.y,
            width: self.width,
            height: self.height,
        })
    }

    fn east(&self) -> Option<Self> {
        if self.x + 1 >= self.width {
            return None;
        };

        Some(Coord {
            x: self.x + 1,
            y: self.y,
            width: self.width,
            height: self.height,
        })
    }

    fn linear_index(&self) -> usize {
        self.y * self.width + self.x
    }

    fn step(&self, direction: &Direction) -> Option<Self> {
        match direction {
            Direction::North => self.north(),
            Direction::East => self.east(),
            Direction::West => self.west(),
            Direction::South => self.south(),
        }
    }
}

struct Map {
    pipes: Vec<Pipe>,
    height: usize,
    width: usize,
    start: Coord,
}

impl Map {
    fn empty_like(&self) -> Self {
        let mut pipes = vec![Pipe::Ground; self.pipes.len()];
        pipes[self.start.linear_index()] = Pipe::Start;

        Map {
            pipes,
            height: self.height,
            width: self.width,
            start: self.start,
        }
    }

    fn set(&mut self, coord: &Coord, pipe: Pipe) {
        self.pipes[coord.linear_index()] = pipe;
    }
    fn get(&self, coord: &Coord) -> Pipe {
        self.pipes[coord.linear_index()]
    }

    fn parse_input(input: &str) -> Map {
        let pipes: Vec<_> = input
            .lines()
            .flat_map(|line| line.chars().map(Pipe::from_char))
            .collect();
        let n_pipes = pipes.len();

        // This is a bit of an awkward way of getting the number of chars in the array
        let width = input
            .lines()
            .next()
            .unwrap()
            .char_indices()
            .map(|(i, _)| i)
            .last()
            .unwrap()
            + 1;
        let height = n_pipes / width;

        let start = pipes
            .iter()
            .enumerate()
            .find_map(|(n, p)| {
                if *p == Pipe::Start {
                    let y = n / width;
                    let x = n % width;
                    Some(Coord {
                        x,
                        y,
                        width,
                        height,
                    })
                } else {
                    None
                }
            })
            .expect("No start tile found");

        Map {
            pipes,
            height,
            width,
            start,
        }
    }

    fn get_first_step(&self) -> (Direction, Coord, Pipe) {
        let directions = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];

        let coord = self.start;
        let (direction, new_coord, new_pipe) = directions
            .iter()
            .find_map(|d| {
                let coord_step = coord.step(d)?;
                let step_pipe = self.get(&coord_step);

                let d_compliment = d.compliment();
                if step_pipe.contains(&d_compliment) {
                    Some((d, coord_step, step_pipe))
                } else {
                    None
                }
            })
            .expect("Unable to find first step");

        (*direction, new_coord, new_pipe)
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.pipes.chunks(self.width as usize) {
            writeln!(
                f,
                "{}",
                line.iter().map(|p| p.to_string()).collect::<String>()
            )?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Pipe {
    Vertical,
    Horizontal,
    BendNE,
    BendNW,
    BendSW,
    BendSE,
    Ground,
    Start,
}

impl Pipe {
    fn from_char(input: char) -> Pipe {
        match input {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::BendNE,
            'J' => Pipe::BendNW,
            '7' => Pipe::BendSW,
            'F' => Pipe::BendSE,
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
            _ => panic!("Unexpected character for pipe."),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Pipe::Vertical => '│',
            Pipe::Horizontal => '─',
            Pipe::BendNE => '└',
            Pipe::BendNW => '┘',
            Pipe::BendSW => '┐',
            Pipe::BendSE => '┌',
            Pipe::Ground => '.',
            Pipe::Start => 'S',
        }
    }

    fn directions(&self) -> Option<(Direction, Direction)> {
        use Direction::{East, North, South, West};

        match self {
            Pipe::Ground => None,
            Pipe::Start => None,
            Pipe::Vertical => Some((North, South)),
            Pipe::Horizontal => Some((East, West)),
            Pipe::BendNE => Some((North, East)),
            Pipe::BendNW => Some((North, West)),
            Pipe::BendSE => Some((South, East)),
            Pipe::BendSW => Some((South, West)),
        }
    }

    /// Given a direction, return the other direction of the pipe
    fn exit(&self, direction: &Direction) -> Option<Direction> {
        let direction_pairs = self.directions()?;

        if direction_pairs.0 == *direction {
            Some(direction_pairs.1)
        } else if direction_pairs.1 == *direction {
            Some(direction_pairs.0)
        } else {
            panic!("Given start direction is invalid.")
        }
    }

    /// Can this pipe be entered from the given direction
    fn contains(&self, direction: &Direction) -> bool {
        if let Some(direction_pairs) = self.directions() {
            direction_pairs.0 == *direction || direction_pairs.1 == *direction
        } else {
            false
        }
    }

    fn from_directions(dir_one: &Direction, dir_two: &Direction) -> Pipe {
        // Sorting reduces the number of cases to check
        let d_one = max(dir_one, dir_two);
        let d_two = min(dir_one, dir_two);

        match (d_two, d_one) {
            (Direction::North, Direction::East) => Pipe::BendNE,
            (Direction::North, Direction::West) => Pipe::BendNW,
            (Direction::East, Direction::South) => Pipe::BendSE,
            (Direction::South, Direction::West) => Pipe::BendSW,
            (Direction::North, Direction::South) => Pipe::Vertical,
            (Direction::East, Direction::West) => Pipe::Horizontal,
            _ => panic!("Invalid directions for pipe"),
        }
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pipe::Vertical => write!(f, "│"),
            Pipe::Horizontal => write!(f, "─"),
            Pipe::BendNE => write!(f, "└"),
            Pipe::BendNW => write!(f, "┘"),
            Pipe::BendSW => write!(f, "┐"),
            Pipe::BendSE => write!(f, "┌"),
            Pipe::Ground => write!(f, "·"),
            Pipe::Start => write!(f, "S"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_map() {
        let input = indoc! {"
            .....
            .....
            .S-7.
            .|.|.
            .L-J.
            .....
        "};

        let map_actual = Map::parse_input(input);
        assert_eq!(map_actual.width, 5);
        assert_eq!(map_actual.height, 6);

        let start_expected = Coord {
            x: 1,
            y: 2,
            height: 6,
            width: 5,
        };
        assert_eq!(start_expected, map_actual.start);

        let map_string = map_actual.to_string();
        let map_string_expected = indoc! {"
             ·····
             ·····
             ·S─┐·
             ·│·│·
             ·└─┘·
             ·····
            "};

        assert_eq!(map_string_expected, map_string);
    }

    #[test]
    fn test_coords() {
        let input = indoc! {"
            .....
            .....
            .S-7.
            .|.|.
            .L-J.
            .....
        "};

        let map = Map::parse_input(input);

        let coord = map.start;

        let north_coord = coord.north().unwrap();
        let north_pipe = map.get(&north_coord);

        assert_eq!(north_pipe, Pipe::Ground);

        let south_coord = coord.south().unwrap();
        let south_pipe = map.get(&south_coord);

        assert_eq!(south_pipe, Pipe::Vertical);

        let ww_coord = coord.west().unwrap().west();

        assert!(ww_coord.is_none())
    }

    #[test]
    fn test_first_steps() {
        let input = indoc! {"
            .....
            .....
            .S-7.
            .|.|.
            .L-J.
            .....
        "};

        let map = Map::parse_input(input);
        let start_coord = map.start;

        let (direction, new_coord, new_pipe) = map.get_first_step();

        assert_eq!(direction, Direction::East);
        assert_eq!(new_coord, start_coord.east().unwrap());
        assert_eq!(new_pipe, Pipe::Horizontal);
    }

    #[test]
    fn test_walk_map() {
        let input = indoc! {"
            .....
            .....
            .S-7.
            .|.|.
            .L-J.
            .....
        "};

        let steps = walk_map(input).0;
        assert_eq!(steps, 4);
    }

    #[test]
    fn test_walk_map_complex() {
        let input = indoc! {"
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...
        "};
        let (steps, filtered_map) = walk_map(input);
        assert_eq!(steps, 8);

        let filtered_map_string = filtered_map.to_string();
        println!("{}", filtered_map_string);

        let filtered_map_string_expected = indoc! {"
             ··┌┐·
             ·┌┘│·
             ┌┘·└┐
             │┌──┘
             └┘···
        "};
        assert_eq!(filtered_map_string_expected, filtered_map_string)
    }

    #[test]
    fn test_is_inside() {
        let input = indoc! {"
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...
        "};
        let (_, filtered_map) = walk_map(input);

        let filtered_map_string = filtered_map.to_string();
        println!("{}", filtered_map_string);
        let inside_count = map_inside(&filtered_map);

        assert_eq!(inside_count, 1)
    }

    #[test]
    fn test_pipe_from_directions() {
        use Direction::{East, North, South, West};
        assert_eq!(Pipe::from_directions(&North, &East), Pipe::BendNE);
        assert_eq!(Pipe::from_directions(&North, &West), Pipe::BendNW);
        assert_eq!(Pipe::from_directions(&South, &East), Pipe::BendSE);
        assert_eq!(Pipe::from_directions(&South, &West), Pipe::BendSW);
        assert_eq!(Pipe::from_directions(&North, &South), Pipe::Vertical);
        assert_eq!(Pipe::from_directions(&East, &West), Pipe::Horizontal);
    }

    #[test]
    fn test_is_inside_nightmare() {
        let input = indoc! {"
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
        "};
        let (_, filtered_map) = walk_map(input);

        println!("{}", filtered_map.to_string());

        let inside_count = map_inside(&filtered_map);

        assert_eq!(inside_count, 10);
    }

    #[test]
    fn test_squeeze() {
        let input = indoc! {"
            ..........
            .S------7.
            .|F----7|.
            .||....||.
            .||....||.
            .|L-7F-J|.
            .|..||..|.
            .L--JL--J.
            ..........
        "};

        let (_, mut filtered_map) = walk_map(input);
        filtered_map.pipes[filtered_map.start.linear_index()] = Pipe::BendSE;

        let inside_count = map_inside(&filtered_map);
        assert_eq!(inside_count, 4)
    }
}
