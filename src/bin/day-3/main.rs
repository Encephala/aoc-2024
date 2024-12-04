fn parse_one_to_three_digit_integer(input: &str, i: &mut usize) -> Option<usize> {
    let mut result = None;

    if *i >= input.len() {
        return result;
    }

    if input[*i..].len() < 3 {}
    else if let Ok(number) = input[*i..*i + 3].parse::<usize>() {
        *i += 3;

        result = Some(number);
    }

    if input[*i..].len() < 2 {}
    else if let Ok(number) = input[*i..*i + 2].parse::<usize>() {
        *i += 2;

        result = Some(number);
    }

    if input[*i..].is_empty() {}
    else if let Ok(number) = input[*i..*i + 1].parse::<usize>() {
        *i += 1;

        result = Some(number);
    }

    return result;
}

fn parse_mul(input: &str, mut i: usize) -> Option<usize> {
    if input.get(i..=i+3)? != "mul(" {
        return None;
    }
    i += 4;

    let left = parse_one_to_three_digit_integer(input, &mut i)?;

    if input.get(i..i+1)? != "," {
        return None;
    }
    i += 1;

    let right = parse_one_to_three_digit_integer(input, &mut i)?;

    if input.get(i..i+1)? != ")" {
        return None;
    }
    i += 1;

    return Some(left * right);
}

fn part_1(input: String) -> usize {
    let mut result = 0;

    for i in 0..input.len() {
        if let Some(product) = parse_mul(&input, i) {
            result += product;
        }
    }

    return result;
}

fn part_2(input: String) -> usize {
    let mut result = 0;

    let mut currently_on_do = true;

    for i in 0..input.len() {
        if let Some("do()") = input.get(i..i+4) {
            // println!("Now on DO due to {}", &input[i..]);
            currently_on_do = true;
            continue;
        }

        if let Some("don't()") = input.get(i..i+7) {
            // println!("Now on DON'T due to {}", &input[i..]);
            currently_on_do = false;
            continue;
        }

        if !currently_on_do {
            continue;
        }

        // println!("Currently on DO, processing {}", &input[i..]);

        if let Some(product) = parse_mul(&input, i) {
            // println!("\tFound {product}");
            result += product;
        }
    }

    return result;
}

fn main() {
    let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();

    let actual_input = std::fs::read_to_string("src/bin/day-3/input.txt").unwrap();

    let result_1 = part_1(actual_input.clone());

    println!("Part 1: {result_1}");

    let test_input_2 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();

    let result_2 = part_2(actual_input);

    println!("Part 2: {result_2}");
}
