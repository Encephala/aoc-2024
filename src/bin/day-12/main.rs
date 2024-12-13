use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Position(usize, usize);

#[derive(Debug)]
struct Region {
    character: char,
    positions: Vec<Position>,
}

struct Garden {
    bounds: (usize, usize),
    regions: Vec<Region>,
    raw: Vec<Vec<char>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Fence {
    row: usize,
    col: usize,
    is_horizontal: bool,
    // Not horizontal/vertical, but for horizontal it's up or down and for vertical it's left/right
    // orientation is true if 'outside' of fence is down or right, false if outside is up or left
    orientation: bool,
}

impl Fence {
    fn is_connecting_fence_horizontally(&self, other_fence: &Fence) -> bool {
        return self.row == other_fence.row
            && (self.col + 1 == other_fence.col || self.col == other_fence.col + 1);
    }

    fn is_connecting_fence_vertically(&self, other_fence: &Fence) -> bool {
        return self.col == other_fence.col
            && (self.row + 1 == other_fence.row || self.row == other_fence.row + 1);
    }

    fn belongs_to_side(&self, side: &[Fence]) -> bool {
        if self.is_horizontal != side.first().unwrap().is_horizontal {
            return false;
        }

        if self.orientation != side.first().unwrap().orientation {
            return false;
        }

        for fence in side {
            match self.is_horizontal {
                true => {
                    if self.is_connecting_fence_horizontally(fence) {
                        return true;
                    }
                },
                false => {
                    if self.is_connecting_fence_vertically(fence) {
                        return true;
                    }
                },
            }
        }

        return false;
    }
}

// This should just be an enum but eh, screw refactoring
const ALL_OFFSETS: [(isize, isize); 4] = [
    // Horizontal offsets
    (0, -1), // Left
    (0, 1), // Right
    // Vertical offsets
    (-1, 0), // Up
    (1, 0), // Down
];

impl Region {
    fn value(&self, bounds: &(usize, usize)) -> usize {
        return self.perimeter_size(bounds) * self.area();
    }

    fn perimeter_size(&self, bounds: &(usize, usize)) -> usize {
        let mut perimeter = 0;

        for position in &self.positions {
            let mut number_of_neighbours_that_are_in_same_region = 0;

            for neighbour in all_neighbours(position, bounds) {
                // If neighbour is in this region, we don't add a fence
                if self.positions.contains(&neighbour) {
                    number_of_neighbours_that_are_in_same_region += 1;
                }
            }

            // For each neighbour that is in the region, this position has one fewer fence
            perimeter += 4 - number_of_neighbours_that_are_in_same_region;
        }

        return perimeter;
    }

    fn area(&self) -> usize {
        return self.positions.len();
    }

    fn discount_value(&self, garden: &Garden) -> usize {
        return self.number_of_sides(garden) * self.area();
    }

    fn all_fences(&self, garden: &Garden) -> HashSet<Fence> {
        // (x, y) means a fence is above row x and on row y, so x runs from 0 to num_rows inclusive and y from 0 to num_cols exclusive
        // Likewise for vertical
        let mut fences = HashSet::<Fence>::new();
        // let mut vertical_fences = HashSet::<Fence>::new();

        for position in &self.positions {
            for horizontal_direction in &ALL_OFFSETS[..2] {
                let position_plus_offset = add_offset(position, horizontal_direction);

                let potential_fence_to_insert = if horizontal_direction.1 == -1 {
                    Fence {
                        // +1 will make it always >= 0 so can cast to usize np
                        // +1 because of the way we enumerate fences
                        row: position_plus_offset.0 as usize,
                        col: (position_plus_offset.1 + 1) as usize,
                        is_horizontal: false,
                        orientation: false,
                    }
                } else {
                    Fence {
                        row: position_plus_offset.0 as usize,
                        col: position_plus_offset.1 as usize,
                        is_horizontal: false,
                        orientation: true,
                    }
                };

                // If out of bounds, always add a fence
                // Don't have to check row value because it's a horizontal offset
                if position_plus_offset.1 < 0 || position_plus_offset.1 >= garden.bounds.1 as isize {
                    fences.insert(potential_fence_to_insert);

                    continue;
                }

                // Man rust is amazing
                if let Some(character) = garden.raw.get(position_plus_offset.0 as usize)
                    .and_then(|row| row.get(position_plus_offset.1 as usize))
                {
                    // Same region, no fence here
                    if *character == self.character {
                        continue;
                    }
                }

                // In that direction there is either nothing or a different region, so we put a fence
                fences.insert(potential_fence_to_insert);
            }

            // You ever heard of the DRY principle? me neither
            for vertical_direction in &ALL_OFFSETS[2..] {
                let position_plus_offset = add_offset(position, vertical_direction);

                let potential_fence_to_insert = if vertical_direction.0 == -1 {
                    Fence {
                        // +1 will make it always >= 0 so can cast to usize np
                        // +1 because of the way we enumerate fences
                        row: (position_plus_offset.0 + 1) as usize,
                        col: position_plus_offset.1 as usize,
                        is_horizontal: true,
                        orientation: false,
                    }
                } else {
                    Fence {
                        row: position_plus_offset.0 as usize,
                        col: position_plus_offset.1 as usize,
                        is_horizontal: true,
                        orientation: true,
                    }
                };

                // If out of bounds, always add a fence
                // Don't have to check row value because it's a horizontal offset
                if position_plus_offset.0 < 0 || position_plus_offset.0 >= garden.bounds.0 as isize {
                    fences.insert(potential_fence_to_insert);

                    continue;
                }

                // Man rust is amazing
                if let Some(character) = garden.raw.get(position_plus_offset.0 as usize)
                    .and_then(|row| row.get(position_plus_offset.1 as usize))
                {
                    // Same region, no fence here
                    if *character == self.character {
                        continue;
                    }
                }

                // In that direction there is either nothing or a different region, so we put a fence
                fences.insert(potential_fence_to_insert);
            }
        }

        return fences;
    }

    fn number_of_sides(&self, garden: &Garden) -> usize {
        let fences = self.all_fences(garden);

        let mut fences = fences.into_iter().collect::<Vec<_>>();

        let mut total_sides = 0;

        while let Some(starting_fence) = fences.pop() {
            // Fences that potentially belong to the same side
            let mut other_fences = fences.clone()
                .into_iter()
                .filter(|other_fence| other_fence.is_horizontal == starting_fence.is_horizontal && other_fence.orientation == starting_fence.orientation)
                .collect::<Vec<_>>();

            let mut current_side = vec![starting_fence];

            while !other_fences.is_empty() {
                let mut found_neighbour_in_current_side = false;

                // Check each of the potential fences to see if it belongs to the current side
                for other_fence in &other_fences {
                    if other_fence.belongs_to_side(&current_side) {
                        current_side.push(other_fence.clone());

                        found_neighbour_in_current_side = true;

                        break;
                    }
                }

                // If no new fence belongs to the current side, it hasn't changed and we're done searching
                if !found_neighbour_in_current_side {
                    break;
                }

                remove_fences_in_current_side(&mut other_fences, &current_side);
            }

            remove_fences_in_current_side(&mut fences, &current_side);

            total_sides += 1;
        }

        return total_sides;
    }
}

fn remove_fences_in_current_side(all_fences: &mut Vec<Fence>, current_side: &[Fence]) {
    // Remove all fences in current side to prevent double counting
    // Iterate in reverse because ykno index out of bounds and stuff otherwise,
    // i.e. elements >i get shifted towards 0 when element i gets removed
    for i in (0..all_fences.len()).rev() {
        if current_side.contains(&all_fences[i]) {
            all_fences.remove(i);
        }
    }
}

fn add_offset(position: &Position, offset: &(isize, isize)) -> (isize, isize) {
    let new_row = position.0 as isize + offset.0;
    let new_col = position.1 as isize + offset.1;

    return (new_row, new_col);
}

fn all_neighbours(position: &Position, bounds: &(usize, usize)) -> Vec<Position> {
    let mut result = vec![];

    if position.0 > 0 {
        result.push(Position(position.0 - 1, position.1))
    }

    if position.0 < bounds.0 - 1 {
        result.push(Position(position.0 + 1, position.1));
    }

    if position.1 > 0 {
        result.push(Position(position.0, position.1 - 1))
    }

    if position.1 < bounds.1 - 1 {
        result.push(Position(position.0, position.1 + 1));
    }

    return result;
}

fn construct_region(region: &mut Region, starting_position: Position, map: &Vec<Vec<char>>, bounds: &(usize, usize)) {
    region.positions.push(starting_position.clone());

    for neighbour in all_neighbours(&starting_position, bounds) {
        // If different region
        if map[neighbour.0][neighbour.1] != region.character {
            continue;
        }

        // If this region already contains this neighbour
        if region.positions.contains(&neighbour) {
            continue;
        }

        construct_region(region, neighbour, map, bounds);
    }
}

fn parse_input(input: &str) -> Garden {
    // Gotta love onelines am I right
    let map = input.lines()
        .map(|line| line.chars().collect::<Vec<_>>() )
        .collect::<Vec<_>>();

    let num_rows = map.len();
    let num_cols = map[0].len(); // Assumes map (i.e. input) nonempty

    let mut positions_to_check: Vec<Position> = (0..num_rows).cartesian_product(0..num_cols)
        .map(|(i, j)| Position(i, j))
        .collect();

    let mut regions = vec![];

    // While there is an unprocessed region left
    // The Some(position) is single position whose entire region will be handled in the iteration
    while let Some(position) = positions_to_check.first() {
        let character = map[position.0][position.1];

        let mut region = Region {
            character,
            positions: vec![],
        };

        construct_region(&mut region, position.clone(), &map, &(num_rows, num_cols));

        // Remove the regions area from the positions to check
        // Very inefficient, much nice
        for position_in_region in region.positions.iter() {
            if let Some(index) = positions_to_check.iter().position(|position_to_check| position_to_check == position_in_region) {
                positions_to_check.remove(index);

                continue;
            }

            // This shouldn't happen, a region can't contain a position that is already part of another region
            panic!("Don't be stupid");
        }

        regions.push(region);
    }

    return Garden {
        bounds: (num_rows, num_cols),
        regions,
        raw: map,
    };
}

fn part_1(input: &str) -> usize {
    let garden = parse_input(input);

    return garden.regions.iter()
        .map(|region| region.value(&garden.bounds))
        .sum();
}

fn part_2(input: &str) -> usize {
    let garden = parse_input(input);

    return garden.regions.iter()
        .map(|region| region.discount_value(&garden))
        .sum();
}

fn main() {
    let test_input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    let actual_input = &std::fs::read_to_string("src/bin/day-12/input.txt").unwrap();

    let result_1 = part_1(actual_input);

    println!("Part 1: {result_1}");

    let result_2 = part_2(actual_input);

    println!("Part 2: {result_2}");
}
