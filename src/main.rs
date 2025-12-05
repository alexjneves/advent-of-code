mod day;
mod y24;
mod y25;

use std::process::exit;

use clap::Parser;
use day::{Day, InputType, Part};

#[derive(Parser)]
struct Cli {
    year: u8,
    day: u8,
    part: u8,
    input: char,
}

fn main() {
    let args: Cli = Cli::parse();

    let part = match args.part {
        1 => Part::One,
        2 => Part::Two,
        _ => exit(-1),
    };

    let input = match args.input {
        'e' => InputType::Example,
        'c' => InputType::Custom,
        _ => exit(-1),
    };

    let day = match get_day(args.year, args.day) {
        Ok(day) => day,
        Err(error) => {
            println!("Error: {}", error.as_str());
            exit(-1);
        }
    };

    let answer = day.run(part, input);

    println!("Result: {}", answer);
}

fn get_day(year: u8, day: u8) -> Result<Box<dyn Day>, String> {
    match (year, day) {
        (24, 1) => Ok(Box::new(y24::day1::Day1 {})),
        (24, 2) => Ok(Box::new(y24::day2::Day2 {})),
        (24, 3) => Ok(Box::new(y24::day3::Day3 {})),
        (24, 5) => Ok(Box::new(y24::day5::Day5 {})),

        (25, 1) => Ok(Box::new(y25::day1::Day1 {})),
        (25, 2) => Ok(Box::new(y25::day2::Day2 {})),
        (25, 3) => Ok(Box::new(y25::day3::Day3 {})),
        (25, 4) => Ok(Box::new(y25::day4::Day4 {})),

        _ => Err("Invalid year/day provided".to_owned())
    }
}
