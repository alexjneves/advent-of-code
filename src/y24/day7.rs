use crate::day::{read_day_input, Day, InputType, Part};

const YEAR_ID: u8 = 24;
const DAY_ID: u8 = 7;

pub struct Day7 {}

impl Day for Day7 {
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
    fn day7_part1_example_input() {
        const EXPECTED_ANSWER: i32 = 0;

        let day7 = Day7 {};
        let answer = day7.run(Part::One, InputType::Example);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day7_part1_custom_input() {
        const EXPECTED_ANSWER: i32 = 0;

        let day7 = Day7 {};
        let answer = day7.run(Part::One,InputType::Custom);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day7_part2_example_input() {
        const EXPECTED_ANSWER: i32 = 0;

        let day7 = Day7 {};
        let answer = day7.run(Part::Two, InputType::Example);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day7_part2_custom_input() {
        const EXPECTED_ANSWER: i32 = 0;

        let day7 = Day7 {};
        let answer = day7.run(Part::Two,InputType::Custom);
        
        assert!(answer == EXPECTED_ANSWER);
    }
}