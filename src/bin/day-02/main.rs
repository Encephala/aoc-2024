struct Report(Vec<usize>);

impl Report {
    fn with_ignored_index(&self, index: usize) -> Self {
        let mut new_levels = self.0.clone();

        new_levels.remove(index);

        return Report(new_levels);
    }

    fn is_valid(&self) -> bool {
        // Assuming no inputs of length 1
        let is_increasing = self.0[1] > self.0[0];

        for (current, next) in self.0.iter().zip(self.0.iter().skip(1)) {
            if (next > current) != is_increasing {
                return false;
            }

            if current.abs_diff(*next) == 0 || current.abs_diff(*next) > 3 {
                return false;
            }
        }

        return true;
    }

    fn has_max_1_fault(&self) -> bool {
        if self.is_valid() {
            return true;
        }

        for skipped_index in 0..self.0.len() {
            if self.with_ignored_index(skipped_index).is_valid() {
                return true;
            }
        }

        return false;
    }
}

fn part_1(input: String) -> usize {
    let reports = input.lines()
        .map(|line| {
            let levels = line.split_ascii_whitespace()
                .map(|digit| digit.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            Report(levels)
        })
        .collect::<Vec<_>>();

    return reports.into_iter().filter(Report::is_valid).count();
}

fn part_2(input: String) -> usize {
    let reports = input.lines()
        .map(|line| {
            let levels = line.split_ascii_whitespace()
                .map(|digit| digit.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            Report(levels)
        })
        .collect::<Vec<_>>();

    return reports.into_iter().filter(Report::has_max_1_fault).count();
}


fn main() {
    let test_input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9".to_string();

    let actual_input = std::fs::read_to_string("src/bin/day-2/input.txt").unwrap();

    let result_1 = part_1(actual_input.clone());

    println!("Part 1: {result_1}");

    let result_2 = part_2(actual_input);

    println!("Part 2: {result_2}");
}
