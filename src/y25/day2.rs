use fancy_regex::Regex;
use lazy_static::lazy_static;

use crate::day::{Day, InputType, Part, read_day_input_string};

const YEAR_ID: u8 = 25;
const DAY_ID: u8 = 2;

pub struct Day2 {}

#[derive(Debug)]
struct ProductRange {
    first: u64,
    last: u64
}

impl Day for Day2 {
    fn run(&self, part: Part, input_type: InputType) -> i64 {
        let input = read_day_input_string(YEAR_ID, DAY_ID, &part, &input_type);
        let product_ranges = parse_product_ranges(&input);

        println!("{:?}", product_ranges);

        let is_id_valid: fn(u64) -> bool = match part {
            Part::One => is_id_valid_part1,
            Part::Two => is_id_valid_part2,
        };

        let mut invalid_ids: Vec<u64> = vec![];

        for product_range in product_ranges {
            for product_id in product_range.first..=product_range.last {
                if !is_id_valid(product_id) {
                    invalid_ids.push(product_id);
                }
            }
        }

        invalid_ids.iter().sum::<u64>() as i64
    }
}

fn is_id_valid_part1(id: u64) -> bool {
    let id_string = id.to_string();
    let id_len = id_string.len();

    if id_len % 2 == 0 {
        let first_half = &id_string[0..id_len / 2];
        let second_half = &id_string[id_len / 2..];
        
        if first_half == second_half {
            return false;
        }
    }

    true
}

// Note: caching the Regex reduces the runtime from 55 minutes to 13 seconds
lazy_static! {
    static ref PRODUCT_ID_REGEX: Regex = Regex::new(r"^(\d+)(\1+)$").unwrap();
}

fn is_id_valid_part2(id: u64) -> bool {
    !PRODUCT_ID_REGEX.is_match(&id.to_string()).unwrap()
}

fn parse_product_ranges(input: &String) -> Vec<ProductRange> {
    let mut ranges: Vec<ProductRange> = vec![];

    let split = input.split(',');

    for raw_range in split {
        let range_split = raw_range.split_once('-').unwrap();
        
        ranges.push(ProductRange { 
            first: range_split.0.parse().unwrap(), 
            last: range_split.1.parse().unwrap()  
        });
    }

    ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_part1_example_input() {
        const EXPECTED_ANSWER: i64 = 1227775554;

        let day2 = Day2 {};
        let answer = day2.run(Part::One, InputType::Example);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day2_part1_custom_input() {
        const EXPECTED_ANSWER: i64 = 38437576669;

        let day2 = Day2 {};
        let answer = day2.run(Part::One,InputType::Custom);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day2_part2_example_input() {
        const EXPECTED_ANSWER: i64 = 4174379265;

        let day2 = Day2 {};
        let answer = day2.run(Part::Two, InputType::Example);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day2_part2_custom_input() {
        const EXPECTED_ANSWER: i64 = 49046150754;

        let day2 = Day2 {};
        let answer = day2.run(Part::Two,InputType::Custom);
        
        assert_eq!(answer, EXPECTED_ANSWER);
    }
}