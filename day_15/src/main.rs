mod problem_one;
mod problem_two;

fn main() {
    let problem_one_answer = problem_one::problem_one();
    println!("Problem One Answer: {}", problem_one_answer);

    let expected = 517315;
    if problem_one_answer != expected {
        println!(
            "Problem One Answer is incorrect! Expected {}, got {}",
            expected, problem_one_answer
        );
    }

    let problem_two_answer = problem_two::problem_two();
    println!("Problem Two Answer: {}", problem_two_answer);

    let expected = 247763;
    if problem_two_answer != expected {
        println!(
            "Problem Two Answer is incorrect! Expected {}, got {}",
            expected, problem_two_answer
        );
    }
}
