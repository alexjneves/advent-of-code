use crate::day::{read_day_input, Day, InputType, Part};

const YEAR_ID: u8 = 25;
const DAY_ID: u8 = 3;

pub struct Day3 {}

struct Bank {
    batteries: Vec<u8>
}

impl Day for Day3 {
    fn run(&self, part: Part, input_type: InputType) -> i64 {
        let input = read_day_input(YEAR_ID, DAY_ID, &part, &input_type);

        let get_max_joltage: fn(&Bank) -> u64 = match part {
            Part::One => |bank| get_max_joltage_part2(bank, 2),
            Part::Two => |bank| get_max_joltage_part2(bank, 12)
        };

        input.iter()
            .map(parse_battery_bank)
            .fold(0i64, |sum, bank| sum + get_max_joltage(&bank) as i64)
    }
}

// Part 1 solution supports only two batteries, but requires only a single enumeration of each bank
#[allow(dead_code)]
fn get_max_joltage_part1(bank: &Bank) -> u64 {
    let Bank { batteries } = bank;

    let mut max_left_value: u8 = 0;
    let mut max_right_value: u8 = 0;

    for (i, battery) in batteries.iter().enumerate() {
        // evaluate up to the second-to-last value
        if i == batteries.len() - 1 {
            break;
        }

        if *battery > max_left_value {
            max_left_value = *battery;
            max_right_value = batteries[i + 1];
        } else {
            max_right_value = std::cmp::max(max_right_value, batteries[i + 1]);
        }
    }

    let joltage: u64 = max_left_value as u64 * 10 + max_right_value as u64;

    println!("Max joltage of bank {:?}: {}", batteries, joltage);

    joltage
}

// Part 2 allows for arbitrary battery counts using a sliding window
fn get_max_joltage_part2(bank: &Bank, battery_count: usize) -> u64 {
    let Bank { batteries } = bank;

    let mut joltage: u64 = 0;
    let mut previous_max_index: isize = -1;

    for i in 0..battery_count {
        let window_start = (previous_max_index + 1) as usize;
        let window_end = batteries.len() - (battery_count - i);

        let mut max_battery: u8 = 0;
        for j in window_start..=window_end {
            let battery = batteries[j];
            if battery > max_battery {
                max_battery = battery;
                previous_max_index = j as isize;
            }
        }

        joltage += (max_battery as u64) * 10u64.pow((battery_count - i - 1) as u32);
    }

    println!("Max joltage of bank {:?}: {}", batteries, joltage);
    
    joltage
}

fn parse_battery_bank(input: &String) -> Bank {
    Bank {
        batteries: input.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_part1_example_input() {
        const EXPECTED_ANSWER: i64 = 357;

        let day3 = Day3 {};
        let answer = day3.run(Part::One, InputType::Example);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day3_part1_custom_input() {
        const EXPECTED_ANSWER: i64 = 16812;

        let day3 = Day3 {};
        let answer = day3.run(Part::One,InputType::Custom);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day3_part2_example_input() {
        const EXPECTED_ANSWER: i64 = 3121910778619;

        let day3 = Day3 {};
        let answer = day3.run(Part::Two, InputType::Example);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day3_part2_custom_input() {
        const EXPECTED_ANSWER: i64 = 166345822896410;

        let day3 = Day3 {};
        let answer = day3.run(Part::Two,InputType::Custom);
        
        assert_eq!(answer, EXPECTED_ANSWER);
    }
}