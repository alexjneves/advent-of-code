use crate::day::{read_day_input, Day, InputType, Part};

const YEAR_ID: u8 = 25;
const DAY_ID: u8 = 9;

pub struct Day9 {}

#[derive(Debug)]
struct Tile {
    x: i64,
    y: i64
}

impl Day for Day9 {
    fn run(&self, part: Part, input_type: InputType) -> i64 {
        let input = read_day_input(YEAR_ID, DAY_ID, &part, &input_type);

        let tiles = parse_tiles(&input);

        match part {
            Part::One => part1(&tiles),
            Part::Two => part2(&tiles)
        }
    }
}

fn part1(tiles: &[Tile]) -> i64 {
    let mut largest_area = 0;
    for i in 0..tiles.len()-1 {
        let tile1 = &tiles[i];

        for j in i+1..tiles.len() {
            let tile2 = &tiles[j];

            let area = calculate_area(tile1, tile2);
            largest_area = std::cmp::max(largest_area, area);
        }
    }

    largest_area as i64
}

fn part2(tiles: &[Tile]) -> i64 {
    0
}

fn parse_tiles(input: &Vec<String>) -> Vec<Tile> {
    let mut tiles: Vec<Tile> = vec![];

    for line in input {
        let split: Vec<i64> = line
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        
        tiles.push(Tile { x: split[0], y: split[1] });
    }

    tiles
}

fn calculate_area(tile1: &Tile, tile2: &Tile) -> u64 {
    let xdiff = (tile1.x as i64 - tile2.x as i64).unsigned_abs() + 1;
    let ydiff = (tile1.y as i64 - tile2.y as i64).unsigned_abs() + 1;

    xdiff * ydiff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9_part1_example_input() {
        const EXPECTED_ANSWER: i64 = 50;

        let day9 = Day9 {};
        let answer = day9.run(Part::One, InputType::Example);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day9_part1_custom_input() {
        const EXPECTED_ANSWER: i64 = 4748985168;

        let day9 = Day9 {};
        let answer = day9.run(Part::One,InputType::Custom);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day9_part2_example_input() {
        const EXPECTED_ANSWER: i64 = 0;

        let day9 = Day9 {};
        let answer = day9.run(Part::Two, InputType::Example);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day9_part2_custom_input() {
        const EXPECTED_ANSWER: i64 = 0;

        let day9 = Day9 {};
        let answer = day9.run(Part::Two,InputType::Custom);
        
        assert_eq!(answer, EXPECTED_ANSWER);
    }
}