#![allow(dead_code)]
use std::fmt::Display;

fn main() {
    problem_one()
}

fn problem_one() {
    let input = include_str!("problem_text");
    let steps = walk_map(input);

    println!("Problem one: {steps}");
}

fn walk_map(input: &str) -> u32 {
    let map = Map::parse_input(input);
    let mut steps = 1;

    let (mut new_direction, mut new_coord, mut new_pipe) = map.get_first_step();

    loop {
        new_direction = new_pipe.exit(&new_direction.compliment()).expect("Unable to exit pipe");
        new_coord = new_coord.step(&new_direction).expect("Attempting to step off map");
        new_pipe = map.get(&new_coord);
        steps += 1;

        if new_pipe == Pipe::Start {
            break
        }
    }

    return get_farthest_distance(steps)
}

/// The furthest point is simply the ceiling of half the number of steps
fn get_farthest_distance(steps: u32) -> u32 {
    (steps + 1) / 2
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn compliment(&self) -> Self {
        use Direction::{North, East, South, West};

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
            return None
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
            return None
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
            return None
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
            return None
        };

        Some(Coord {
            x: self.x + 1,
            y: self.y,
            width: self.width,
            height: self.height,
        })
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
    fn get(&self, coord: &Coord) -> Pipe {
        let linear_index = coord.y * self.width + coord.x;
        self.pipes[linear_index]
    }

    fn parse_input(input: &str) -> Map {
        let pipes: Vec<_> = input.lines().flat_map(|line| line.chars().map(Pipe::from_char)).collect();
        let n_pipes = pipes.len();

        // This is a bit of an awkward way of getting the number of chars in the array
        let width = input.lines().next().unwrap().char_indices().map(|(i, _)| i).last().unwrap() + 1;
        let height = n_pipes / width;

        let start = pipes.iter().enumerate().find_map(|(n, p)| {
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
        }).expect("No start tile found");

        Map {
            pipes,
            height,
            width,
            start
        }
    }

    fn get_first_step(&self) -> (Direction, Coord, Pipe) {
        let directions = [Direction::North, Direction::East, Direction::South, Direction::West];

        let coord = self.start;
        let (direction, new_coord, new_pipe) = directions.iter().find_map(|d| {
            let coord_step = coord.step(d)?;
            let step_pipe = self.get(&coord_step);

            let d_compliment = d.compliment();
            if step_pipe.contains(&d_compliment) {
                Some((d, coord_step, step_pipe))
            } else {
                None
            }
        }).expect("Unable to find first step");

        (*direction, new_coord, new_pipe)
        }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.pipes.chunks(self.width as usize) {
            writeln!(f, "{}", line.iter().map(|p| p.to_string()).collect::<String>())?;
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
            _ => panic!("Unexpected character for pipe.")
        }
    }

    fn directions(&self) -> Option<(Direction, Direction)> {
        use Direction::{North, South, East, West};

        match self {
            Pipe::Ground     => None,
            Pipe::Start      => None,
            Pipe::Vertical   => Some((North, South)),
            Pipe::Horizontal => Some((East, West)),
            Pipe::BendNE     => Some((North, East)),
            Pipe::BendNW     => Some((North, West)),
            Pipe::BendSE     => Some((South, East)),
            Pipe::BendSW     => Some((South, West)),
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
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pipe::Vertical   => write!(f, "│"),
            Pipe::Horizontal => write!(f, "─"),
            Pipe::BendNE     => write!(f, "└"),
            Pipe::BendNW     => write!(f, "┘"),
            Pipe::BendSW     => write!(f, "┐"),
            Pipe::BendSE     => write!(f, "┌"),
            Pipe::Ground     => write!(f, "·"),
            Pipe::Start      => write!(f, "S"),
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

        let start_expected = Coord{x: 1, y:2, height: 6, width: 5};
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

        let steps = walk_map(input);
        assert_eq!(steps, 4);
    }

    #[test]
    fn test_walk_map_complex() {
        let input = indoc!{"
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...
        "};
        let steps = walk_map(input);
        assert_eq!(steps, 8);
    }
}
