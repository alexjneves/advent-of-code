use crate::day::{read_day_input, Day, InputType, Part};

const YEAR_ID: u8 = 24;
const DAY_ID: u8 = 5;

pub struct Day5 {}

#[derive(Debug)]
struct PageOrderingRule {
    first: i64,
    second: i64
}

impl Day for Day5 {
    fn run(&self, part: Part, input_type: InputType) -> i64 {
        let input = read_day_input(YEAR_ID, DAY_ID, &part, &input_type);

        match part {
            Part::One => part1(&input),
            Part::Two => part2(&input)
        }
    }
}

fn part1(input: &Vec<String>) -> i64 {
    let (rules, pages) = parse_input(input);

    let (valid_pages, _) = split_pages(&rules, &pages);
        
    valid_pages.iter()
        .map(|page| get_middle_page_number(page))
        .sum()
}

fn part2(input: &Vec<String>) -> i64 {
    let (rules, pages) = parse_input(input);

    let (_, invalid_pages) = split_pages(&rules, &pages);
        
    invalid_pages.iter()
        .map(|page| fix_page(&rules, &page))
        .map(|page| get_middle_page_number(&page))
        .sum()
}

fn parse_input(input: &Vec<String>) -> (Vec<PageOrderingRule>, Vec<Vec<i64>>) {
    let mut page_ordering_rules: Vec<PageOrderingRule> = vec![];
    let mut pages: Vec<Vec<i64>> = vec![];

    let mut parse_pages = false;

    for line in input.iter() {
        if line.is_empty() {
            parse_pages = true;
            continue;
        }

        if parse_pages {
            pages.push(parse_page(line));
        } else {
            page_ordering_rules.push(parse_page_order_rule(line));
        }
    }

    (page_ordering_rules, pages)
}

fn parse_page_order_rule(input: &String) -> PageOrderingRule {
    let split: Vec<&str> = input.split('|').collect();
    let first = split.get(0).unwrap();
    let second = split.get(1).unwrap();

    PageOrderingRule {
        first: first.parse().unwrap(),
        second: second.parse().unwrap()
    }
}

fn parse_page(input: &String) -> Vec<i64> {
    input
        .split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect()
}

fn split_pages<'a>(rules: &Vec<PageOrderingRule>, pages: &'a Vec<Vec<i64>>) -> (Vec<&'a Vec<i64>>, Vec<&'a Vec<i64>>) {
    let mut valid_pages: Vec<&Vec<i64>> = vec![];
    let mut invalid_pages: Vec<&Vec<i64>> = vec![];

    for page in pages {
        let mut is_valid = true;

        for rule in rules {
            if !does_page_satisfy_rule(rule, page) {
                is_valid = false;
                break;
            }
        }

        if is_valid {
            valid_pages.push(page);
        } else {
            invalid_pages.push(page);
        }
    }

    (valid_pages, invalid_pages)
}

fn does_page_satisfy_rule(rule: &PageOrderingRule, page: &Vec<i64>) -> bool {
    if let Some(first_index) = page.iter().position(|i| *i == rule.first) {
        if let Some(second_index) = page.iter().position(|i| *i == rule.second) {
            return first_index < second_index;
        }
    }

    return true;
}

fn get_middle_page_number(page: &Vec<i64>) -> i64 {
    let middle_index = page.len() / 2;
    page.get(middle_index).unwrap().to_owned()
}

fn fix_page(rules: &Vec<PageOrderingRule>, page: &Vec<i64>) -> Vec<i64> {
    let mut result = page.to_owned();
    let mut is_valid = false;

    while !is_valid {
        for rule in rules {
            fix_page_for_rule(rule, &mut result);
        }

        is_valid = true;

        for rule in rules {
            if !does_page_satisfy_rule(rule, &result) {
                is_valid = false;
                break;
            }
        }
    }

    result
}

fn fix_page_for_rule(rule: &PageOrderingRule, page: &mut Vec<i64>) -> () {
    if let Some(first_index) = page.iter().position(|i| *i == rule.first) {
        if let Some(second_index) = page.iter().position(|i| *i == rule.second) {
            if first_index > second_index {
                page.swap(first_index, second_index);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_part1_example_input() {
        const EXPECTED_ANSWER: i64 = 143;

        let day5 = Day5 {};
        let answer = day5.run(Part::One, InputType::Example);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day5_part1_custom_input() {
        const EXPECTED_ANSWER: i64 = 6949;

        let day5 = Day5 {};
        let answer = day5.run(Part::One,InputType::Custom);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day5_part2_example_input() {
        const EXPECTED_ANSWER: i64 = 123;

        let day5 = Day5 {};
        let answer = day5.run(Part::Two, InputType::Example);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day5_part2_custom_input() {
        const EXPECTED_ANSWER: i64 = 4145;

        let day5 = Day5 {};
        let answer = day5.run(Part::Two,InputType::Custom);
        
        assert_eq!(answer, EXPECTED_ANSWER);
    }
}