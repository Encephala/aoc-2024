#[derive(Debug)]
struct Equation (
    usize,
    Vec<usize>,
);

fn parse_input(input: &str) -> Vec<Equation> {
    let mut result = vec![];

    for line in input.lines() {
        let (outcome, numbers) = line.split_once(':').expect("line doesn't have a colon ya dingus");

        let outcome = outcome.parse::<usize>().unwrap();

        let numbers = numbers.split_ascii_whitespace()
            .map(|number| number.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        result.push(Equation(outcome, numbers));
    }

    return result;
}

fn equation_can_be_valid(expected_result: usize, numbers: Vec<usize>) -> bool {
    let number_of_operators = (numbers.len() - 1) as u32;

    // Mask is a binary representation of whether to choose '+' (0) or '*' (1)
    for mask in 0..(2u32.pow(number_of_operators)) {
        let mut result = numbers[0];

        for (i, number) in numbers[1..].iter().enumerate() {
            // get i-th bit from mask, starting with least significant bit
            let operator_is_multiply = mask >> i & 1;

            if operator_is_multiply == 1 {
                result *= number;
            } else {
                result += number;
            }
        }

        if result == expected_result {
            return true;
        }
    }

    return false;
}

fn part_1(input: &str) -> usize {
    let equations = parse_input(input);

    let mut sum_of_results = 0;

    for Equation(expected_result, numbers) in equations {
        if equation_can_be_valid(expected_result, numbers) {
            sum_of_results += expected_result;
        }
    }

    return sum_of_results;
}

fn all_possible_concatenations(numbers: &[usize]) -> Vec<Vec<usize>> {
    println!("Finding all concatenations in {numbers:?}");

    // Base cases
    if numbers.is_empty() {
        return vec![];
    }

    if numbers.len() == 1 {
        return vec![numbers.to_vec()];
    }

    let mut result = vec![];

    // TODO: This isn't right
    // Just concatenate the first two numbers or don't,
    // Then recurse and append these vectors
    for i in 1..numbers.len() - 1 {
        dbg!(i);
        let head = numbers[..i].to_vec();

        let mut head_no_concat = head.clone();

        head_no_concat.extend(&numbers[i..i + 2]);

        let mut head_yes_concat = head.clone();

        head_yes_concat.push({
            let left = numbers[i];
            let right = numbers[i + 1];

            let concatenated = left.to_string() + &right.to_string();

            concatenated.parse::<usize>().unwrap()
        });

        let all_tails = all_possible_concatenations(&numbers[i + 2..]);

        for mut tail in all_tails {
            let mut new_possibility_no_concat = head_no_concat.clone();
            new_possibility_no_concat.append(&mut tail);

            result.push(new_possibility_no_concat);

            let mut new_possibility_yes_concat = head_no_concat.clone();
            new_possibility_yes_concat.append(&mut tail);

            result.push(new_possibility_yes_concat);
        }
    }

    println!("Found combinations: {result:?}");

    return result;
}

fn insert_concatenation(numbers: &[usize], at_index: usize) -> Vec<usize> {
    let mut result = vec![];

    result.append(&mut numbers[..at_index].to_vec());

    let concatenated_numbers = {
        let left = numbers[at_index];
        let right = numbers[at_index + 1];

        let concatenated = left.to_string() + &right.to_string();

        concatenated.parse::<usize>().unwrap()
    };

    result.push(concatenated_numbers);

    result.append(&mut numbers[at_index + 2..].to_vec());

    return result;
}

fn part_2(input: &str) -> usize {
    let equations = parse_input(input);

    let mut sum_of_results = 0;

    for Equation(expected_result, numbers) in equations {
        // If the equation is valid, early `continue` the loop
        // Else, insert concatenations and check the result

        // Actually, the first case is a specific case of the second case,
        // as in if we're looping through all possible combinations of inserting concatenations,
        // not inserting any is just one of those combinations

        for combination in all_possible_concatenations(&numbers) {
            if equation_can_be_valid(expected_result, combination) {
                sum_of_results += expected_result;

                break;
            }
        }
    }

    return sum_of_results;
}

fn main() {
    let test_input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    let actual_input = &std::fs::read_to_string("src/bin/day-07/input.txt").unwrap();

    let result_1 = part_1(actual_input);

    println!("Part 1: {result_1}");

    let test_numbers = [0, 1, 2, 3, 4];

    dbg!(&all_possible_concatenations(&test_numbers));
}
