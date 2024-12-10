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

fn equation_can_be_valid(expected_result: usize, numbers: &[usize]) -> bool {
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
        if equation_can_be_valid(expected_result, &numbers) {
            sum_of_results += expected_result;
        }
    }

    return sum_of_results;
}

fn concatenate(left: usize, right: usize) -> usize {
    let concatenated = left.to_string() + &right.to_string();

    return concatenated.parse::<usize>().unwrap();
}

// I thought concatenation was always done as pre-processing,
// but it's not.
// 6 * 8 || 6 * 15 = 68 || 6 * 15 = 686 * 15 = 7290, valid in the test input
// I interpreted that as 6 * 86 * 15 = 7740, invalid in the test input
// fn all_possible_concatenations(numbers: &[usize]) -> Vec<Vec<usize>> {
//     // println!("Finding all concatenations in {numbers:?}");

//     // Base cases
//     if numbers.is_empty() {
//         // println!("Input empty, no combinations found");
//         return vec![vec![]];
//     }

//     if numbers.len() == 1 {
//         // println!("Input single digit, found {:?}", vec![vec![numbers[0]]]);
//         return vec![vec![numbers[0]]];
//     }

//     let mut result = vec![];

//     // General case 1: no concatenation
//     let head_no_concat = vec![numbers[0]];

//     let all_tails_no_concat = all_possible_concatenations(&numbers[1..]);

//     for tail in all_tails_no_concat {
//         let mut new_possibility = head_no_concat.clone();
//         new_possibility.extend(tail);

//         result.push(new_possibility);
//     }

//     // General case 2: concatenation
//     let head_yes_concat = vec![concatenate(numbers[0], numbers[1])];

//     let all_tails_yes_concat = all_possible_concatenations(&numbers[2..]);

//     for tail in all_tails_yes_concat {
//         let mut new_possibility = head_yes_concat.clone();
//         new_possibility.extend(tail);

//         result.push(new_possibility);
//     }

//     // println!("Found combinations: {result:?}");

//     return result;
// }

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

fn extend_vector_returning_value(vector: Vec<Operator>, extension: &[Operator]) -> Vec<Operator> {
    let mut result = vector;

    result.extend(extension);

    return result;
}

fn build_all_operator_sequences(length: usize) -> Vec<Vec<Operator>> {
    use Operator::*;

    let mut result = vec![];

    // Base case
    if length == 0 {
        return vec![vec![]];
    }

    let add_head = vec![Add];
    let multiply_head = vec![Multiply];
    let concatenate_head = vec![Concatenate];

    // General case
    let tails = build_all_operator_sequences(length - 1);

    for tail in &tails {
        result.push(extend_vector_returning_value(add_head.clone(), &tail));
        result.push(extend_vector_returning_value(multiply_head.clone(), &tail));
        result.push(extend_vector_returning_value(concatenate_head.clone(), &tail));
    }

    return result;
}

fn part_2(input: &str) -> usize {
    let equations = parse_input(input);

    let mut sum_of_results = 0;

    for Equation(expected_result, numbers) in equations {
        // Brute force solution but fuck it
        for operator_sequence in build_all_operator_sequences(numbers.len() - 1) {
            let mut result = numbers[0];

            for (i, operator) in operator_sequence.iter().enumerate() {
                match operator {
                    Operator::Add => { result += numbers[i + 1]; },
                    Operator::Multiply => { result *= numbers[i + 1]; },
                    Operator::Concatenate => {
                        result = concatenate(result, numbers[i + 1])
                    },
                }
            }

            if result == expected_result {
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

    let result_2 = part_2(actual_input);

    println!("Part 2: {result_2}");
}
