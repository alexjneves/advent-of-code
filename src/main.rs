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
    input: char
}

fn main() {
    let args: Cli = Cli::parse();

    let part = match args.part {
        1 => Part::One,
        2 => Part::Two,
        _ => exit(-1)
    };

    let input = match args.input {
        'e' => InputType::Example,
        'c' => InputType::Custom,
        _ => exit(-1)
    };

    let day = format!("{}-{}", args.year, args.day);

    let answer: Result<i32, String> = match day.as_str() {
        "24-1" => Ok(y24::day1::Day1 {}.run(part, input)),
        "24-2" => Ok(y24::day2::Day2 {}.run(part, input)),
        "24-3" => Ok(y24::day3::Day3 {}.run(part, input)),
        "24-5" => Ok(y24::day5::Day5 {}.run(part, input)),
        "24-6" => Ok(y24::day6::Day6 {}.run(part, input)),
        "24-7" => Ok(y24::day7::Day7 {}.run(part, input)),
        "24-8" => Ok(y24::day8::Day8 {}.run(part, input)),

        "25-1" => Ok(y25::day1::Day1 {}.run(part, input)),

        _ => Err("Invalid year/day provided".to_owned())
    };

    match answer {
        Ok(answer) => println!("Result: {}", answer),
        Err(error)=> println!("Error: {}", error.as_str()) 
    }
}
