use std::fs;

pub trait Day {
    fn run(&self, part: Part, input: InputType) -> i32;
}

pub enum Part {
    One,
    Two,
}

pub enum InputType {
    Example,
    Custom,
}

pub fn read_day_input(year: u8, day: u8, part: &Part, input_type: &InputType) -> Vec<String> {
    let contents = read_day_input_string(year, day, part, input_type);

    contents
        .lines()
        .map(|line| line.to_owned())
        .collect()
}

pub fn read_day_input_string(year: u8, day: u8, part: &Part, input_type: &InputType) -> String {
    let path = match input_type {
        InputType::Example => format!(
            "src/y{}/inputs/y{}_d{}_part{}_example_input.txt",
            year,
            year,
            day,
            part_to_int(part)
        ),
        InputType::Custom => format!(
            "src/y{}/inputs/y{}_d{}_custom_input.txt",
            year,
            year,
            day,
        )
    };

    fs::read_to_string(path).unwrap()
}

fn part_to_int(part: &Part) -> i32 {
    match part {
        Part::One => 1,
        Part::Two => 2,
    }
}
