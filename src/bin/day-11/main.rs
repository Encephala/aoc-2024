use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<usize> {
    return input.split_ascii_whitespace().map(|number| number.parse().unwrap()).collect();
}

fn blink(state: &[usize]) -> Vec<usize> {
    let mut result = vec![];

    for number in state {
        match *number {
            0 => result.push(1),
            n if n.to_string().len() % 2 == 0 => {
                let n = n.to_string();

                let (left, right) = n.split_at(n.len() / 2);

                result.push(left.parse().unwrap());
                result.push(right.parse().unwrap());
            },
            other => result.push(other * 2024),
        }
    }

    return result;
}

fn part_1(input: &str) -> usize {
    let mut rocks = parse_input(input);

    for _ in 0..25 {
        rocks = blink(&rocks);
    }

    return rocks.len();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Rock(usize);

impl Rock {
    fn blink(&self) -> Vec<Rock> {
        return match self.0 {
            0 => vec![Rock(1)],
            n if n.to_string().len() % 2 == 0 => {
                let n = n.to_string();

                let (left, right) = n.split_at(n.len() / 2);

                vec![Rock(left.parse().unwrap()), Rock(right.parse().unwrap())]
            },
            other => vec![Rock(other * 2024)],
        };
    }

    fn total_descendant_rocks(&self, num_blinks: usize, all_rocks: &mut HashMap<(Rock, usize), usize>) -> usize {
        // Base case 1: no blinks remaining
        if num_blinks == 0 {
            return 1;
        }

        let mut total_descendants = 0;

        // General case: value is the sum of the values of the descendant rocks after one blink
        for rock in self.blink() {
            // Shortcut: we've already processed this rock at this depth
            if let Some(descendant_count) = all_rocks.get(&(rock, num_blinks - 1)) {
                total_descendants += descendant_count;

                continue;
            }

            let descendant_count = rock.total_descendant_rocks(num_blinks - 1, all_rocks);

            total_descendants += descendant_count;

            // num_blinks == 0 is the base case anyways, only benefit in inserting it is one function call
            // But then for debugging a cleaner output is nicer
            if num_blinks - 1 != 0 {
                all_rocks.insert((rock, num_blinks - 1), descendant_count);
            }
        }

        return total_descendants;
    }
}

fn part_2(input: &str) -> usize {
    let rocks = parse_input(input);

    let mut all_rocks = HashMap::new();
    let mut total_count = 0;

    for rock in rocks {
        total_count += Rock(rock).total_descendant_rocks(75, &mut all_rocks)
    }

    return total_count;
}

fn main() {
    let test_input = "125 17";

    let actual_input = &std::fs::read_to_string("src/bin/day-11/input.txt").unwrap();

    let result_1 = part_1(actual_input);

    println!("Part 1: {result_1}");

    let result_2 = part_2(actual_input);

    println!("Part 2: {result_2}");
}
