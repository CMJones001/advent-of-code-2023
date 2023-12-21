use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::Display;

fn main() {
    let problem_one_answer = problem_one();
    println!("Problem one answer: {}", problem_one_answer);

    let expected_one = 6994;

    match problem_one_answer.cmp(&expected_one) {
        Ordering::Less => println!("Problem one answer is lower than expected answer: {}", expected_one),
        Ordering::Greater => println!("Problem one answer is higher than expected answer: {}", expected_one),
        _ => {}
    }
}

fn problem_one() -> u64 {
    let input = include_str!("problem_text");
    get_total_energy(input)
}

fn get_total_energy(input: &str) -> u64 {
    let mut map = match parse_map(input) {
        Ok(map) => map,
        Err(e) => panic!("{}", e),
    };

    calculate_laser_path(&mut map);
    map.energized.iter().filter(|&e| *e).count() as u64
}

fn calculate_laser_path(map: &mut Map) {
    let mut active_beams = Vec::new();
    let mut beam_history = HashSet::new();

    let starting_beam = Beam {
        direction: Direction::Right,
        x: 0,
        y: 0,
        height: map.height,
        width: map.width,
    };
    map.energise(&starting_beam);

    // Take the first step
    let mut beam = starting_beam;
    let tile = map.get_tile(&beam);
    reflect_beam(&mut beam, &mut active_beams, &mut beam_history, tile);
    calculate_beam_path(map, &mut beam, &mut active_beams, &mut beam_history);

    // Process the active beams until there are none left
    while let Some(mut next_beam) = active_beams.pop() {
        calculate_beam_path(map, &mut next_beam, &mut active_beams, &mut beam_history);
    }
}

/// Process the beam, adding any new beams caused by splitters to the active beams to be processed later
///
/// Will continue to run until the beam has left the map or has been found in the beam history
fn calculate_beam_path(
    map: &mut Map,
    beam: &mut Beam,
    active_beams: &mut Vec<Beam>,
    beam_history: &mut HashSet<Beam>,
) {
    // Step the beam until it leaves the map (or hits a loop)
    while let Some(beam_) = beam.step() {
        *beam = beam_;

        let new_tile = map.get_tile(beam);
        reflect_beam(beam, active_beams, beam_history, new_tile);
        map.energise(beam);

        // If the beam has already been found in the history, then we have found a loop, so stop processing
        // We note that we only do this check after processing the beam, so that we can process the first beam
        // even if it is a loop
        if beam_history.contains(beam) {
            break;
        } else {
            beam_history.insert(*beam);
        }
    }
}

/// Reflect the beam based on the tile it has hit
///
/// This alters the direction of the beam in place, and adds any new beams caused by splitters to the active beams
/// The new beam is also added to the beam history to avoid infinite loops
fn reflect_beam(
    beam: &mut Beam,
    active_beams: &mut Vec<Beam>,
    beam_history: &mut HashSet<Beam>,
    new_tile: Tile,
) {
    match (new_tile, beam.direction) {
        (Tile::Empty, _) => {}
        (Tile::MirrorUp, Direction::Up) => beam.direction = Direction::Right,
        (Tile::MirrorUp, Direction::Left) => beam.direction = Direction::Down,
        (Tile::MirrorUp, Direction::Down) => beam.direction = Direction::Left,
        (Tile::MirrorUp, Direction::Right) => beam.direction = Direction::Up,
        (Tile::MirrorDown, Direction::Up) => beam.direction = Direction::Left,
        (Tile::MirrorDown, Direction::Left) => beam.direction = Direction::Up,
        (Tile::MirrorDown, Direction::Down) => beam.direction = Direction::Right,
        (Tile::MirrorDown, Direction::Right) => beam.direction = Direction::Down,
        (Tile::SplitterHorizontal, Direction::Up | Direction::Down) => {
            let new_beam = Beam::with_direction(beam, Direction::Right);
            // Only add the beam if it hasn't been found in the history
            if !beam_history.contains(&new_beam) {
                // The new beam is not added to the history, so it will be processed
                active_beams.push(new_beam);
            }
            beam.direction = Direction::Left;
        }
        (Tile::SplitterHorizontal, _) => {}
        (Tile::SplitterVertical, Direction::Left | Direction::Right) => {
            let new_beam = Beam::with_direction(beam, Direction::Down);
            // Only add the beam if it hasn't been found in the history
            if !beam_history.contains(&new_beam) {
                active_beams.push(new_beam);
            }
            beam.direction = Direction::Up;
        }
        (Tile::SplitterVertical, _) => {}
    };
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Beam {
    x: usize,
    y: usize,
    height: usize,
    width: usize,
    direction: Direction,
}

impl Beam {
    fn with_coords(&self, x: usize, y: usize) -> Beam {
        Beam {
            x,
            y,
            height: self.height,
            width: self.width,
            direction: self.direction,
        }
    }

    fn with_direction(&self, direction: Direction) -> Beam {
        Beam {
            x: self.x,
            y: self.y,
            height: self.height,
            width: self.width,
            direction,
        }
    }

    /// Step the beam in the current direction, returning None if it has left the map
    fn step(&self) -> Option<Beam> {
        // Slightly awkward way of doing it, but we need to avoid 0 - 1 underflowing
        match self.direction {
            Direction::Up => {
                if self.y > 0 {
                    Some(self.with_coords(self.x, self.y - 1))
                } else {
                    None
                }
            }
            Direction::Down => {
                if self.y < self.height - 1 {
                    Some(self.with_coords(self.x, self.y + 1))
                } else {
                    None
                }
            }
            Direction::Left => {
                if self.x > 0 {
                    Some(self.with_coords(self.x - 1, self.y))
                } else {
                    None
                }
            }
            Direction::Right => {
                if self.x < self.width - 1 {
                    Some(self.with_coords(self.x + 1, self.y))
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Tile {
    Empty,
    MirrorUp,
    MirrorDown,
    SplitterHorizontal,
    SplitterVertical,
}

impl Tile {
    fn from_char(c: char) -> Result<Tile, String> {
        match c {
            '.' => Ok(Tile::Empty),
            '/' => Ok(Tile::MirrorUp),
            '\\' => Ok(Tile::MirrorDown),
            '-' => Ok(Tile::SplitterHorizontal),
            '|' => Ok(Tile::SplitterVertical),
            _ => Err(format!("Invalid tile character: {}", c)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Indicator{
    Empty,
    Up,
    Down,
    Left,
    Right,
    Count(usize),
}

impl Display for Indicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Indicator::Empty => write!(f, " "),
            Indicator::Up => write!(f, "↑"),
            Indicator::Down => write!(f, "↓"),
            Indicator::Left => write!(f, "←"),
            Indicator::Right => write!(f, "→"),
            Indicator::Count(count) => {
                if *count < 10 {
                    write!(f, "{}", count)
                } else {
                    write!(f, "X")
                }
            },
        }
    }
}


struct Map {
    titles: Vec<Tile>,
    energized: Vec<bool>,
    indicators: Vec<Indicator>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(titles: Vec<Tile>, width: usize, height: usize) -> Map {
        let energized = vec![false; width * height];
        let indicators = vec![Indicator::Empty; width * height];
        Map {
            titles,
            energized,
            indicators,
            width,
            height,
        }
    }

    fn get_tile(&self, beam: &Beam) -> Tile {
        self.titles
            .get(beam.y * self.width + beam.x)
            .copied()
            .expect("Beam out of bounds")
    }

    /// Energise the tile at the given coordinates
    fn energise(&mut self, beam: &Beam) {
        let energized_index = beam.y * self.width + beam.x;
        self.energized[energized_index] = true;

        let indicator = self.indicators.get_mut(energized_index).unwrap();
        match indicator {
            Indicator::Empty => {
                match beam.direction {
                    Direction::Up => {
                        *indicator = Indicator::Up;
                    },
                    Direction::Down => {
                        *indicator = Indicator::Down;
                    },
                    Direction::Left => {
                        *indicator = Indicator::Left;
                    },
                    Direction::Right => {
                        *indicator = Indicator::Right;
                    },
                }
            },
            Indicator::Count(count) => {
                *count += 1;
                *indicator = Indicator::Count(*count);
            },
            _ => *indicator = Indicator::Count(2),
        }

    }

    fn pretty_string(&self) -> String {
        let mut output = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let energized_index = y * self.width + x;
                output.push_str(&format!("{}", self.indicators[energized_index]));
            }
            output.push('\n');
        }
        output
    }
}

fn parse_map(input: &str) -> Result<Map, String> {
    let mut titles = Vec::new();
    let mut width = None;
    let mut height = 0;

    for line in input.trim().lines() {
        if let Some(w) = width {
            if line.len() != w {
                return Err(format!("Line length mismatch: {} vs {}", line.len(), w));
            }
        }
        width = Some(line.len());

        height += 1;
        for c in line.chars() {
            titles.push(Tile::from_char(c)?);
        }
    }

    Ok(Map::new(titles, width.unwrap(), height))
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    impl Map {
        fn energised_string(&self) -> String {
            let mut output = String::new();
            for y in 0..self.height {
                for x in 0..self.width {
                    let energized_index = y * self.width + x;
                    if self.energized[energized_index] {
                        output.push('X');
                    } else {
                        output.push('.');
                    }
                }
                output.push('\n');
            }
            output
        }
    }

    fn get_input() -> &'static str {
        indoc! {r#"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....
        "#}
    }

    #[test]
    fn test_simple_mirror() {
        let test_input = indoc! {r#"
            ..-.\
            .....
            ....|
            .....
            ....\
        "#};
        let mut map = match parse_map(test_input) {
            Ok(map) => map,
            Err(e) => panic!("{}", e),
        };

        calculate_laser_path(&mut map);

        let energized_beam = map.energised_string();
        let expected_output = indoc! {r#"
            XXXXX
            ....X
            ....X
            ....X
            ....X
        "#};

        assert_eq!(energized_beam, expected_output);
    }

    #[test]
    fn test_simple_mirror_loop() {
        let test_input = indoc! {r#"
            .\-/.\
            .\././
            ......
            ......
            ...\..
        "#};
        let mut map = match parse_map(test_input) {
            Ok(map) => map,
            Err(e) => panic!("{}", e),
        };

        calculate_laser_path(&mut map);

        let energized_beam = map.energised_string();
        let expected_output = indoc! {r#"
            XX.XXX
            .XXXXX
            ...X..
            ...X..
            ...XXX
        "#};

        assert_eq!(energized_beam, expected_output);
    }

    #[test]
    fn test_trivial_case() {
        let test_input = ".....";

        let mut map = match parse_map(test_input) {
            Ok(map) => map,
            Err(e) => panic!("{}", e),
        };

        calculate_laser_path(&mut map);

        let energized_beam = map.energised_string();
        let expected_output = "XXXXX\n";

        assert_eq!(energized_beam, expected_output);
    }

    #[test]
    fn test_energised_path() {
        let test_input = get_input();
        let mut map = match parse_map(test_input) {
            Ok(map) => map,
            Err(e) => panic!("{}", e),
        };

        calculate_laser_path(&mut map);

        let expected_output = indoc! {r#"
            XXXXXX....
            .X...X....
            .X...XXXXX
            .X...XX...
            .X...XX...
            .X...XX...
            .X..XXXX..
            XXXXXXXX..
            .XXXXXXX..
            .X...X.X..
        "#};

        println!("{}", map.energised_string());
        assert_eq!(map.energised_string(), expected_output);
    }

    #[test]
    fn test_starting_tile() {
        let test_input = indoc! {r#"
            \./.\
            .....
            ..-..
            .....
            \.../
        "#};
        let mut map = match parse_map(test_input) {
            Ok(map) => map,
            Err(e) => panic!("{}", e),
        };

        calculate_laser_path(&mut map);

        let energized_beam = map.energised_string();
        let expected_output = indoc! {r#"
            X.XXX
            X.X.X
            XXXXX
            X...X
            XXXXX
        "#};

        assert_eq!(energized_beam, expected_output);

        let pretty_string_expected = indoc! {r#"
             → ↓←←
             ↓ ↓ ↑
             2←←→2
             ↓   ↑
             →→→→↑
        "#};
        assert_eq!(map.pretty_string(), pretty_string_expected);
    }

    #[test]
    fn test_edge_splitter() {
        let test_input = indoc! {r#"
            ...\.
            .....
            /..-|
            .....
            \...\
        "#};
        let mut map = match parse_map(test_input) {
            Ok(map) => map,
            Err(e) => panic!("{}", e),
        };

        calculate_laser_path(&mut map);

        let pretty_string_expected = indoc! {r#"
              →→→↓↑
                 ↓↑
              ↓←←←↑
              ↓   ↓
              →→→→2
        "#};
        println!("{}", map.pretty_string());
        assert_eq!(map.pretty_string(), pretty_string_expected);
    }


    #[test]
    fn test_sample_one() {
        let input = get_input();
        let total_energy = get_total_energy(input);

        assert_eq!(total_energy, 46);
    }
}
