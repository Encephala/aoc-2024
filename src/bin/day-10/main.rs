use std::collections::{HashMap, HashSet, VecDeque};

use itertools::kmerge;

type Bounds = (usize, usize);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position(usize, usize);

impl Position {
    fn all_four_neighbours(&self, bounds: Bounds) -> Vec<Position> {
        let mut result = vec![];

        if self.0 >= 1 {
            result.push(Position(
                self.0 - 1,
                self.1,
            ))
        }

        if self.0 <= bounds.0 - 2 {
            result.push(Position(
                self.0 + 1,
                self.1,
            ));
        }

        if self.1 >= 1 {
            result.push(Position(
                self.0,
                self.1 - 1,
            ))
        }

        if self.1 <= bounds.1 - 2 {
            result.push(Position(
                self.0,
                self.1 + 1,
            ));
        }

        return result;
    }
}

struct Map(Vec<Vec<usize>>);

impl Map {
    fn index(&self, position: &Position) -> usize {
        return self.0[position.0][position.1];
    }

    fn bounds(&self) -> Bounds {
        if self.0.is_empty() {
            panic!("Don't be stupid")
        }

        return (self.0.len(), self.0[0].len());
    }

    fn num_trails_from(&self, starting_position: &Position) -> usize {
        let mut valid_trails_backlog = VecDeque::new();

        let starting_trail = Trail(vec![starting_position.clone()]);

        valid_trails_backlog.push_back(starting_trail);

        let mut all_valid_trails = HashSet::<(Position, Position)>::new();

        while let Some(trail) = valid_trails_backlog.pop_front() {
            if self.index(trail.tail()) == 9 {
                all_valid_trails.insert((starting_position.clone(), trail.tail().clone()));

                continue;
            }

            for new_position in trail.possible_next_positions(self) {
                valid_trails_backlog.push_back(trail.extend(new_position));
            }
        }

        return all_valid_trails.len();
    }

    // Counts different paths from start to finish as unique trails rather than the same trail
    fn num_unique_trails_from(&self, starting_position: &Position) -> usize {
        let mut valid_trails_backlog = VecDeque::new();

        let starting_trail = Trail(vec![starting_position.clone()]);

        valid_trails_backlog.push_back(starting_trail);

        let mut count = 0;

        while let Some(trail) = valid_trails_backlog.pop_front() {
            if self.index(trail.tail()) == 9 {
                count += 1;

                continue;
            }

            for new_position in trail.possible_next_positions(self) {
                valid_trails_backlog.push_back(trail.extend(new_position));
            }
        }

        return count;
    }
}

#[derive(Debug, Clone)]
struct Trail(Vec<Position>);

impl Trail {
    fn tail(&self) -> &Position {
        return self.0.last().unwrap();
    }

    fn extend(&self, new_tail: Position) -> Self {
        let mut result = self.clone();

        result.0.push(new_tail);

        return result;
    }

    fn possible_next_positions(&self, map: &Map) -> Vec<Position> {
        let mut result = vec![];

        let current_value = map.index(self.tail());

        for new_position in self.tail().all_four_neighbours(map.bounds()) {
            if self.0.contains(&new_position) {
                continue;
            }

            if map.index(&new_position) == current_value + 1 {
                result.push(new_position);
            }
        }

        return result;
    }
}

fn parse_input(input: &str) -> Map {
    let mut result = vec![];

    for line in input.lines() {
        let mut current_line = vec![];

        for character in line.chars() {
            // For while I'm runnig the incomplete test inputs
            if character == '.' {
                current_line.push(usize::MAX);

                continue;
            }

            current_line.push(character.to_digit(10).unwrap() as usize);
        }

        result.push(current_line);
    }

    return Map(result);
}

fn part_1(input: &str) -> usize {
    let map = parse_input(input);

    // Maps a starting position to the number of hike trails from there
    let mut hike_trails = HashMap::<Position, usize>::new();

    for (i, row) in map.0.iter().enumerate() {
        for (j, digit) in row.iter().enumerate() {
            if *digit != 0 {
                continue;
            }

            let starting_position = Position(i, j);

            let num_trails = map.num_trails_from(&starting_position);

            hike_trails.insert(starting_position, num_trails);
        }
    }

    return hike_trails.values().sum();
}

fn part_2(input: &str) -> usize {
    let map = parse_input(input);

    // Maps a starting position to the number of hike trails from there
    let mut hike_trails = HashMap::<Position, usize>::new();

    for (i, row) in map.0.iter().enumerate() {
        for (j, digit) in row.iter().enumerate() {
            if *digit != 0 {
                continue;
            }

            let starting_position = Position(i, j);

            let num_trails = map.num_unique_trails_from(&starting_position);

            hike_trails.insert(starting_position, num_trails);
        }
    }

    return hike_trails.values().sum();
}

fn main() {
    let test_input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    let actual_input = &std::fs::read_to_string("src/bin/day-10/input.txt").unwrap();

    let result_1 = part_1(actual_input);

    println!("Part 1: {result_1}");

    let result_2 = part_2(actual_input);

    println!("Part 2: {result_2}");
}
