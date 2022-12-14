use std::fmt::Display;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use fxhash::FxHashMap as HashMap;

// ========================= Challenge Logic ============================
// Define your own output type here for the `parse_input` function.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coord(i32, i32);

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(mut self, rhs: Coord) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign<Coord> for Coord {
    fn add_assign(&mut self, rhs: Coord) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(mut self, rhs: Coord) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign<Coord> for Coord {
    fn sub_assign(&mut self, rhs: Coord) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}


#[derive(Copy, Clone, Debug)]
pub enum State {
    Rock,
    Sand
}

type Board = HashMap<Coord, State>;
type ParsedInput = (Board, i32);

pub fn parse_input(input: &str) -> ParsedInput {
    let paths = input.lines().map(|l|
        l.split(" -> ")
        .map(|coord|
            match coord.split_once(",") {
                Some((x, y)) => Coord(x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()),
                None => panic!("Expected coord")
            }
        )
        .collect::<Vec<Coord>>()
    )
    .collect::<Vec<Vec<Coord>>>();

    // Construct Board
    let mut board: HashMap<Coord, State> = HashMap::default();
    board.reserve(1024);

    // Get y range of input & insert rock into board
    let mut max_y = 0;
    for path in paths {
        for coord in path.windows(2) {
            let (x1, y1) = (coord[0].0, coord[0].1);
            let (x2, y2) = (coord[1].0, coord[1].1);
            match (x1 - x2, y1 - y2) {
                (0, delta) if delta > 0 => {
                    for i in 0..=delta {
                        board.insert(Coord(x1, y2 + i), State::Rock);
                    }
                },
                (0, delta) if delta <= 0 => {
                    for i in 0..=delta.abs() {
                        board.insert(Coord(x1, y1 + i), State::Rock);
                    }
                },
                (delta, 0) if delta > 0 => {
                    for i in 0..=delta {
                        board.insert(Coord(x2 + i, y1), State::Rock);
                    }
                },
                (delta, 0) if delta <= 0 => {
                    for i in 0..=delta.abs() {
                        board.insert(Coord(x1 + i, y1), State::Rock);
                    }
                },
                (x, y) => panic!("Panic! Got ({},{})", x, y),
            }
            max_y = max_y.max(y1.max(y2));
        }
    }

    // Return board and max_y
    (board, max_y)
}

pub fn part1(input: &ParsedInput) -> impl Display {
    let (mut board, max_y) = (input.0.clone(), input.1);

    // Add a new sand particle
    let mut history: Vec<Coord> = Vec::with_capacity(1024);
    let mut sand = Coord(500, 0);

    let mut counter = 0;
    'falling: loop {
        // Increase sand particle counter
        counter += 1;

        // Let it fall down
        while sand.1 < max_y {
            // Compute bottom, bottom-left and bottom-right positions.
            let bottom = sand + Coord(0, 1);
            let left = sand + Coord(-1, 1);
            let right = sand + Coord(1, 1);

            // See what is in the bottom-left, bottom, and bottom-right positions.
            match (board.get(&left), board.get(&bottom), board.get(&right)) {
                (_, None, _) => {history.push(sand); sand = bottom}, // If there is nothing in the bottom position, continue there.
                (None, _, _) => {history.push(sand); sand = left},   // If there is nothing in the bottom-left position, continue there.
                (_, _, None) => {history.push(sand); sand = right},  // If there is nothing in the bottom-right position, continue there.
                (Some(_), Some(_), Some(_)) => { 
                    // If there is something in all positions, insert a sand particle at the current position
                    board.insert(sand, State::Sand);
                    sand = history.pop().unwrap();
                    // Drop in a new sand particle
                    continue 'falling;
                },
            }

            
        }

        // If the for-loop terminated successfully, it means the sand particle
        // went out of bounds. Therefore return counter
        return counter - 1;
    }
}

pub fn part2(input: &ParsedInput) -> impl Display {// Construct Board
    let (mut board, max_y) = (input.0.clone(), input.1);

    // Add a new sand particle
    let mut history: Vec<Coord> = Vec::with_capacity(1024);
    let mut sand = Coord(500, 0);

    let mut counter = 0;
    'falling: loop {
        // Increase sand particle counter
        counter += 1;
        
        // Check whether origin is still free
        if board.get(&sand).is_some() {
            return counter - 1;
        }

        // Let it fall down
        while sand.1 < max_y + 1 {
            // Compute bottom, bottom-left and bottom-right positions.
            let bottom = sand + Coord(0, 1);
            let left = sand + Coord(-1, 1);
            let right = sand + Coord(1, 1);

            // See what is in the bottom-left, bottom, and bottom-right positions.
            match (board.get(&left), board.get(&bottom), board.get(&right)) {
                (_, None, _) => {history.push(sand); sand = bottom}, // If there is nothing in the bottom position, continue there.
                (None, _, _) => {history.push(sand); sand = left},   // If there is nothing in the bottom-left position, continue there.
                (_, _, None) => {history.push(sand); sand = right},  // If there is nothing in the bottom-right position, continue there.
                (Some(_), Some(_), Some(_)) => { 
                    // If there is something in all positions, insert a sand particle at the current position
                    board.insert(sand, State::Sand);
                    sand = history.pop().unwrap_or(Coord(500, 0));

                    // Drop in a new sand particle
                    continue 'falling;
                },
            }
        }

        // If the for-loop terminated successfully, it means the sand particle
        // has landed on the floor, add it there
        board.insert(sand, State::Sand);
        sand = history.pop().unwrap_or(Coord(500, 0));
    }
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    // Parse sample and challenge input
    let sample = parse_input(&read_file("sample"));
    let input = parse_input(&read_file("input"));

    // Part 1
    // Define sample answer
    let sample_answer_part_1 = "24";

    // Evaluate sample input, and compare with sample answer
    assert_eq!(format!("{}", part1(&sample)), sample_answer_part_1);

    // If sample input evaluated correctly, print output of part 1 with 
    // challenge output.
    formatted_print("1", part1(&input));

    // Part 2
    // Define sample answer
    let sample_answer_part_2 = "93";

    // Evaluate sample input, and compare with sample answer
    assert_eq!(format!("{}", part2(&sample)), sample_answer_part_2);

    // If sample input evaluated correctly, print output of part 1 with 
    // challenge output.
    formatted_print("2", part2(&input));

}

pub fn read_file(file_name: &str) -> String {
    return std::fs::read_to_string(file_name).expect(format!("File {} not found", file_name).as_str());
}

fn formatted_print<T : Display>(part: &str, output: T) {
    println!("==================== Part {} ======================", part);
    println!("{}", output);
    println!("==================================================");
}