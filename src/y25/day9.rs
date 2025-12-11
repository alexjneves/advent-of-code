use std::collections::HashMap;

use crate::day::{read_day_input, Day, InputType, Part};

const YEAR_ID: u8 = 25;
const DAY_ID: u8 = 9;

pub struct Day9 {}

#[derive(Debug)]
struct Tile {
    x: i64,
    y: i64
}

#[derive(Debug)]
struct MovieTheater {
    rows: HashMap<i64, RowColBound>,
    cols: HashMap<i64, RowColBound>
}

#[derive(Debug)]
struct RowColBound {
    min: i64,
    max: i64
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
    let movie_theater = tiles_to_movie_theater(&tiles);

    let mut largest_area = 0;
    let mut valid_area_count = 0;
    let mut areas: Vec<u64> = vec![];
    for i in 0..tiles.len()-1 {
        let tile1 = &tiles[i];

        for j in i+1..tiles.len() {
            let tile2 = &tiles[j];

            // Corners must be opposite
            if tile1.x == tile2.x || tile1.y == tile2.y {
                continue;
            }

            // Test that opposite corners are in bounds
            if !is_tile_within_bounds(&movie_theater, tile1.x, tile2.y) || !is_tile_within_bounds(&movie_theater, tile2.x, tile1.y) {
                continue;
            }
 
            // Brute force by checking the entire perimeter of the area is in bounds
            let x_min = std::cmp::min(tile1.x, tile2.x);
            let x_max = std::cmp::max(tile1.x, tile2.x);

            let y_min = std::cmp::min(tile1.y, tile2.y);
            let y_max = std::cmp::max(tile1.y, tile2.y);

            let mut invalid_tile_found = false;

            // if x_max - x_min < y_max - y_min {
            //     let mut step = 0;

            //     let mut y_offset = ((y_max - y_min) / (x_max - x_min)) * step;

            //     for x in x_min..=x_max {
            //         // testing against +1 to try and account for integer truncation during division
            //         if !is_tile_within_bounds(&movie_theater, x, y_min + y_offset) || !is_tile_within_bounds(&movie_theater, x, y_min + y_offset + 1) {
            //             invalid_tile_found = true;
            //             break;
            //         }
            //         step = step + 1;
            //         y_offset = ((y_max - y_min) / (x_max - x_min)) * step;
            //     }
            // } else {
            //     let mut step = 0;

            //     let mut x_offset = ((x_max - x_min) / (y_max - y_min)) * step;

            //     for y in y_min..=y_max {
            //         if !is_tile_within_bounds(&movie_theater, x_min + x_offset, y) || !is_tile_within_bounds(&movie_theater, x_min + x_offset + 1, y) {
            //             invalid_tile_found = true;
            //             break;
            //         }
            //         step = step + 1;
            //         x_offset = ((x_max - x_min) / (y_max - y_min)) * step;
            //     }
            // }

            // if invalid_tile_found {
            //     continue;
            // }

            for x in x_min..=x_max {
                if !is_tile_within_bounds(&movie_theater, x, tile1.y) || !is_tile_within_bounds(&movie_theater, x, tile2.y) {
                    invalid_tile_found = true;
                    break;
                }
            }

            if invalid_tile_found {
                continue;
            }

            for y in y_min..=y_max {
                if !is_tile_within_bounds(&movie_theater, tile1.x, y) || !is_tile_within_bounds(&movie_theater, tile2.x, y) {
                    invalid_tile_found = true;
                    break;
                }
            }

            if invalid_tile_found {
                continue;
            }

            let area = calculate_area(tile1, tile2);
            valid_area_count = valid_area_count + 1;
            areas.push(area);

            largest_area = std::cmp::max(largest_area, area);
        }
    }

    // Debug, remove
    println!("Valid areas: {}", valid_area_count);
    areas.sort();
    areas.reverse();

    largest_area as i64
}

// Transform tiles into lookup tables that contain valid ranges for each row/col
fn tiles_to_movie_theater(tiles: &[Tile]) -> MovieTheater {
    let mut movie_theater = MovieTheater { rows: HashMap::new(), cols: HashMap::new() };

    for i in 0..tiles.len()-1 {
        insert_red_tiles_into_movie_theater(&mut movie_theater, &tiles[i], &tiles[i + 1]);
    }

    insert_red_tiles_into_movie_theater(&mut movie_theater, tiles.last().unwrap(), tiles.first().unwrap());

    movie_theater
}

fn insert_red_tiles_into_movie_theater(movie_theater: &mut MovieTheater, tile1: &Tile, tile2: &Tile) {
    if tile1.x == tile2.x {
        for i in std::cmp::min(tile1.y, tile2.y)..=std::cmp::max(tile1.y, tile2.y) {
            movie_theater.rows.entry(i)
                .and_modify(|v| update_bounds(v, tile1.x))
                .or_insert(RowColBound { min: tile1.x, max: tile1.x });
        }
    } else /* y == y */ {
        for i in std::cmp::min(tile1.x, tile2.x)..=std::cmp::max(tile1.x, tile2.x) {
            movie_theater.cols.entry(i)
                .and_modify(|v| update_bounds(v, tile1.y))
                .or_insert(RowColBound { min: tile1.y, max: tile1.y });
        }
    }
}

fn update_bounds(bounds: &mut RowColBound, n: i64) {
    if n < bounds.min {
        bounds.min = n;
    } else if n > bounds.max {
        bounds.max = n;
    }
}

fn is_tile_within_bounds(movie_theater: &MovieTheater, x: i64, y: i64) -> bool {
    let x_within_bounds = match movie_theater.rows.get(&y) {
        Some(row) => x >= row.min && x <= row.max,
        None => false
    };

    if !x_within_bounds {
        return false;
    }

    match movie_theater.cols.get(&x) {
        Some(col) => y >= col.min && y <= col.max,
        None => false
    }
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
    let xdiff = (tile1.x - tile2.x).unsigned_abs() + 1;
    let ydiff = (tile1.y - tile2.y).unsigned_abs() + 1;

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
        const EXPECTED_ANSWER: i64 = 24;

        let day9 = Day9 {};
        let answer = day9.run(Part::Two, InputType::Example);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day9_part2_custom_input() {
        const EXPECTED_ANSWER: i64 = 1550760868;

        let day9 = Day9 {};
        let answer = day9.run(Part::Two,InputType::Custom);
        
        assert_eq!(answer, EXPECTED_ANSWER);
    }
}