use crate::day::{read_day_input, Day, InputType, Part};

const YEAR_ID: u8 = 25;
const DAY_ID: u8 = 7;

pub struct Day7 {}

impl Day for Day7 {
    fn run(&self, part: Part, input_type: InputType) -> i64 {
        let input = read_day_input(YEAR_ID, DAY_ID, &part, &input_type);

        let mut rows: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect::<Vec<char>>()).collect();

        let (first_row, remaining_rows) = rows.split_first_mut().unwrap();

        let start_index = first_row.iter().position(|c| *c == 'S').unwrap();
        remaining_rows[0][start_index] = '|';

        let mut splits = 0;

        // let num_rows = remaining_rows.len();
        // for (row_index, row) in remaining_rows[..num_rows-1].iter_mut().enumerate() {
        //     let next_row: &mut Vec<char> = &mut remaining_rows[row_index + 1];

        //     for (column_index, c) in row.iter().enumerate() {
        //         if *c != '|' {
        //             continue;
        //         }

        //         // if next_row[column_index] == '.' {
        //         //     next_row[column_index] = '|';
        //         // }
        //     }
        // }

        let num_rows = remaining_rows.len();
        for row_index in 0..num_rows-1 {
            let [row, next_row] = remaining_rows.get_disjoint_mut([row_index, row_index + 1]).unwrap();

            for (column_index, c) in row.iter().enumerate() {
                if *c != '|' {
                    continue;
                }

                if next_row[column_index] == '^' {
                    // if next_row[column_index - 1] == '.' {
                    //     next_row[column_index - 1] = '|';
                    //     splits += 1;
                    // }

                    // if next_row[column_index + 1] == '.' {
                    //     next_row[column_index + 1] = '|';
                    //     splits += 1;
                    // }
                    next_row[column_index - 1] = '|';
                    next_row[column_index + 1] = '|';
                    splits += 1;
                } else {
                    next_row[column_index] = '|';
                }
            }
        }
        

        splits

        // match part {
        //     Part::One => part1(&input),
        //     Part::Two => part2(&input)
        // }
    }
}

// fn part1(input: &Vec<String>) -> i64 {
//     0
// }

// fn part2(input: &Vec<String>) -> i64 {
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7_part1_example_input() {
        const EXPECTED_ANSWER: i64 = 21;

        let day7 = Day7 {};
        let answer = day7.run(Part::One, InputType::Example);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day7_part1_custom_input() {
        const EXPECTED_ANSWER: i64 = 1662;

        let day7 = Day7 {};
        let answer = day7.run(Part::One,InputType::Custom);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day7_part2_example_input() {
        const EXPECTED_ANSWER: i64 = 0;

        let day7 = Day7 {};
        let answer = day7.run(Part::Two, InputType::Example);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day7_part2_custom_input() {
        const EXPECTED_ANSWER: i64 = 0;

        let day7 = Day7 {};
        let answer = day7.run(Part::Two,InputType::Custom);
        
        assert_eq!(answer, EXPECTED_ANSWER);
    }
}