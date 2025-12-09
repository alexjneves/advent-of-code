use crate::day::{read_day_input, Day, InputType, Part};

const YEAR_ID: u8 = 25;
const DAY_ID: u8 = 7;

pub struct Day7 {}

impl Day for Day7 {
    fn run(&self, part: Part, input_type: InputType) -> i64 {
        let input = read_day_input(YEAR_ID, DAY_ID, &part, &input_type);

        let rows: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect::<Vec<char>>()).collect();

        match part {
            Part::One => part1(rows),
            Part::Two => part2_count_timelines(&rows)
        }
    }
}

fn part1(mut rows: Vec<Vec<char>>) -> i64 {
    let (first_row, remaining_rows) = rows.split_first_mut().unwrap();

    let start_index = first_row.iter().position(|c| *c == 'S').unwrap();
    remaining_rows[0][start_index] = '|';

    let mut splits = 0;

    let num_rows = remaining_rows.len();
    for row_index in 0..num_rows-1 {
        let [row, next_row] = remaining_rows.get_disjoint_mut([row_index, row_index + 1]).unwrap();

        for (column_index, c) in row.iter().enumerate() {
            if *c != '|' {
                continue;
            }

            if next_row[column_index] == '^' {
                next_row[column_index - 1] = '|';
                next_row[column_index + 1] = '|';
                splits += 1;
            } else {
                next_row[column_index] = '|';
            }
        }
    }
    
    splits
}

enum Cell {
    Start,
    Empty,
    Splitter,
    Laser(u64),
}

fn part2_count_timelines(rows: &Vec<Vec<char>>) -> i64 {
    let mut cells: Vec<Vec<Cell>> = rows.iter()
        .map(|row| 
            row.iter()
                .map(char_to_cell)
                .collect())
        .collect();

    let (first_row, remaining_rows) = cells.split_first_mut().unwrap();

    let start_index = first_row.iter().position(|c| matches!(*c, Cell::Start)).unwrap();
    remaining_rows[0][start_index] = Cell::Laser(1);

    let num_rows = remaining_rows.len();
    for row_index in 0..num_rows-1 {
        let [row, next_row] = remaining_rows.get_disjoint_mut([row_index, row_index + 1]).unwrap();

        for (column_index, cell) in row.iter().enumerate() {
            if let Cell::Laser(count) = *cell {
                // Determine which indexes in the next row the laser will occupy.
                // If we've hit a splitter we go left + right, otherwise we go directly down
                let update_indexes = match next_row[column_index] {
                    Cell::Splitter => vec![column_index - 1, column_index + 1],
                    Cell::Empty | Cell::Laser(_) => vec![column_index],
                    _ => panic!()
                };

                // Add the laser to the cells. 
                // If the cell is empty, move all the lasers. Otherwise, we add to existing lasers
                for index in update_indexes {
                    match next_row[index] {
                        Cell::Empty => {
                            next_row[index] = Cell::Laser(count);
                        },
                        Cell::Laser(existing_count) => {
                            next_row[index] = Cell::Laser(count + existing_count);
                        },
                        _ => panic!()
                    }
                }
            }
        }
    }

    let mut timelines = 0;
    for cell in remaining_rows.last().unwrap() {
        if let Cell::Laser(count) = cell {
            timelines += *count as i64;
        }
    }
    
    timelines
}

#[allow(dead_code)]
fn print_cells(rows: &[Vec<Cell>]) {
    for row in rows {
        print_cell_row(row);
    }
}

fn print_cell_row(cells: &Vec<Cell>) {
    for cell in cells {
        let c: String = match cell {
            Cell::Start => "S".to_string(),
            Cell::Empty => ".".to_string(),
            Cell::Splitter => "^".to_string(),
            Cell::Laser(count) => format!("{}", count).to_string(),
        };

        print!("{}\t", c);
    }
    println!();
}

fn char_to_cell(c: &char) -> Cell {
    match c {
        '.' => Cell::Empty,
        '^' => Cell::Splitter,
        '|' => Cell::Laser(1),
        'S' => Cell::Start,
        _ => panic!("Unknown character: {}", c)
    }
}

// Recursive solution, too slow for large data sets
#[allow(dead_code)]
fn part2_recursive(rows: Vec<Vec<char>>) -> i64 {
    let (first_row, remaining_rows) = rows.split_first().unwrap();

    let start_index = first_row.iter().position(|c| *c == 'S').unwrap();

    return part2_recursive_impl(remaining_rows, start_index);
}

fn part2_recursive_impl(manifold: &[Vec<char>], particle_pos: usize) -> i64 {
    println!("Processing manifold: length {}", manifold.len());

    if manifold.len() == 1 {
        return 1;
    }

    if manifold[1][particle_pos] != '^' {
        return part2_recursive_impl(&manifold[1..], particle_pos);
    }

    return part2_recursive_impl(&manifold[1..], particle_pos - 1) + part2_recursive_impl(&manifold[1..], particle_pos + 1);
}

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
        const EXPECTED_ANSWER: i64 = 40;

        let day7 = Day7 {};
        let answer = day7.run(Part::Two, InputType::Example);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day7_part2_custom_input() {
        const EXPECTED_ANSWER: i64 = 40941112789504;

        let day7 = Day7 {};
        let answer = day7.run(Part::Two,InputType::Custom);
        
        assert_eq!(answer, EXPECTED_ANSWER);
    }
}