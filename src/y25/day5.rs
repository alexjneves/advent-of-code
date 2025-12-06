use std::{collections::HashSet, ops::Range};

use crate::day::{read_day_input, Day, InputType, Part};

const YEAR_ID: u8 = 25;
const DAY_ID: u8 = 5;

pub struct Day5 {}

#[derive(Debug)]
struct IngredientDatabase {
    fresh_ingredient_ranges: Vec<Range<u64>>,
    available_ingredients: Vec<u64>
}

impl Day for Day5 {
    fn run(&self, part: Part, input_type: InputType) -> i64 {
        let input = read_day_input(YEAR_ID, DAY_ID, &part, &input_type);

        let ingredient_database = parse_ingredient_database(&input);

        match part {
            Part::One => part1(&ingredient_database),
            Part::Two => part2(&ingredient_database)
        }
    }
}

fn part1(ingredient_database: &IngredientDatabase) -> i64 {
    let mut fresh_ingredient_count: i64 = 0;

    for ingredient in ingredient_database.available_ingredients.iter() {
        for fresh_ingredient_range in ingredient_database.fresh_ingredient_ranges.iter() {
            if (fresh_ingredient_range.start..=fresh_ingredient_range.end).contains(&ingredient) {
                fresh_ingredient_count += 1;
                break;
            }
        }
    }

    fresh_ingredient_count
}

fn part2(ingredient_database: &IngredientDatabase) -> i64 {
    let mut reduced_ranges = ingredient_database.fresh_ingredient_ranges.clone();

    loop {
        let (updated_ranges, reduced) = reduce_ranges(&reduced_ranges);

        // when no reductions occurred we've collapsed all of the overlapping ranges
        if !reduced {
            break;
        }

        reduced_ranges = updated_ranges;
    }

    let mut fresh_ingredients_count: i64 = 0;

    for range in &reduced_ranges {
        fresh_ingredients_count += (range.end - range.start + 1) as i64;
    }

    fresh_ingredients_count
}

fn reduce_ranges(ranges: &Vec<Range<u64>>) -> (Vec<Range<u64>>, bool) {
    let mut reduced_ranges: Vec<Range<u64>> = vec![];
    let mut reduced = false;

    for range in ranges {
        let mut optimized_range_updated = false;

        for reduced_range in &mut reduced_ranges {
            // case: range fits within existing range
            if range.start >= reduced_range.start && range.end <= reduced_range.end {
                optimized_range_updated = true;
                break;
            }

            // case: range encapsulates existing range
            if reduced_range.start >= range.start && reduced_range.end <= range.end {
                reduced_range.start = range.start;
                reduced_range.end = range.end;
                optimized_range_updated = true;
                break;
            }

            // case: range overlaps with existing range max
            if (range.start >= reduced_range.start && range.start <= reduced_range.end) && range.end > reduced_range.end {
                reduced_range.end = range.end;
                optimized_range_updated = true;
                break;
            }

            // case: range overlaps with existing range min
            if (range.end >= reduced_range.start && range.end <= reduced_range.end) && range.start < reduced_range.start {
                reduced_range.start = range.start;
                optimized_range_updated = true;
                break;
            }
        }

        // push if it is a unique range
        if !optimized_range_updated {
            reduced_ranges.push(range.clone());
        } else  {
            reduced = true;
        }
    }

    (reduced_ranges, reduced)
}

fn parse_ingredient_database(input: &Vec<String>) -> IngredientDatabase {
    let mut fresh_ingredient_ranges: Vec<Range<u64>> = vec![];
    let mut available_ingredients: Vec<u64> = vec![];

    let mut line_break_found = false;

    for item in input.iter() {
        if item.is_empty() {
            line_break_found = true;
            continue;
        }

        if !line_break_found {
            // ranges
            let split = item.split_once('-').unwrap();
            let range = split.0.parse::<u64>().unwrap()..split.1.parse::<u64>().unwrap();
            fresh_ingredient_ranges.push(range);
        } else {
            // ingredients
            available_ingredients.push(item.parse::<u64>().unwrap());
        }
    }

    IngredientDatabase { fresh_ingredient_ranges, available_ingredients }
}

#[allow(dead_code)]
fn part2_brute_force(ingredient_database: &IngredientDatabase) -> i64 {
    let mut fresh_ingredient_ids: HashSet<u64> = HashSet::new();

    for range in &ingredient_database.fresh_ingredient_ranges {
        for id in range.start..=range.end {
            fresh_ingredient_ids.insert(id);
        }
    }

    fresh_ingredient_ids.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_part1_example_input() {
        const EXPECTED_ANSWER: i64 = 3;

        let day5 = Day5 {};
        let answer = day5.run(Part::One, InputType::Example);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day5_part1_custom_input() {
        const EXPECTED_ANSWER: i64 = 733;

        let day5 = Day5 {};
        let answer = day5.run(Part::One,InputType::Custom);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day5_part2_example_input() {
        const EXPECTED_ANSWER: i64 = 14;

        let day5 = Day5 {};
        let answer = day5.run(Part::Two, InputType::Example);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day5_part2_custom_input() {
        const EXPECTED_ANSWER: i64 = 345821388687084;

        let day5 = Day5 {};
        let answer = day5.run(Part::Two,InputType::Custom);
        
        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn reduced_ranges_tests() {
        let r1: Range<u64> = 485988698650749..489359725670561;
        let r2: Range<u64> = 484319494831395..491517821437404;

        let mut ranges = vec![];
        ranges.push(r1);
        ranges.push(r2);

        let (updated_ranges, reduced) = reduce_ranges(&ranges);

        assert_eq!(true, reduced);
        assert_eq!(1, updated_ranges.len());
    }
}