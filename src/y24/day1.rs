use std::collections::HashMap;

use crate::day::{Day, InputType, read_day_input, Part};

const YEAR_ID: u8 = 24;
const DAY_ID: u8 = 1;

pub struct Day1 {}

impl Day for Day1 {
    fn run(&self, part: Part, input_type: InputType) -> i32 {
        let input = read_day_input(YEAR_ID, DAY_ID, &part, &input_type);

        let (column1, column2) = get_columns(&input);

        match part {
            Part::One => part1(column1, column2),
            Part::Two => part2(&column1, &column2)
        }
    }
}

fn part1(mut column1: Vec<i32>, mut column2: Vec<i32>) -> i32 {
    column1.sort();
    column2.sort();

    let total: i32 = column1.iter().zip(column2.iter())
        .fold(0, |acc, pair| acc + pair.0.abs_diff(*(pair.1)) as i32);

    total
}

fn part2(column1: &Vec<i32>, column2: &Vec<i32>) -> i32 {
    let mut total: i32 = 0;

    let mut scores: HashMap<i32, i32> = HashMap::new();

    for num in column1 {
        if let Some(score) = scores.get(num) {
            total += score;
            continue;
        }

        let count = column2.iter().filter(|x| *x == num).count() as i32;
        let score = num * count;

        scores.insert(*num, score);
        total += score;
    }

    total
}

fn get_columns(input: &Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    for line in input {
        let numbers: Vec<&str> = line.split_whitespace().collect();

        let first = numbers.get(0).unwrap().parse::<i32>().unwrap();
        column1.push(first);

        let second = numbers.get(1).unwrap().parse::<i32>().unwrap();
        column2.push(second);
    }

    (column1, column2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_part1_example_input() {
        const EXPECTED_ANSWER: i32 = 11;

        let day1 = Day1 {};
        let answer = day1.run(Part::One, InputType::Example);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day1_part1_custom_input() {
        const EXPECTED_ANSWER: i32 = 1388114;

        let day1 = Day1 {};
        let answer = day1.run(Part::One,InputType::Custom);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day1_part2_example_input() {
        const EXPECTED_ANSWER: i32 = 31;

        let day1 = Day1 {};
        let answer = day1.run(Part::Two, InputType::Example);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day1_part2_custom_input() {
        const EXPECTED_ANSWER: i32 = 23529853;

        let day1 = Day1 {};
        let answer = day1.run(Part::Two,InputType::Custom);

        assert!(answer == EXPECTED_ANSWER);
    }
}