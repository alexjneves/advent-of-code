use crate::day::{read_day_input, Day, InputType, Part};

const YEAR_ID: u8 = 25;
const DAY_ID: u8 = 4;

const MAX_ADJACENT_PAPER: usize = 3;

pub struct Day4 {}

#[derive(Debug)]
struct PaperGrid {
    grid: Vec<char>,
    width: usize,
    height: usize
}

impl Day for Day4 {
    fn run(&self, part: Part, input_type: InputType) -> i64 {
        let input = read_day_input(YEAR_ID, DAY_ID, &part, &input_type);

        let grid = parse_grid(&input);

        match part {
            Part::One => part1(&grid),
            Part::Two => part2(&grid)
        }
    }
}

fn part1(paper_grid: &PaperGrid) -> i64 {
    let PaperGrid { grid, width, height } = paper_grid;

    let mut accessible_rolls = 0;

    for (i, element) in grid.iter().enumerate() {
        if *element != '@' {
            continue;
        }

        let adjacent_indexes = get_indexes_of_adjacent_elements(i as isize, *width, *height);

        let mut adjacent_paper_rolls = 0;
        for index in adjacent_indexes {
            if grid[index] == '@' {
                adjacent_paper_rolls += 1;
            }
        }

        if adjacent_paper_rolls <= MAX_ADJACENT_PAPER {
            accessible_rolls += 1;
        }
    }

    accessible_rolls
}

fn part2(paper_grid: &PaperGrid) -> i64 {
    let PaperGrid { grid: grid_immutable, width, height } = paper_grid;

    let mut grid: Vec<char> = grid_immutable.to_vec();
    let mut accessible_rolls = 0;

    loop {
        let mut paper_to_remove: Vec<usize> = vec![];

        for (i, element) in grid.iter().enumerate() {
            if *element != '@' {
                continue;
            }

            let adjacent_indexes = get_indexes_of_adjacent_elements(i as isize, *width, *height);

            let mut adjacent_paper_rolls = 0;
            for index in adjacent_indexes {
                if grid[index] == '@' {
                    adjacent_paper_rolls += 1;
                }
            }

            if adjacent_paper_rolls <= MAX_ADJACENT_PAPER {
                accessible_rolls += 1;
                paper_to_remove.push(i);
            }
        }

        // With Rust borrow semantics, we cannot borrow both a mutable and immutable reference at the same time.
        // This means we have to do additional work to remove the paper, rather than doing it in-line in the previous loop
        for paper in paper_to_remove.iter() {
            grid[*paper] = '.';
        }

        // If no paper was removed then we've done the final evaluation
        if paper_to_remove.len() == 0 {
            break;
        }
    }

    accessible_rolls
}

fn get_indexes_of_adjacent_elements(index: isize, grid_width: usize, grid_height: usize) -> Vec<usize> {
    let mut valid_indexes: Vec<usize> = vec![];

    let mut add_if_valid_index = |i| { 
        if i >= 0 && i < ((grid_width * grid_height) as isize) {
            valid_indexes.push(i as usize);
        }
    };
    
    let is_left_edge = index % (grid_width as isize) == 0;
    let is_right_edge = (index + 1) % (grid_width as isize) == 0;

    // top left
    if !is_left_edge {
        add_if_valid_index(index - (grid_width as isize) - 1);
    }

    // top middle
    add_if_valid_index(index - (grid_width as isize));

    // top right
    if !is_right_edge {
        add_if_valid_index(index - (grid_width as isize) + 1);
    }

    // left
    if !is_left_edge {
        add_if_valid_index(index - 1);
    }

    // right
    if !is_right_edge {
        add_if_valid_index(index + 1);
    }

    // bottom left
    if !is_left_edge {
        add_if_valid_index(index + (grid_width as isize) - 1);
    }

    // bottom middle
    add_if_valid_index(index + (grid_width as isize));

    // bottom right
    if !is_right_edge {
        add_if_valid_index(index + (grid_width as isize) + 1);
    }

    valid_indexes
}

fn parse_grid(input: &Vec<String>) -> PaperGrid {
    let width = input.first().unwrap().len();
    let height = input.len();

    let mut grid: Vec<char> = vec![];

    for row in input {
        for c in row.chars() {
            grid.push(c);
        }
    }

    PaperGrid { grid, width, height }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4_part1_example_input() {
        const EXPECTED_ANSWER: i64 = 13;

        let day4 = Day4 {};
        let answer = day4.run(Part::One, InputType::Example);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day4_part1_custom_input() {
        const EXPECTED_ANSWER: i64 = 1344;

        let day4 = Day4 {};
        let answer = day4.run(Part::One,InputType::Custom);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day4_part2_example_input() {
        const EXPECTED_ANSWER: i64 = 43;

        let day4 = Day4 {};
        let answer = day4.run(Part::Two, InputType::Example);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day4_part2_custom_input() {
        const EXPECTED_ANSWER: i64 = 8112;

        let day4 = Day4 {};
        let answer = day4.run(Part::Two,InputType::Custom);
        
        assert_eq!(answer, EXPECTED_ANSWER);
    }
}