use crate::{Direction, Instruction};

pub fn parse_sparse_list(input: &str) -> Result<Vec<Instruction>, String> {
    let mut instructions = Vec::new();

    for line in input.lines() {
        let instruction = parse_instruction(line)?;
        instructions.push(instruction);
    }

    Ok(instructions)
}

fn parse_instruction(input: &str) -> Result<Instruction, String> {
    // Remove everything until the the first '#'
    let input = input
        .split('#')
        .nth(1)
        .ok_or_else(|| format!("Unable to find # in {}", input))?;

    // The first five digits are the hex distance
    let distance = u32::from_str_radix(&input[..5], 16).map_err(|e| format!("{}", e))?;

    // The next digit is the direction
    let direction = match input.chars().nth(5).ok_or("No fifth character")? {
        '0' => Direction::Right,
        '1' => Direction::Down,
        '2' => Direction::Left,
        '3' => Direction::Up,
        _ => panic!("Invalid direction"),
    };

    let instruction = Instruction::new(direction, distance, 0);
    Ok(instruction)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        let input = "R 6 (#70c710)";
        let expected = Instruction::new(Direction::Right, 461937, 0);

        assert_eq!(parse_instruction(input), Ok(expected));
    }

    #[test]
    fn test_parse_list() {
        let input = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)";
        let expected = vec![
            Instruction::new(Direction::Right, 461937, 0),
            Instruction::new(Direction::Down, 56407, 0),
            Instruction::new(Direction::Right, 356671, 0),
        ];
        assert_eq!(parse_sparse_list(input), Ok(expected));
    }
}
