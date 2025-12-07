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

        println!("{:?}", math_problems);

        math_problems.iter()
            .map(solve_math_problem)
            .sum()
    }
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

fn parse_math_problems(input: &Vec<String>) -> Vec<MathProblem> {
    let mut problems: Vec<MathProblem> = vec![];

    for (i, line) in input.iter().enumerate() {
        if i == input.len() - 1 {
            // operators
            let operators: Vec<char> = line.split_whitespace().map(|l| l.chars().nth(0).unwrap()).collect();
            for (j, operator) in operators.iter().enumerate() {
                problems[j].operator = *operator;
            }

        } else {
            // numbers
            let numbers: Vec<i64> = line.split_whitespace().map(|l| l.parse::<i64>().unwrap()).collect();
            for (j, number) in numbers.iter().enumerate() {
                if problems.get(j).is_none() {
                    problems.push(MathProblem { numbers: vec![], operator: '.' });
                }

                problems[j].numbers.push(*number);
            }
        }
    }

    problems
}

fn parse_cephalopod_math_problem(input: &Vec<String>) -> Vec<MathProblem> {
    let mut problems: Vec<MathProblem> = vec![];

    let (operators_row, number_rows) = input.split_last().unwrap();

    let mut columns: Vec<Vec<char>> = vec![];

    for number_row in number_rows {
        for (i, c) in number_row.chars().enumerate() {
            if columns.get(i).is_none() {
                columns.push(vec![]);
            }

            columns[i].push(c);
        }
    }

    let column_strings: Vec<String> = columns.iter().map(|c| c.iter().collect::<String>().trim().to_owned()).collect();

    let mut column_index = 0;
    for column_string in column_strings {
        if !column_string.is_empty() {
            let num = column_string.parse::<i64>().unwrap();
            if problems.get(column_index).is_none() {
                problems.push(MathProblem { numbers: vec![], operator: '.' });
            }
            problems[column_index].numbers.push(num);
        } else {
            column_index += 1;
        }
    }

    // operators
    let operators: Vec<char> = operators_row.split_whitespace().map(|l| l.chars().nth(0).unwrap()).collect();
    for (i, operator) in operators.iter().enumerate() {
        problems[i].operator = *operator;
    }

    problems
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