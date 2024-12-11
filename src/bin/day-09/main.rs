const EMPTY_SPACE: usize = usize::MAX;

struct Disk(Vec<usize>);

impl Disk {
    // Going to assume there will always be some empty space and some non-empty blocks
    fn first_empty_block(&self) -> usize {
        return self.0.iter().position(|block| *block == EMPTY_SPACE).unwrap();
    }

    fn last_nonempty_block(&self) -> usize {
        return self.0.len() - 1 - self.0.iter().rev().position(|block| *block != EMPTY_SPACE).unwrap();
    }

    fn compress(&mut self) {
        loop {
            let first_empty = self.first_empty_block();
            let last_nonempty = self.last_nonempty_block();

            if first_empty > last_nonempty {
                break;
            }

            self.0.swap(first_empty, last_nonempty);
        }
    }

    fn checksum(&self) -> usize {
        let mut sum = 0;

        for (i, block_content) in self.0.iter().enumerate() {
            if *block_content != EMPTY_SPACE {
                sum += i * block_content;
            }
        }

        return sum;
    }

    fn empty_spaces(&self) -> Vec<(usize, usize)> {
        let mut result = vec![];

        let mut currently_on_empty_space = false;
        let mut current_start_idx = 0;
        let mut current_length = 0;

        for (i, block_value) in self.0.iter().enumerate() {
            if *block_value != EMPTY_SPACE {
                // If going from empty to full
                if currently_on_empty_space {
                    result.push((current_start_idx, current_length));

                    currently_on_empty_space = false;
                    current_length = 0;
                }

                continue;
            }

            // If going from full blocks to empty
            if !currently_on_empty_space {
                current_start_idx = i;
                currently_on_empty_space = true;
            }

            current_length += 1;
        }

        return result;
    }

    // Oh boy this isn't pretty
    fn compress_no_fragmentation(&mut self) {
        // Index to loop over the blocks, starting from the end
        let mut index = self.0.len() - 1;
        let mut handled_files = vec![];

        loop {
            if self.0[index] == EMPTY_SPACE || handled_files.contains(&self.0[index]) {
                if index == 0 {
                    break;
                }

                index -= 1;

                continue;
            }

            // Find current file size
            let current_file = self.0[index];
            let current_file_length = self.0[..=index].iter().rev().take_while(|block| **block == current_file).count();

            // Move to first empty space that fits it
            for (start_idx, length) in self.empty_spaces() {
                // Can't move it forward anymore
                if start_idx >= index {
                    break;
                }

                if length >= current_file_length {
                    for i in 0..current_file_length {
                        self.0.swap(start_idx + i, index - i);
                    }

                    break;
                }
            }

            handled_files.push(current_file);

            if index < current_file_length {
                break;
            }

            index -= current_file_length;
        }
    }
}

fn make_disk(input: &str) -> Disk {
    let mut result = vec![];

    let mut currently_on_a_file = true;
    let mut current_file_index = 0;

    for char in input.chars() {
        if char == '\n' {
            continue;
        }

        let digit = char.to_digit(10).unwrap();

        if currently_on_a_file {
            for _ in 0..digit {
                result.push(current_file_index);
            }

            current_file_index += 1;
        } else {
            for _ in 0..digit {
                result.push(EMPTY_SPACE);
            }
        }

        // Alternatingly, the disk map contains a file and a length of empty space
        currently_on_a_file = !currently_on_a_file;
    }

    return Disk(result);
}

fn part_1(input: &str) -> usize {
    let mut disk = make_disk(input);

    disk.compress();

    return disk.checksum();
}

fn part_2(input: &str) -> usize {
    let mut disk = make_disk(input);

    disk.compress_no_fragmentation();

    return disk.checksum();
}

fn main() {
    let test_input = "2333133121414131402";

    let actual_input = &std::fs::read_to_string("src/bin/day-09/input.txt").unwrap();

    let result_1 = part_1(actual_input);

    println!("Part 1: {result_1}");

    let result_2 = part_2(actual_input);

    println!("Part 2: {result_2}");
}
