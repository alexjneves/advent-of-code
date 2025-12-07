use crate::day::{read_day_input, Day, InputType, Part};

const YEAR_ID: u8 = 25;
const DAY_ID: u8 = 6;

pub struct Day6 {}

#[derive(Debug)]
struct MathProblem {
    numbers: Vec<i64>,
    operator: char
}

impl Day for Day6 {
    fn run(&self, part: Part, input_type: InputType) -> i64 {
        let input = read_day_input(YEAR_ID, DAY_ID, &part, &input_type);

        let math_problems = match part {
            Part::One => parse_math_problems(&input),
            Part::Two => parse_cephalopod_math_problem(&input),
        };

        math_problems.iter()
            .map(solve_math_problem)
            .sum()
    }
}

fn parse_math_problems(input: &Vec<String>) -> Vec<MathProblem> {
    let mut problems: Vec<MathProblem> = vec![];

    let (operators_row, number_rows) = input.split_last().unwrap();

    for number_row in number_rows.iter() {
        let numbers: Vec<i64> = number_row
            .split_whitespace()
            .map(|l| l.parse::<i64>().unwrap())
            .collect();

        for (i, number) in numbers.iter().enumerate() {
            if problems.get(i).is_none() {
                problems.push(MathProblem { numbers: vec![], operator: '.' });
            }

            problems[i].numbers.push(*number);
        }
    }

    let operators = operators_row
        .split_whitespace()
        .map(|l| l.chars().nth(0).unwrap());

    for (i, operator) in operators.enumerate() {
        problems[i].operator = operator;
    }

    problems
}

fn parse_cephalopod_math_problem(input: &Vec<String>) -> Vec<MathProblem> {
    let mut problems: Vec<MathProblem> = vec![];

    let (operators_row, number_rows) = input.split_last().unwrap();

    let mut columns_as_chars: Vec<Vec<char>> = vec![];
    for number_row in number_rows {
        for (i, c) in number_row.chars().enumerate() {
            if columns_as_chars.get(i).is_none() {
                columns_as_chars.push(vec![]);
            }

            columns_as_chars[i].push(c);
        }
    }

    let columns_as_strings= columns_as_chars.iter()
        .map(|c| 
            c.iter().collect::<String>()
            .trim()
            .to_owned());

    let mut column_index = 0;
    for string_column in columns_as_strings {
        if !string_column.is_empty() {
            let number = string_column.parse::<i64>().unwrap();

            if problems.get(column_index).is_none() {
                problems.push(MathProblem { numbers: vec![], operator: '.' });
            }

            problems[column_index].numbers.push(number);
        } else {
            column_index += 1;
        }
    }

    let operators = operators_row
        .split_whitespace()
        .map(|l| l.chars().nth(0).unwrap());

    for (i, operator) in operators.enumerate() {
        problems[i].operator = operator;
    }

    problems
}

fn solve_math_problem(problem: &MathProblem) -> i64 {
    let init = match problem.operator {
        '*' => 1,
        '+' => 0,
        _ => panic!("Unsupported operator")
    };

    problem.numbers.iter().fold(init, |acc, n| apply_operator(acc, *n, problem.operator))
}

fn apply_operator(x: i64, y: i64, operator: char) -> i64 {
    match operator {
        '*' => x * y,
        '+' => x + y,
        _ => panic!("Unsupported operator")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6_part1_example_input() {
        const EXPECTED_ANSWER: i64 = 4277556;

        let day6 = Day6 {};
        let answer = day6.run(Part::One, InputType::Example);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day6_part1_custom_input() {
        const EXPECTED_ANSWER: i64 = 3785892992137;

        let day6 = Day6 {};
        let answer = day6.run(Part::One,InputType::Custom);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day6_part2_example_input() {
        const EXPECTED_ANSWER: i64 = 3263827;

        let day6 = Day6 {};
        let answer = day6.run(Part::Two, InputType::Example);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day6_part2_custom_input() {
        const EXPECTED_ANSWER: i64 = 7669802156452;

        let day6 = Day6 {};
        let answer = day6.run(Part::Two,InputType::Custom);
        
        assert_eq!(answer, EXPECTED_ANSWER);
    }
}