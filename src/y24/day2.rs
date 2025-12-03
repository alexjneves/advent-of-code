use crate::day::{Day, InputType, read_day_input, Part};

const YEAR_ID: u8 = 24;
const DAY_ID: u8 = 2;

pub struct Day2 {}

impl Day for Day2 {
    fn run(&self, part: Part, input_type: InputType) -> i64 {
        let input = read_day_input(YEAR_ID, DAY_ID, &part, &input_type);

        match part {
            Part::One => part1(&input),
            Part::Two => part2(&input)
        }
    }
}

fn part1(input: &Vec<String>) -> i64 {
    input.iter()
        .map(parse_report)
        .map(|report| is_level_safe_part_1(&report))
        .filter(|is_safe| *is_safe)
        .count() as i64
}

fn part2(input: &Vec<String>) -> i64 {
    input.iter()
        .map(parse_report)
        .map(|report| is_level_safe_part_2_brute_force(&report))
        .filter(|is_safe| *is_safe)
        .count() as i64
}

fn parse_report(input: &String) -> Vec<u32> {
    input.split_whitespace().map(|i| i.parse::<u32>().unwrap()).collect()
}

fn is_level_safe_part_1(level: &Vec<u32>) -> bool {
    let mut asc: Option<bool> = Option::None;

    for window in level.windows(2) {
        if let [x, y] = window {
            if !is_diff_in_range(x, y) {
                return false;
            }

            if asc.is_none() {
                asc = Option::Some(x < y);
            }

            if asc.unwrap() {
                if x > y {
                    return false;
                }
            } else {
                if x < y {
                    return false;
                }
            }
        }
    }

    true
}

fn is_level_safe_part_2_brute_force(level: &Vec<u32>) -> bool {
    if is_level_safe_part_1(&level) {
        return true;
    }

    for i in 0..level.len() {
        let mut level_without_index = level.to_owned();
        level_without_index.remove(i);

        if is_level_safe_part_1(&level_without_index) {
            return true;
        }
    }

    false
}

// Doesn't work
// fn is_level_safe_part_2(level: &Vec<u32>, problem_dampener: bool) -> bool {
//     let mut asc: Option<bool> = Option::None;
//     let mut allowed_skip = problem_dampener;

//     let mut i  = 0;
//     let mut j = 1;

//     while j < level.len() {
//         let x = level.get(i).unwrap();
//         let y = level.get(j).unwrap();

//         let check_asc = asc.unwrap_or(x < y);

//         let is_valid = is_diff_in_range(x, y) && (if check_asc {x < y} else {x > y});

//         if !is_valid {
//             if !allowed_skip {
//                 return false;
//             }

//             allowed_skip = false;
//         } else {
//             if asc.is_none() {
//                 asc = Some(check_asc);
//             }

//             i = j;
//         }

//         j = j + 1;
//     }

//     true
// }

fn is_diff_in_range(x: &u32, y: &u32) -> bool {
    let diff = x.abs_diff(*y);
    diff >= 1 && diff <= 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_part1_example_input() {
        const EXPECTED_ANSWER: i64 = 2;

        let day2 = Day2 {};
        let answer = day2.run(Part::One, InputType::Example);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day2_part1_custom_input() {
        const EXPECTED_ANSWER: i64 = 321;

        let day2 = Day2 {};
        let answer = day2.run(Part::One,InputType::Custom);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day2_part2_example_input() {
        const EXPECTED_ANSWER: i64 = 4;

        let day2 = Day2 {};
        let answer = day2.run(Part::Two, InputType::Example);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day2_part2_custom_input() {
        const EXPECTED_ANSWER: i64 = 386;

        let day2 = Day2 {};
        let answer = day2.run(Part::Two,InputType::Custom);
        
        assert_eq!(answer, EXPECTED_ANSWER);
    }
}