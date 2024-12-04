use itertools::Itertools;

struct Puzzle(Vec<Vec<char>>);

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for line in self.0.iter() {
            let mut line_result = String::new();

            for char in line {
                line_result.push(*char);
                line_result.push(' ');
            }

            line_result.pop();

            result.push_str(&line_result);
            result.push('\n');
        }

        result.pop();

        return f.write_str(&result);
    }
}

impl Puzzle {
    fn dimensions(&self) -> (usize, usize) {
        let rows = self.0.len();

        let cols = self.0[0].len();

        return (rows, cols);
    }

    fn get(&self, row: usize, col: usize) -> char {
        return self.0[row][col];
    }

    fn find_word(&self, row: usize, col: usize, word: &[char], direction: Direction) -> bool {
        // Base case (found)
        if word.is_empty() {
            return true;
        }

        let (num_rows, num_cols) = self.dimensions();

        let new_position = add_offset(row, col, num_rows, num_cols, direction);
        if new_position.is_none() {
            return false;
        }

        let (new_row, new_col) = new_position.unwrap();

        // Other base case (not found)
        if self.get(new_row, new_col) != word[0] {
            return false;
        }

        return self.find_word(new_row, new_col, &word[1..], direction);
    }
}

type Direction = (isize, isize);
const DIRECTIONS: [Direction; 8] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
];

fn parse_input(input: &str) -> Puzzle {
    return Puzzle(
        input.lines()
            .map(|line| line.chars().collect())
            .collect()
    );
}

fn add_offset(row: usize, col: usize, max_row: usize, max_col: usize, offset: Direction) -> Option<(usize, usize)> {
    // All my homes love integer types
    let new_row = row as isize + offset.0;
    if new_row < 0 || new_row >= max_row as isize {
        return None;
    }

    let new_col = col as isize + offset.1;
    if new_col < 0 || new_col >= max_col as isize {
        return None;
    }

    return Some((new_row as usize, new_col as usize));
}

fn part_1(input: &str) -> usize {
    let input = parse_input(input);

    let (rows, cols) = input.dimensions();

    let mut found_count = 0;

    for (row, col) in (0..rows).cartesian_product(0..cols) {
        if input.get(row, col) != 'X' {
            continue;
        }

        for direction in DIRECTIONS {
            if input.find_word(row, col, &['M', 'A', 'S'], direction) {
                found_count += 1;
            }
        }
    }

    return found_count;
}

impl Puzzle {
    fn char_at_offset_equals(&self, row: usize, col: usize, offset: Direction, character: char) -> bool {
        let (num_rows, num_cols) = self.dimensions();

        let position = add_offset(row, col, num_rows, num_cols, offset);
        if position.is_none() {
            return false;
        }
        let (row, col) = position.unwrap();

        return self.get(row, col) == character;
    }

    fn this_A_is_surrounded_by(&self, row: usize, col: usize, offsets: &Vec<Direction>) -> bool {
        assert_eq!(offsets.len(), 4); // Little sanity check

        return self.char_at_offset_equals(row, col, offsets[0], 'M')
            && self.char_at_offset_equals(row, col, offsets[1], 'M')
            && self.char_at_offset_equals(row, col, offsets[2], 'S')
            && self.char_at_offset_equals(row, col, offsets[3], 'S');
    }
}

fn part_2(input: &str) -> usize {
    let input = parse_input(input);

    let (rows, cols) = input.dimensions();

    let mut found_count = 0;

    // Only search 1 from input borders because the 'A' needs to be surrounded by letters
    for (row, col) in (1..rows - 1).cartesian_product(1..cols - 1) {
        if input.get(row, col) != 'A' {
            continue;
        }

        let mut offsets = vec![
            (1, 1),
            (1, -1),
            (-1, -1),
            (-1, 1),
        ];

        for i in 0..4 {
            offsets.rotate_right(i);

            if input.this_A_is_surrounded_by(row, col, &offsets) {
                found_count += 1;

                // Can only be X-MAS in one direction
                continue;
            }
        }
    }

    return found_count;
}

fn main() {
    let test_input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    let actual_input = &std::fs::read_to_string("src/bin/day-4/input.txt").unwrap();

    let result_1 = part_1(actual_input);

    println!("Part 1: {result_1}");

    let result_2 = part_2(actual_input);

    println!("Part 2: {result_2}");
}
