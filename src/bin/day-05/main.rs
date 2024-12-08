use std::collections::HashMap;

use itertools::Itertools;

// A rule is an `X` and all the `Y` that it has to come before
type Rules = HashMap<usize, Vec<usize>>;
#[derive(Debug)]
struct Input {
    rules: Rules,
    updates: Vec<Vec<usize>>,
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines().peekable();

    // Rules
    let mut rules = HashMap::new();

    while lines.peek() != Some(&"") {
        let current_line = lines.next().unwrap();

        let (left, right) = current_line.split_once('|').unwrap();
        let left = left.parse::<usize>().unwrap();
        let right = right.parse::<usize>().unwrap();

        rules.entry(left).and_modify(|vector: &mut Vec<usize>| vector.push(right)).or_insert(vec![right]);
    }

    // Skip the empty line
    lines.next();

    // Updates
    let mut updates = Vec::new();

    while let Some(current_line) = lines.next() {
        updates.push(
            current_line.split(',')
                .map(|element| element.parse::<usize>().unwrap())
                .collect()
        );
    }

    return Input {
        rules,
        updates,
    };
}

fn validate_update_forward(rules: &Rules, update: &[usize]) -> bool {
    for i in 1..update.len() {
        let current_page = update[i];

        let current_rules = rules.get(&current_page);

        // If there are no rules, they can't be violated
        if current_rules.is_none() {
            continue;
        }
        let current_rules = current_rules.unwrap();

        for other_page in update[i + 1..].iter() {
            // This assumes rules are exhaustive,
            // i.e. if there are any rules for current_page (`X`),
            // then there is a rule for current_page and any other_page (`Y`).
            //
            // That is to say, we have to check that there is *no* rule that says other_page has to come before
            // current_page, but what we're actually checking is that there *is* a rule that says other_page has
            // to come after current_page.
            if !current_rules.contains(other_page) {
                // If there is no such rule, the update is invalid, continue to the next one
                return false;
            }
        }
    };

    return true;
}

fn validate_update_backward(rules: &Rules, update: &[usize]) -> bool {
    // This also works and does not make the assumption in the above comment
    // Starting from the last and going to the first page in the update,
    // check that none of the pages before it have a rule that would make this ordering invalid
    for i in (1..update.len()).rev() {
        let current_page = update[i];

        let current_rules = rules.get(&current_page);

        // if there are no rules, they can't be violated either
        if current_rules.is_none() {
            continue;
        }
        let current_rules = current_rules.unwrap();

        for other_page in update[0..i].iter() {
            if current_rules.contains(other_page) {
                return false;
            }
        }
    }

    return true;
}

fn part_1(input: &str) -> usize {
    let input = parse_input(input);

    // The sum of middle page numbers
    let mut result = 0;

    for update in input.updates {
        let update_is_valid = validate_update_backward(&input.rules, &update);

        if update_is_valid {
            let current_mid = update[(update.len() - 1) / 2];

            result += current_mid;
        }
    }

    return result;
}

// See here, children
// The labours of a man who thought all the rules were mutually consistent
// They were not.
// (As in I had a rule 99|98 and 98|95 and 95|99)

// // Takes a set of rules and returns a vector that contains _all_ pages in an ordering that is valid
// fn make_correct_global_ordering(rules: &Rules) -> Vec<usize> {
//     let mut sorted_pages = vec![];

//     // Sorting so that results are stable for debugging
//     for (&X, Ys) in rules.iter().sorted_by_key(|(&page, _)| page) {
//         if X == 99 {
//             println!("Handling 99");

//             println!("Currently sorted pages: {sorted_pages:?}");

//             println!("Rules for 99: {Ys:?}");
//         }

//         if Ys.is_empty() {
//             panic!("If there are no rules for X, it shouldn't have been in the Rules hashmap");
//         }

//         // Find the rule (the `Y`) that is the most stringent,
//         // i.e. that itself as an `X` has the most stringent pages to come after it
//         // Don't worry, that comment makes sense to me
//         let Y_indices = Ys.iter()
//             .map(|Y| sorted_pages.iter().position(|page| page == Y).unwrap_or(usize::MAX))
//             .collect::<Vec<_>>();

//         if X == 99 {
//             println!("Y_indices for 99: {Y_indices:?}");
//         }

//         // Can unwrap because we check for Ys being empty above
//         let most_stringent_Y_idx = *Y_indices.iter().min().unwrap();

//         if X == 99 {
//             println!("Inserting at {most_stringent_Y_idx}");
//         }

//         if most_stringent_Y_idx == usize::MAX {
//             sorted_pages.push(X);

//             continue;
//         }

//         sorted_pages.insert(most_stringent_Y_idx, X);
//     }

//     if !validate_update_backward(rules, &sorted_pages) {
//         println!("Made correct global ordering but it wasn't actually correct:\n\t{sorted_pages:?}\n");
//     }

//     return sorted_pages;
// }

// // Takes a set of pages and reorders them by the correct global ordering
// fn make_correct_ordering(correct_ordering: &[usize], pages: &[usize]) -> Vec<usize> {
//     let mut ordered_pages = pages.to_vec();

//     ordered_pages.sort_unstable_by_key(|page| {
//         correct_ordering.iter()
//             .position(|correct_page| correct_page == page)
//             // If it's not in the global ordering, there was no rule for it, so it can come at the end
//             .unwrap_or(usize::MAX)
//     });

//     return ordered_pages;
// }

// If there is a pair that violates a rule, return Some((left_idx, right_idx))
// Otherwise, return None
fn find_invalid_pair(rules: &Rules, update: &[usize]) -> Option<(usize, usize)> {
    // println!("\tChecking {update:?}");

    for (i, right) in update.iter().enumerate().rev() {
        // println!("\t\tValidating {right}");
        for (j, left) in update[0..i].iter().enumerate() {
            // println!("\t\tValidating against {left}");
            // If there are rules for the current `right`,
            // and those rules demand that `left` comes before it,
            // that is an invalid pair
            if let Some(true) = rules.get(right).map(|rules| rules.contains(left)) {
                // println!("\tFound invalid pair ({i}, {j})");
                return Some((i, j));
            }
        }
    }

    // println!("\tAll good");

    return None;
}

fn part_2(input: &str) -> usize {
    let input = parse_input(input);

    let mut result = 0;

    for update in &input.updates {
        // Only invalid rules contribute to part 2
        if validate_update_backward(&input.rules, update) {
            continue;
        }

        let mut ordered_update = update.clone();

        while let Some((left_idx, right_idx)) = find_invalid_pair(&input.rules, &ordered_update) {
            ordered_update.swap(left_idx, right_idx);
        }

        // Sanity check
        if !validate_update_backward(&input.rules, &ordered_update) {
            panic!("Made a 'correct' ordering of\n{update:?}\n->\n{ordered_update:?},\nbut the result wasn't a valid ordering");
        }

        let current_mid = ordered_update[(update.len() - 1) / 2];

        result += current_mid;
    }

    return result;
}

fn main() {
    let test_input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    let actual_input = &std::fs::read_to_string("src/bin/day-05/input.txt").unwrap();

    let result_1 = part_1(actual_input);

    println!("Part 1: {result_1}");

    let result_2 = part_2(actual_input);

    println!("Part 2: {result_2}");
}
