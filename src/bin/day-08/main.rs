use std::{collections::{HashMap, HashSet}, ops::{Mul, Neg}};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position(
    usize,
    usize,
);

#[derive(Debug, Clone, Copy)]
struct Offset(
    isize,
    isize,
);

impl Neg for Offset {
    type Output = Self;

    fn neg(self) -> Self::Output {
        return Offset(
            -self.0,
            -self.1
        );
    }
}

impl Mul<isize> for Offset {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        return Offset(
            self.0 * rhs,
            self.1 * rhs,
        );
    }
}

#[derive(Debug)]
struct City {
    bounds: (usize, usize),
    antennae: HashMap<char, Vec<Position>>,
}

fn parse_input(input: &str) -> City {
    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().chars().count();

    let mut antennae = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        for (j, character) in line.chars().enumerate() {
            if character == '.' {
                continue;
            }

            antennae.entry(character)
                .and_modify(|positions: &mut Vec<Position>| positions.push(Position(i, j)))
                .or_insert(vec![Position(i, j)]);
        }
    }

    return City {
        bounds: (num_rows, num_cols),
        antennae,
    };
}

fn difference(position: &Position, other_position: &Position) -> Offset {
    let row_offset = other_position.0 as isize - position.0 as isize;
    let col_offset = other_position.1 as isize - position.1 as isize;

    return Offset(row_offset, col_offset);
}

// Returns None if the new position is out of the bounds
fn add_offset_bounded(position: &Position, offset: &Offset, bounds: &(usize, usize)) -> Option<Position> {
    let new_row = position.0 as isize + offset.0;
    let new_col = position.1 as isize + offset.1;

    if new_row < 0 || new_row >= bounds.0 as isize {
        return None;
    }

    if new_col < 0 || new_col >= bounds.1 as isize {
        return None;
    }

    return Some(Position(new_row as usize, new_col as usize));
}

fn part_1(input: &str) -> usize {
    let city = parse_input(input);

    let mut resonant_locations = HashSet::<Position>::new();

    // Which character it is, is irrelevant at this point
    for positions in city.antennae.values() {
        for pair in positions.iter().combinations(2) {
            let position = pair[0];
            let other_position = pair[1];

            let offset = difference(position, other_position);

            if let Some(first_resonance) = add_offset_bounded(position, &-offset, &city.bounds) {
                resonant_locations.insert(first_resonance);
            }

            if let Some(second_resonance) = add_offset_bounded(other_position, &offset, &city.bounds) {
                resonant_locations.insert(second_resonance);
            }
        }
    }

    return resonant_locations.len();
}

fn all_resonant_locations(position: &Position, offset: &Offset, bounds: &(usize, usize)) -> Vec<Position> {
    let mut result = vec![];

    // First positive multiples of offset
    let mut i = 0;

    while let Some(resonant_location) = add_offset_bounded(position, &(*offset * i), bounds) {
        result.push(resonant_location);

        i += 1;
    }

    // Then negative offsets
    let mut i = -1;

    while let Some(resonant_location) = add_offset_bounded(position, &(*offset * i), bounds) {
        result.push(resonant_location);

        i -= 1;
    }

    return result;
}

fn part_2(input: &str) -> usize {
    let city = parse_input(input);

    let mut resonant_locations = HashSet::<Position>::new();

    // Which character it is, is irrelevant at this point
    for positions in city.antennae.values() {
        for pair in positions.iter().combinations(2) {
            let position = pair[0];
            let other_position = pair[1];

            let offset = difference(position, other_position);

            resonant_locations.extend(all_resonant_locations(position, &offset, &city.bounds));
        }
    }

    return resonant_locations.len();
}

fn main() {
    let test_input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    let other_test_input = "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";

    let actual_input = &std::fs::read_to_string("src/bin/day-08/input.txt").unwrap();

    let result_1 = part_1(actual_input);

    println!("Part 1: {result_1}");

    let result_2 = part_2(actual_input);

    println!("Part 2: {result_2}");
}
