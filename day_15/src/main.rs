fn main() {
    let problem_one_answer = problem_one();
    println!("Problem One Answer: {}", problem_one_answer);

    let expected = 517315;
    if problem_one_answer != expected {
        println!("Problem One Answer is incorrect! Expected {}, got {}", expected, problem_one_answer);
    }
}

fn problem_one() -> u64 {
    let input = include_str!("problem_text");
    get_commands_sum(input)
}

fn get_commands_sum(input: &str) -> u64 {
    let commands = parse_input(input);
    commands.iter().map(|c| c.execute()).sum()
}


#[derive(Debug, PartialEq)]
struct Command(String);

impl Command {
    fn new(s: &str) -> Command {
        Command(s.to_string())
    }

    fn execute(&self) -> u64 {
        self.0.chars().fold(0, |acc, c| score_char(c, acc))
    }
}

fn score_char(c: char, init: u64) -> u64 {
    ((init + c as u64) * 17) % 256
}

fn parse_input(input: &str) -> Vec<Command> {
    // We note the ``trim`` command to remove the trailing newline that would
    // otherwise be parsed as a command.
    input.trim().split(",").map(|s| Command(s.to_string())).collect()
}


#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_simple_string() {
        let command = Command::new("HASH");
        assert_eq!(command.execute(), 52);
    }

    #[test_case("rn=1", 30)]
    #[test_case("cm-", 253)]
    #[test_case("pc=4", 180)]
    fn test_strings(input: &str, expected: u64) {
        let command = Command::new(input);
        assert_eq!(command.execute(), expected);
    }

    #[test]
    fn test_command_parse() {
        let input = "rn=1,cm-";
        let commands = parse_input(input);

        let commands_expected = vec![
            Command::new("rn=1"),
            Command::new("cm-"),
        ];

        assert_eq!(commands, commands_expected);
    }

    #[test]
    fn sample_one() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(get_commands_sum(input), 1320);
    }
}