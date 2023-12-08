fn main() {
    let input = include_str!("problem_text_one");
    let solution_one = get_winning_product(input);
    println!("Solution One: {}", solution_one);

    let input = include_str!("problem_text_two");
    let solution_two = get_winning_product(input);
    println!("Solution Two: {}", solution_two);
}

#[derive(Debug, PartialEq)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    // We could reduce this by only returning the length of the winning times
    // but this is more useful for debugging
    fn get_winning_times(&self) -> Vec<u64> {
        (1..self.time)
            .filter(|hold_time| get_distance_travelled(*hold_time, self.time) > self.distance)
            .collect()
    }
}

fn get_winning_product(input: &str) -> u64 {
    let races = parse_input(input);

    races.into_iter().map(|r| r.get_winning_times().len() as u64).product()
}

fn get_distance_travelled(hold_time: u64, total_time: u64) -> u64 {
    assert!(hold_time <= total_time);
    let acceleration = 1;
    acceleration * hold_time * (total_time - hold_time)
}

fn parse_input(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times = lines
        .next()
        .expect("No first line")
        .split(' ')
        .skip(1)
        .filter_map(|v| v.parse::<u64>().ok());

    let distances = lines
        .next()
        .expect("No second line")
        .split(' ')
        .skip(1)
        .filter_map(|v| v.parse::<u64>().ok());

    times
        .zip(distances)
        .map(|(time, distance)| Race::new(time, distance))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_sample_input() {
        let test_input = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};
        let expected = vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)];

        let actual = parse_input(test_input);

        assert_eq!(actual, expected)
    }

    #[test]
    fn get_race_times() {
        let race = Race::new(7, 9);
        let expected_winners = [2, 3, 4, 5];

        let actual_winners = race.get_winning_times();
        assert_eq!(actual_winners, expected_winners)
    }

    #[test]
    fn test_sample_one() {
        let test_input = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};

        let expected = 288;

        let actual = get_winning_product(test_input);
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_sample_two() {
        let test_input = indoc! {"
            Time:      71530
            Distance:  940200
        "};

        let expected = 71503;

        let actual = get_winning_product(test_input);
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_problem_two() {
        let test_input = include_str!("problem_text_two");
        let mut races = parse_input(test_input);

        assert_eq!(races.len(), 1);

        let race_actual = races.pop().unwrap();
        let race_expected = Race::new(47_70_75_66, 282_1079_1147_1062);

        assert_eq!(race_actual, race_expected);
    }
}
