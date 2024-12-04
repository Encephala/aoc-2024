use std::collections::HashMap;

fn parse_numbers(input: String) -> (Vec<usize>, Vec<usize>) {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();

        if parts.len() != 2 {
            panic!("Length was not 2 but {}", parts.len());
        }

        left.push(parts[0].parse::<usize>().unwrap());
        right.push(parts[1].parse::<usize>().unwrap());
    }

    return (left, right);
}

fn part_1(input: String) -> usize {
    let (mut left, mut right) = parse_numbers(input);

    left.sort_unstable();
    right.sort_unstable();

    let cumulative_difference = left.iter().zip(right.iter()).map(|(left_el, right_el)| {
        left_el.abs_diff(*right_el)
    })
    .sum();

    return cumulative_difference;
}

fn part_2(input: String) -> usize {
    let (left, right) = parse_numbers(input);

    // Key is the digit, value is how many times that digit occurs in left/right number list
    let mut left_counts = HashMap::<usize, usize>::new();
    let mut right_counts = HashMap::<usize, usize>::new();

    for val in left {
        left_counts.entry(val)
            .and_modify(|entry| *entry += 1)
            .or_insert(1);
    }

    for val in right {
        right_counts.entry(val)
            .and_modify(|entry| *entry += 1)
            .or_insert(1);
    }

    let mut result = 0;
    for (element, left_count) in left_counts.into_iter() {
        let right_count = right_counts.get(&element).unwrap_or(&0);

        result += element * left_count * right_count;
    }

    return result;
}

fn main() {
    let test_input = "3   4
4   3
2   5
1   3
3   9
3   3".to_string();

    let actual_input = std::fs::read_to_string("src/bin/day-1/input.txt").unwrap();

    let result = part_1(actual_input.clone());
    println!("Part 1: {result}");

    let result = part_2(actual_input);
    println!("Part 2: {result}");
}
