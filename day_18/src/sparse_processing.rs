use crate::sparse_parser::parse_sparse_list;
use crate::{Direction, Instruction};
use rayon::prelude::*;

pub fn problem_two() -> u64 {
    let parallel = true;
    let input = include_str!("problem_text");
    let instructions = parse_sparse_list(input).unwrap();
    if parallel {
        get_total_inside_count_par(instructions)
    } else {
        get_total_inside_count(instructions)
    }
}

fn collect_intersections_on_row(walls: &[Wall], y: i32) -> Vec<&Wall> {
    walls.iter().filter(|wall| wall.is_on_row(y)).collect()
}

/// Count the number of pixels on the boundary or inside the path.
///
/// This is the somewhat naive approach of scanning every line in the image,
/// we might be able to cache most of the results, but lets start with this.
///
/// We note that this needs to a contiguous path, we need the end point to be
/// the same as the start point.
fn get_total_inside_count(instructions: Vec<Instruction>) -> u64 {
    let walls = instructions_to_walls(&instructions);
    assert!(is_wall_loop(&walls));

    let min_x = walls.iter().map(|wall| wall.min_x()).min().unwrap();

    let max_y = walls.iter().map(|wall| wall.max_y()).max().unwrap();
    let min_y = walls.iter().map(|wall| wall.min_y()).min().unwrap();

    let mut count = 0;
    let mut last_intersection = None;

    for y in min_y..=max_y {
        let mut intersections = collect_intersections_on_row(&walls, y);

        // Cache the last intersection, so we can reuse the score if the intersections are the same
        if let Some((ref prev_wall, prev_score)) = last_intersection {
            if &intersections == prev_wall {
                count += prev_score as u64;
                continue;
            }
        }
        intersections.sort_by_key(|wall| wall.mid_x());

        let row_count = get_row_count(min_x, &mut intersections);
        count += row_count as u64;
        last_intersection = Some((intersections.clone(), row_count));
    }

    count
}

/// A parallel version of get_total_inside_count.
fn get_total_inside_count_par(instructions: Vec<Instruction>) -> u64 {
    let walls = instructions_to_walls(&instructions);
    assert!(is_wall_loop(&walls));

    let min_x = walls.iter().map(|wall| wall.min_x()).min().unwrap();

    let max_y = walls.iter().map(|wall| wall.max_y()).max().unwrap();
    let min_y = walls.iter().map(|wall| wall.min_y()).min().unwrap();

    let count = (min_y..=max_y)
        .into_par_iter()
        .fold(
            || (0, None),
            |(score, last_intersection), y| {
                let mut intersections = collect_intersections_on_row(&walls, y);

                // Cache the last intersection, so we can reuse the score if the intersections are the same
                if let Some((ref prev_wall, prev_score)) = last_intersection {
                    if &intersections == prev_wall {
                        return (score + prev_score as u64, last_intersection);
                    }
                }
                intersections.sort_by_key(|wall| wall.mid_x());

                let row_count = get_row_count(min_x, &mut intersections);
                let last_intersection = Some((intersections.clone(), row_count));
                (score + row_count as u64, last_intersection)
            },
        )
        // Fold leaves us with another iterator (due to the parallel nature), so we need to
        // sum the results.
        .map(|(a, _)| a)
        .sum::<u64>();

    count
}

fn get_row_count(start_x: i32, intersections: &mut Vec<&Wall>) -> i32 {
    let mut inside = false;
    let mut row_count = 0;
    let mut x = start_x;

    for wall in intersections {
        // Skip to the start of the wall, unless we're already inside
        let distance = wall.min_x() - x;
        if distance > 0 {
            if inside {
                row_count += distance;
            }
            x += distance;
        }

        match wall {
            Wall::Vertical { .. } => {
                // +1 for the wall itself, which is always inside
                x += 1;
                row_count += 1;
                inside = !inside;
            }
            Wall::Horizontal { polarity, .. } => {
                // Advance to the end of the wall
                // Neither end of the wall is counted here, it is instead dealt with by the
                // vertical wall segments.
                let remaining_length = wall.max_x() - x;
                row_count += remaining_length;
                x += remaining_length;

                if polarity == &Polarity::Line {
                    inside = !inside;
                }
            }
        }
    }
    // We could account for the remaining distance here, but it's not necessary, anything
    // remaining is outside the path.
    row_count
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Wall {
    Vertical {
        x: i32,
        y1: i32,
        y2: i32,
    },
    Horizontal {
        y: i32,
        x1: i32,
        x2: i32,
        polarity: Polarity,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Polarity {
    Line,
    Loop,
}

/// Return true if the final point is the same as the start point (0, 0).
fn is_wall_loop(wall: &[Wall]) -> bool {
    let first_point = wall.first().unwrap();

    let (&x_start, &y_start) = match first_point {
        Wall::Vertical { y1: y, x, .. } => (x, y),
        Wall::Horizontal { y, x1: x, .. } => (x, y),
    };

    let final_wall = wall.last().unwrap();

    let (&x_final, &y_final) = match final_wall {
        Wall::Vertical { y2: y, x, .. } => (x, y),
        Wall::Horizontal { x2: x, y, .. } => (x, y),
    };

    x_final == x_start && y_final == y_start
}

impl Wall {
    fn new_vertical(x: i32, y1: i32, y2: i32) -> Wall {
        Wall::Vertical { x, y1, y2 }
    }

    fn new_horizontal(y: i32, x1: i32, x2: i32, polarity: Polarity) -> Wall {
        Wall::Horizontal {
            y,
            x1,
            x2,
            polarity,
        }
    }

    // I have a suspicion that this is the choke point, as we're doing a lot of
    // comparisons here, but profiling is required.
    fn is_on_row(&self, y: i32) -> bool {
        match self {
            Wall::Vertical { y1, y2, .. } => {
                let min_y = *y1.min(y2);
                let max_y = *y1.max(y2);
                y >= min_y && y <= max_y
            }
            Wall::Horizontal { y: y1, .. } => y == *y1,
        }
    }

    fn max_y(&self) -> i32 {
        match self {
            Wall::Vertical { y1, y2, .. } => *y2.max(y1),
            Wall::Horizontal { y, .. } => *y,
        }
    }

    fn min_y(&self) -> i32 {
        match self {
            Wall::Vertical { y1, y2, .. } => *y2.min(y1),
            Wall::Horizontal { y, .. } => *y,
        }
    }

    fn min_x(&self) -> i32 {
        match self {
            Wall::Vertical { x, .. } => *x,
            Wall::Horizontal { x1, x2, .. } => *x1.min(x2),
        }
    }

    fn mid_x(&self) -> i32 {
        // This is a better way of sorting the walls, as otherwise we have issues with
        // the left corner of horizontal walls and vertical walls.
        match self {
            Wall::Vertical { x, .. } => *x,
            Wall::Horizontal { x1, x2, .. } => (*x1 + *x2) / 2,
        }
    }

    fn max_x(&self) -> i32 {
        match self {
            Wall::Vertical { x, .. } => *x,
            Wall::Horizontal { x1, x2, .. } => *x1.max(x2),
        }
    }
}

fn instructions_to_walls(instructions: &[Instruction]) -> Vec<Wall> {
    let mut walls = Vec::new();
    let mut x = 0;
    let mut y = 0;

    for (num, instruction) in instructions.iter().enumerate() {
        match instruction.direction {
            Direction::Up => {
                let end = y + (instruction.distance as i32);
                let y1 = y;
                let y2 = end;
                walls.push(Wall::new_vertical(x, y1, y2));
                y = end;
            }
            Direction::Down => {
                let end = y - (instruction.distance as i32);
                let y1 = y;
                let y2 = end;
                walls.push(Wall::new_vertical(x, y1, y2));
                y = end;
            }
            Direction::Left => {
                let end = x - (instruction.distance as i32);
                let polarity = get_polarity(instructions, num);
                let x1 = x;
                let x2 = end;
                walls.push(Wall::new_horizontal(y, x1, x2, polarity));
                x = end;
            }
            Direction::Right => {
                let end = x + (instruction.distance as i32);
                let polarity = get_polarity(instructions, num);
                let x1 = x;
                let x2 = end;
                walls.push(Wall::new_horizontal(y, x1, x2, polarity));
                x = end;
            }
        }
    }

    walls
}

fn get_polarity(instructions: &[Instruction], num: usize) -> Polarity {
    let next_index = if num == instructions.len() - 1 {
        0
    } else {
        num + 1
    };
    let prev_index = if num == 0 {
        instructions.len() - 1
    } else {
        num - 1
    };

    let next = instructions.get(next_index).unwrap().direction;
    let prev = instructions.get(prev_index).unwrap().direction;

    if next == Direction::Right
        || prev == Direction::Right
        || next == Direction::Left
        || prev == Direction::Left
    {
        panic!("Cannot have two horizontal walls next to each other");
    }

    if next == prev {
        Polarity::Line
    } else {
        Polarity::Loop
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_is_on_row() {
        let wall = Wall::new_vertical(0, 0, 10);
        assert!(wall.is_on_row(5));
        assert!(!wall.is_on_row(11));
        assert!(!wall.is_on_row(-1));

        let wall = Wall::new_horizontal(0, 0, 10, Polarity::Line);
        assert!(wall.is_on_row(0));
        assert!(!wall.is_on_row(1));
        assert!(!wall.is_on_row(-1));

        let wall = Wall::new_horizontal(0, 0, 10, Polarity::Loop);
        assert!(wall.is_on_row(0));
        assert!(!wall.is_on_row(1));
        assert!(!wall.is_on_row(-1));
    }

    #[test]
    fn test_polarity_simple() {
        /*
        O
        #
        ######
             #
         */
        let instructions = vec![
            Instruction::new(Direction::Down, 6, 0x70c710),
            Instruction::new(Direction::Right, 5, 0x0dc571),
            Instruction::new(Direction::Down, 3, 0x0dc571),
        ];

        let walls = instructions_to_walls(&instructions);

        match walls[1] {
            Wall::Horizontal { polarity, .. } => {
                assert_eq!(polarity, Polarity::Line, "Expected Line")
            }
            _ => panic!("Expected horizontal wall"),
        }

        /*
         ####
         #  #
         ###O
        */

        let instructions = vec![
            Instruction::new(Direction::Up, 6, 0x70c710),
            Instruction::new(Direction::Left, 5, 0x0dc571),
            Instruction::new(Direction::Down, 3, 0x0dc571),
            Instruction::new(Direction::Right, 5, 0x0dc571),
        ];

        let walls = instructions_to_walls(&instructions);

        match walls[1] {
            Wall::Horizontal { polarity, .. } => {
                assert_eq!(polarity, Polarity::Loop, "Expected loop (U to D)")
            }
            _ => panic!("Expected horizontal wall"),
        }

        match walls[3] {
            Wall::Horizontal { polarity, .. } => {
                assert_eq!(polarity, Polarity::Loop, "Expected loop (D to U)")
            }
            _ => panic!("Expected horizontal wall"),
        }
    }

    fn complex_wall() -> Vec<Wall> {
        /*
        ######
        #    #
        #    #
        #    #
        O##  #
          #  #
          ####
        */

        let instructions = vec![
            Instruction::new(Direction::Right, 2, 0x70c710),
            Instruction::new(Direction::Down, 2, 0x0dc571),
            Instruction::new(Direction::Right, 3, 0x0dc571),
            Instruction::new(Direction::Up, 6, 0x0dc571),
            Instruction::new(Direction::Left, 5, 0x0dc571),
            Instruction::new(Direction::Down, 4, 0x0dc571),
        ];

        instructions_to_walls(&instructions)
    }

    #[test]
    fn test_polarity_complex() {
        let walls = complex_wall();

        match walls[0] {
            Wall::Horizontal { polarity, .. } => {
                assert_eq!(polarity, Polarity::Line, "Expected Line, first wall D to D")
            }
            _ => panic!("Expected horizontal wall"),
        }

        match walls[2] {
            Wall::Horizontal { polarity, .. } => {
                assert_eq!(polarity, Polarity::Loop, "Expected loop, D to U")
            }
            _ => panic!("Expected vertical wall"),
        }

        match walls[4] {
            Wall::Horizontal { polarity, .. } => {
                assert_eq!(polarity, Polarity::Loop, "Expected loop, U to D")
            }
            _ => panic!("Expected vertical wall"),
        }
    }

    fn create_instruction_list(instructions: Vec<(Direction, u32)>) -> Vec<Instruction> {
        instructions
            .into_iter()
            .map(|(direction, distance)| Instruction::new(direction, distance, 0))
            .collect()
    }

    #[test_case(-2, 3, 4)]
    #[test_case(-1, 2, 4)]
    #[test_case(0, 4, 6)]
    #[test_case(1, 2, 6)]
    #[test_case(2, 2, 6)]
    #[test_case(3, 2, 6)]
    #[test_case(4, 3, 6)]
    fn test_parse_row(y: i32, n_intersections_expected: usize, row_count_expected: i32) {
        assert!(is_wall_loop(&complex_wall()));
        let walls = complex_wall();
        let min_x = walls.iter().map(|wall| wall.min_x()).min().unwrap();

        let mut intersections = collect_intersections_on_row(&walls, y);
        intersections.sort_by_key(|wall| wall.mid_x());

        assert_eq!(
            intersections.len(),
            n_intersections_expected,
            "Intersection count for y={y}"
        );

        let row_count = get_row_count(min_x, &mut intersections);
        assert_eq!(row_count, row_count_expected, "Row count for y={}", y);
    }

    #[test_case(1, 6)]
    #[test_case(0, 8)]
    #[test_case(-1, 10)]
    #[test_case(-2, 10)]
    #[test_case(-3, 10)]
    fn test_parse_row_lower(y: i32, row_count_expected: i32) {
        /*
          ###  ###
        O## #  # #
        #   #### #
        #        #
        ##########
         */

        let instructions = create_instruction_list(vec![
            (Direction::Right, 2),
            (Direction::Up, 1),
            (Direction::Right, 2),
            (Direction::Down, 2),
            (Direction::Right, 3),
            (Direction::Up, 2),
            (Direction::Right, 2),
            (Direction::Down, 4),
            (Direction::Left, 9),
            (Direction::Up, 3),
        ]);

        let walls = instructions_to_walls(&instructions);
        assert!(is_wall_loop(&walls));

        let min_x = walls.iter().map(|wall| wall.min_x()).min().unwrap();

        let mut intersections = collect_intersections_on_row(&walls, y);
        intersections.sort_by_key(|wall| wall.mid_x());

        let row_count = get_row_count(min_x, &mut intersections);
        assert_eq!(row_count, row_count_expected, "Row count for y={}", y);
    }

    #[test_case(0, 7)]
    #[test_case(1, 9)]
    #[test_case(2, 9)]
    #[test_case(3, 7)]
    fn test_parse_row_upper(y: i32, row_count_expected: i32) {
        /*
           #######
           #IIIII###
           ##I###II#
            O## ####
        */

        let instructions = create_instruction_list(vec![
            (Direction::Right, 2),
            (Direction::Up, 1),
            (Direction::Right, 2),
            (Direction::Down, 1),
            (Direction::Right, 3),
            (Direction::Up, 2),
            (Direction::Left, 2),
            (Direction::Up, 1),
            (Direction::Left, 6),
            (Direction::Down, 2),
            (Direction::Right, 1),
            (Direction::Down, 1),
        ]);

        let walls = instructions_to_walls(&instructions);
        assert!(is_wall_loop(&walls));

        let min_x = walls.iter().map(|wall| wall.min_x()).min().unwrap();

        let mut intersections = collect_intersections_on_row(&walls, y);
        intersections.sort_by_key(|wall| wall.mid_x());

        let row_count = get_row_count(min_x, &mut intersections);
        assert_eq!(row_count, row_count_expected, "Row count for y={}", y);
    }

    #[test]
    fn test_parse_sample_one() {
        // We should be able to recover the results from sample one (without the colour encoding)

        let input = crate::test::test_input();
        let instructions = crate::parse(input).unwrap();
        let total = get_total_inside_count(instructions);

        assert_eq!(total, 62);
    }

    #[test]
    fn test_parse_sample_two() {
        // We should be able to recover the results from sample two (without the colour encoding)

        let input = crate::test::test_input();
        let instructions = parse_sparse_list(input).unwrap();
        let total = get_total_inside_count(instructions);

        assert_eq!(total, 952408144115);
    }

    #[test]
    fn test_parse_sample_two_par() {
        // We should be able to recover the results from sample two (without the colour encoding)

        let input = crate::test::test_input();
        let instructions = parse_sparse_list(input).unwrap();
        let total = get_total_inside_count_par(instructions);

        assert_eq!(total, 952408144115);
    }
}
