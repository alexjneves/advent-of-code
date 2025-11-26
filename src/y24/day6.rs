use crate::day::{read_day_input, Day, InputType, Part};

const YEAR_ID: u8 = 24;
const DAY_ID: u8 = 6;

pub struct Day6 {}

impl Day for Day6 {
    fn run(&self, part: Part, input_type: InputType) -> i32 {
        let input = read_day_input(YEAR_ID, DAY_ID, &part, &input_type);

        match part {
            Part::One => part1(&input),
            Part::Two => part2(&input)
        }
    }
}

fn part1(input: &Vec<String>) -> i32 {
    0
}

fn part2(input: &Vec<String>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6_part1_example_input() {
        const EXPECTED_ANSWER: i32 = 0;

        let day6 = Day6 {};
        let answer = day6.run(Part::One, InputType::Example);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day6_part1_custom_input() {
        const EXPECTED_ANSWER: i32 = 0;

        let day6 = Day6 {};
        let answer = day6.run(Part::One,InputType::Custom);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day6_part2_example_input() {
        const EXPECTED_ANSWER: i32 = 0;

        let day6 = Day6 {};
        let answer = day6.run(Part::Two, InputType::Example);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day6_part2_custom_input() {
        const EXPECTED_ANSWER: i32 = 0;

        let day6 = Day6 {};
        let answer = day6.run(Part::Two,InputType::Custom);
        
        assert!(answer == EXPECTED_ANSWER);
    }
}