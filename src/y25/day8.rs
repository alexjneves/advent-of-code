use core::num;
use std::{cell::RefCell, collections::{HashMap, HashSet}, rc::Rc};

use by_address::ByAddress;

use crate::day::{read_day_input, Day, InputType, Part};

const YEAR_ID: u8 = 25;
const DAY_ID: u8 = 8;

pub struct Day8 {}

#[derive(Hash, Eq, PartialEq, Debug)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64
}

#[derive(Debug)]
struct JunctionBoxPair<'a> {
    box1: &'a JunctionBox,
    box2: &'a JunctionBox,
    distance: f64
}

type Circuit<'a> = Vec<&'a JunctionBox>;

impl Day for Day8 {
    fn run(&self, part: Part, input_type: InputType) -> i64 {
        let input = read_day_input(YEAR_ID, DAY_ID, &part, &input_type);

        let junction_boxes = parse_junction_boxes(&input);

        let mut junction_box_pairs: Vec<JunctionBoxPair> = vec![];
        for i in 0..junction_boxes.len()-1 {
            let box1 = &junction_boxes[i];

            for j in i+1..junction_boxes.len() {
                let box2 = &junction_boxes[j];

                let distance = calculate_distance(box1, box2);
                junction_box_pairs.push(JunctionBoxPair { box1, box2, distance });
            }
        }

        junction_box_pairs.sort_by(|a, b| f64::total_cmp(&a.distance, &b.distance));

        let num_connections_to_eval = match input_type {
            InputType::Example => 10,
            InputType::Custom => 1000
        };

        match part {
            Part::One => part1(&junction_box_pairs[0..num_connections_to_eval]),
            Part::Two => part2(&junction_box_pairs)
        }
    }
}

fn part1(junction_box_pairs: &[JunctionBoxPair]) -> i64 {
    let mut circuit_lookup: HashMap<&JunctionBox, Rc<RefCell<Circuit>>> = HashMap::new();

    for JunctionBoxPair { box1, box2, .. } in junction_box_pairs {
        let circuit1 = Rc::clone(circuit_lookup.entry(box1).or_insert_with(|| Rc::new(RefCell::new(vec![*box1]))));
        let circuit2 = Rc::clone(circuit_lookup.entry(box2).or_insert_with(|| Rc::new(RefCell::new(vec![*box2]))));

        if !Rc::ptr_eq(&circuit1, &circuit2) {
            for junction_box in circuit2.borrow().iter() {
                circuit1.borrow_mut().push(junction_box);
                circuit_lookup.entry(junction_box).and_modify(|v| { *v = Rc::clone(&circuit1) });
            }
        }
    }

    let mut distinct_circuits: HashSet<ByAddress<Rc<RefCell<Circuit>>>> = HashSet::new();
    for circuit in circuit_lookup.values() {
        distinct_circuits.insert(ByAddress(Rc::clone(circuit)));
    }

    let mut distinct_circuits_vec: Vec<&ByAddress<Rc<RefCell<Circuit>>>> = distinct_circuits.iter().collect();
    distinct_circuits_vec.sort_by(|a, b| b.borrow().len().cmp(&a.borrow().len()));

    let product = distinct_circuits_vec.iter()
        .take(3)
        .fold(1, |acc, circuit| acc * circuit.borrow().len());

    product as i64
}

fn part2(junction_box_pairs: &[JunctionBoxPair]) -> i64 {
    let mut circuit_lookup: HashMap<&JunctionBox, Rc<RefCell<Circuit>>> = HashMap::new();
    let mut circuits: HashSet<ByAddress<Rc<RefCell<Circuit>>>> = HashSet::new();

    for JunctionBoxPair { box1, box2, .. } in junction_box_pairs {
        if !circuit_lookup.contains_key(box1) {
            let box1_circuit = Rc::new(RefCell::new(vec![*box1]));
            circuit_lookup.insert(box1, Rc::clone(&box1_circuit));
            circuits.insert(ByAddress(box1_circuit));
        }

        if !circuit_lookup.contains_key(box2) {
            let box2_circuit = Rc::new(RefCell::new(vec![*box2]));
            circuit_lookup.insert(box2, Rc::clone(&box2_circuit));
            circuits.insert(ByAddress(box2_circuit));
        }
    }

    for JunctionBoxPair { box1, box2, .. } in junction_box_pairs {
        let circuit1 = Rc::clone(circuit_lookup.get(box1).unwrap());
        let circuit2 = Rc::clone(circuit_lookup.get(box2).unwrap());

        if !Rc::ptr_eq(&circuit1, &circuit2) {
            for junction_box in circuit2.borrow().iter() {
                circuit1.borrow_mut().push(junction_box);
                circuit_lookup.entry(junction_box).and_modify(|v| { *v = Rc::clone(&circuit1) });
            }

            circuits.remove(&ByAddress(Rc::clone(&circuit2)));
            if circuits.len() == 1 {
                return box1.x * box2.x;
            }
        }
    }

    0
}

fn parse_junction_boxes(input: &Vec<String>) -> Vec<JunctionBox> {
    let mut boxes: Vec<JunctionBox> = vec![];

    for line in input {
        let split: Vec<i64> = line
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        
        boxes.push(JunctionBox { x: split[0], y: split[1], z: split[2] });
    }

    boxes
}

fn calculate_distance(box1: &JunctionBox, box2: &JunctionBox) -> f64 {
    let dx = box1.x - box2.x;
    let dy = box1.y - box2.y;
    let dz = box1.z - box2.z;

    ((dx * dx + dy * dy + dz * dz) as f64).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day8_part1_example_input() {
        const EXPECTED_ANSWER: i64 = 40;

        let day8 = Day8 {};
        let answer = day8.run(Part::One, InputType::Example);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day8_part1_custom_input() {
        const EXPECTED_ANSWER: i64 = 52668;

        let day8 = Day8 {};
        let answer = day8.run(Part::One,InputType::Custom);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day8_part2_example_input() {
        const EXPECTED_ANSWER: i64 = 25272;

        let day8 = Day8 {};
        let answer = day8.run(Part::Two, InputType::Example);

        assert_eq!(answer, EXPECTED_ANSWER);
    }

    #[test]
    fn day8_part2_custom_input() {
        const EXPECTED_ANSWER: i64 = 1474050600;

        let day8 = Day8 {};
        let answer = day8.run(Part::Two,InputType::Custom);
        
        assert_eq!(answer, EXPECTED_ANSWER);
    }
}