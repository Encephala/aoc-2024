use std::collections::HashSet;

type Position = (usize, usize);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Direction {
    RIGHT,
    DOWN,
    LEFT,
    UP,
}

impl Direction {
    fn next(&self) -> Self {
        use Direction::*;

        return match self {
            RIGHT => DOWN,
            DOWN => LEFT,
            LEFT => UP,
            UP => RIGHT,
        };
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Guard {
    position: Position,
    direction: Direction,
}

#[derive(Debug, Clone)]
struct Game {
    objects: Vec<Vec<bool>>,
    guard: Guard,
    previous_states: HashSet::<Guard>,
    is_done: bool,
}

// Returns Some(new_position) if moving from old in direction does not violate bounds
// Returns None if it does violate bounds
fn add_positions(old: &Position, direction: &Direction, bounds: &(usize, usize)) -> Option<Position> {
    let (delta_row, delta_column) = match direction {
        Direction::RIGHT => (0, 1),
        Direction::DOWN => (1, 0),
        Direction::LEFT => (0, -1),
        Direction::UP => (-1, 0),
    };

    let new_row = old.0 as isize + delta_row;
    let new_col = old.1 as isize + delta_column;

    if new_row < 0 || new_row >= bounds.0 as isize {
        return None;
    }

    if new_col < 0 || new_col >= bounds.1 as isize {
        return None;
    }

    return Some((new_row as usize, new_col as usize));
}

impl Game {
    fn bounds(&self) -> (usize, usize) {
        if self.objects.is_empty() {
            panic!("Called bounds but game wasn't initialised properly")
        }

        return (self.objects.len(), self.objects[0].len());
    }

    fn is_object(&self, position: &Position) -> bool {
        return self.objects[position.0][position.1];
    }

    // Returns true if the new state of the guard is in the previous_states
    fn move_guard(&mut self) -> bool {
        if let Some(new_position) = add_positions(&self.guard.position, &self.guard.direction, &self.bounds()) {
            if self.is_object(&new_position) {
                self.guard.direction = self.guard.direction.next();

                let same_as_previous_state = self.previous_states.contains(&self.guard);

                self.previous_states.insert(self.guard.clone());

                return same_as_previous_state;
            }

            self.guard.position = new_position;

            let same_as_previous_state = self.previous_states.contains(&self.guard);

            self.previous_states.insert(self.guard.clone());

            return same_as_previous_state;
        }

        self.is_done = true;

        // If new_position is None, guard exited the board, which can't be a previous state.
        return false;
    }
}

fn parse_input(input: &str) -> Game {
    let mut objects = vec![];

    let mut guard = None;

    for (i, line) in input.lines().enumerate() {
        let mut current_line = vec![];

        for (j, char) in line.chars().enumerate() {
            match char {
                '#' => current_line.push(true),
                '.' => current_line.push(false),
                '^' => {
                    current_line.push(false);

                    guard = Some(Guard { position: (i, j), direction: Direction::UP });
                }
                '>' => {
                    current_line.push(false);

                    guard = Some(Guard { position: (i, j), direction: Direction::RIGHT });
                }
                'v' => {
                    current_line.push(false);

                    guard = Some(Guard { position: (i, j), direction: Direction::DOWN });
                }
                '<' => {
                    current_line.push(false);

                    guard = Some(Guard { position: (i, j), direction: Direction::LEFT });
                }
                _ => panic!("Invalid character {char}"),
            }
        }

        objects.push(current_line);
    }

    let guard = guard.expect("No guard found while parsing");

    let mut previous_states = HashSet::new();

    previous_states.insert(guard.clone());

    return Game {
        objects,
        guard,
        previous_states,
        is_done: false,
    };
}

fn part_1(input: &str) -> usize {
    let mut game = parse_input(input);

    while !game.is_done {
        game.move_guard();
    }

    return game.previous_states.len();
}

fn part_2(input: &str) -> usize {
    // Play the game
    // Every move, check:
    // If putting an object in front of the guard at its current position and direction
    // would put it onto a path if has already been on,
    // that's a solution.
    // However, it's nontrivial to find out if she'd get put on a path that she's already been on -
    // you basically have to finish playing the game to check that.

    // How impossible would brute force be?
    // 16061 non-object spots in my input
    // Sounds doable tbh

    // It was pretty doable, went through 130 rows of the input at about 2 rows per second
    // Fuck the answer is wrong

    let game = parse_input(input);

    let mut count = 0;

    for i in 0..game.objects.len() {
        println!("Row {} / {}", i + 1, game.objects.len());

        for (j, is_object) in game.objects[i].iter().enumerate() {
            if *is_object {
                continue;
            }

            let mut test_game = game.clone();

            // Sanity check
            assert!(!test_game.objects[i][j]);

            // Insert an object at this [i][j]
            test_game.objects[i][j] = true;

            while !test_game.is_done {
                if test_game.move_guard() {
                    count += 1;

                    break;
                }
            }

            // Game is done, guard left the arena,
            // this was not a loop
        }
    }

    return count;
}

fn main() {
    let test_input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    let actual_input = &std::fs::read_to_string("src/bin/day-06/input.txt").unwrap();

    let result_1 = part_1(actual_input);

    println!("Part 1: {result_1}");

    let result_2 = part_2(actual_input);

    println!("Part 2: {result_2}");
}
