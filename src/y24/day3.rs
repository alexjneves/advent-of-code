use regex::Regex;

use crate::day::{read_day_input_string, Day, InputType, Part};

const YEAR_ID: u8 = 24;
const DAY_ID: u8 = 3;

pub struct Day3 {}

struct Mult {
    x: i64,
    y: i64
}

impl Day for Day3 {
    fn run(&self, part: Part, input_type: InputType) -> i64 {
        let input = read_day_input_string(YEAR_ID, DAY_ID, &part, &input_type);

        match part {
            Part::One => part1(&input),
            Part::Two => part2(&input)
        }
    }
}

fn part1(input: &String) -> i64 {
    parse_mults(input)
        .iter()
        .map(|m| m.x * m.y)
        .fold(0, |acc, x| acc + x)
}

fn part2(input: &String) -> i64 {
    parse_mults_with_do_dont(input)
        .iter()
        .map(|m| m.x * m.y)
        .fold(0, |acc, x| acc + x)
}

fn parse_mults(input: &String) -> Vec<Mult> {
    let re = Regex::new(r"mul\((?<x>\d{1,3}),(?<y>\d{1,3})\)").unwrap();

    re.captures_iter(input).map(|capture| {
        let x_s = capture.name("x").unwrap().as_str();
        let y_s = capture.name("y").unwrap().as_str();

        Mult {
            x: x_s.parse().unwrap(), 
            y: y_s.parse().unwrap()
        }
    }).collect()
}

fn parse_mults_with_do_dont(input: &String) -> Vec<Mult> {
    let mut to_parse: Vec<String> = vec![];

    let split_on_do: Vec<&str> = input
        .split("do()")
        .collect();

    for split_do in split_on_do.iter() {
        let do_parse = split_do
            .split("don't()")
            .collect::<Vec<&str>>()
            .get(0)
            .unwrap()
            .to_owned();

        to_parse.push((*do_parse).to_owned());
    }

    to_parse
        .iter()
        .map(parse_mults)
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_part1_example_input() {
        const EXPECTED_ANSWER: i64 = 161;

        let day3 = Day3 {};
        let answer = day3.run(Part::One, InputType::Example);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day3_part1_custom_input() {
        const EXPECTED_ANSWER: i64 = 167090022;

        let day3 = Day3 {};
        let answer = day3.run(Part::One,InputType::Custom);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day3_part2_example_input() {
        const EXPECTED_ANSWER: i64 = 48;

        let day3 = Day3 {};
        let answer = day3.run(Part::Two, InputType::Example);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day3_part2_custom_input() {
        const EXPECTED_ANSWER: i64 = 89823704;

        let day3 = Day3 {};
        let answer = day3.run(Part::Two,InputType::Custom);
        
        assert_eq!(answer, EXPECTED_ANSWER);
    }
}