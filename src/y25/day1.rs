use crate::day::{read_day_input, Day, InputType, Part};

const YEAR_ID: u8 = 25;
const DAY_ID: u8 = 1;

pub struct Day1 {}

#[derive(Debug)]
#[derive(PartialEq)]
enum Direction {
    Left,
    Right
}

#[derive(Debug)]
struct Rotation {
    direction: Direction,
    distance: i32
}

impl Day for Day1 {
    fn run(&self, part: Part, input_type: InputType) -> i32 {
        let input = read_day_input(YEAR_ID, DAY_ID, &part, &input_type);

        let rotations = input.iter().map(parse_rotation).collect();
        
        match part {
            Part::One => part1(&rotations),
            Part::Two => part2(&rotations)
        }
    }
}

fn parse_rotation(input: &String) -> Rotation {
    let direction = match &input[0..1] {
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("Invalid direction: {}", &input[0..0])
    };

    Rotation { 
        direction, 
        distance: (&input[1..]).parse().unwrap() 
    }
}

fn part1(rotations: &Vec<Rotation>) -> i32 {
    let mut dial: i32 = 50;
    let mut zero_count: i32 = 0;

    for rotation in rotations  {
        let rotation_distance: i32 = match rotation.direction {
            Direction::Right => rotation.distance % 100,
            Direction::Left => 100 - (rotation.distance % 100)
        };

        dial = (dial + rotation_distance) % 100;

        if dial == 0 {
            zero_count += 1;
        }

        assert!(dial >= 0 && dial <= 99, "Dial = {}", dial);
    }

    zero_count
}

fn part2(rotations: &Vec<Rotation>) -> i32 {
    let mut dial: i32 = 50;
    let mut zero_count: i32 = 0;

    for r in rotations  {
        let distance_modulo = r.distance % 100;

        if r.direction == Direction::Right {
            if dial + distance_modulo > 99 {
                zero_count += 1;

                dial = (dial + distance_modulo) % 100;
            } else {
                dial = dial + distance_modulo;
            }
        } else {
            if dial - distance_modulo < 0 {
                if dial != 0 {
                    zero_count += 1;
                }

                dial = 100 - (distance_modulo - dial);
            } else {
                dial = dial - distance_modulo;
            }

            if dial == 0 {
                zero_count += 1;
            }
        }

        let full_rotations = r.distance / 100;
        zero_count += full_rotations;

        assert!(dial >= 0 && dial <= 99, "Dial = {}", dial);
    }

    zero_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_part1_example_input() {
        const EXPECTED_ANSWER: i32 = 3;

        let day1 = Day1 {};
        let answer = day1.run(Part::One, InputType::Example);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day1_part1_custom_input() {
        const EXPECTED_ANSWER: i32 = 1007;

        let day1 = Day1 {};
        let answer = day1.run(Part::One,InputType::Custom);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day1_part2_example_input() {
        const EXPECTED_ANSWER: i32 = 6;

        let day1 = Day1 {};
        let answer = day1.run(Part::Two, InputType::Example);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day1_part2_custom_input() {
        const EXPECTED_ANSWER: i32 = 5820;

        let day1 = Day1 {};
        let answer = day1.run(Part::Two,InputType::Custom);
        
        assert_eq!(answer, EXPECTED_ANSWER);
    }
}