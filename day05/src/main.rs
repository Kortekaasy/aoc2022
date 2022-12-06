use std::fmt::Display;

// ========================= Challenge Logic ============================
// Define your own output type here for the `parse_input` function.
type Crates = Vec<Vec<char>>;
type Moves = Vec<(usize, usize, usize)>;
type ParsedInput = (Crates, Moves);

pub fn parse_input(input: &str) -> ParsedInput {
    // Split input in initial crate config and move list
    let (crates, moves) = input.split_once("\n\n").expect("Invalid input structure");
    
    // Compute the number of stacks in the input
    let num_stacks = (crates.find('\n').unwrap() + 1) / 4;
    
    // Create a data structure to hold the values from the input
    let mut stacks: Crates = vec![Vec::with_capacity(50); num_stacks];
    
    // Loop through all lines of the `crate` part of the challenge in reverse,
    // skipping the first line with the numbers
    crates.lines().rev().skip(1).map(|l| {
        // Grab all characters of the crate contents
        l.chars().skip(1).step_by(4).collect::<Vec<char>>()
    })
    .for_each(|row| 
        // Insert the values of the rows into the stacks
        stacks.iter_mut().zip(row.iter())
        // Make sure to filter out the 'empty' values.
        .filter(|(_, &e)| e != ' ')
        .for_each(|(stack, &e)| 
            stack.push(e)
        )
    );

    let parsed_moves = moves.lines()
    .map(|l| l.split(" "))
    .map(|mut split| (
        split.nth(1).expect("amnt").parse::<usize>().unwrap(),
        split.nth(1).expect("from").parse::<usize>().unwrap() - 1,
        split.nth(1).expect("to").parse::<usize>().unwrap() - 1,
    ))
    .collect::<Moves>();

    (stacks, parsed_moves)
}

pub fn part1(input: &ParsedInput) -> impl Display {
    // Clone input to get mutable `stacks` access
    let (mut stacks, moves) = input.clone();
    
    // Go through all moves and perform them
    for (amount, from, to) in moves {
        for _ in 0..amount {
            let popped = stacks[from].pop().unwrap();
            stacks[to].push(popped);
        }
    }

    // Pop an item from each stack, and combine them to the answer
    stacks.iter_mut()
    .map(|stack| stack.pop().expect("expected char"))
    .collect::<String>()
}

pub fn part2(input: &ParsedInput) -> impl Display {
    // Clone input to get mutable `stacks` access
    let (mut stacks, moves) = input.clone();
    
    // Go through all moves and perform them
    let mut popped: Vec<char> = Vec::with_capacity(50);
    for (amount, from, to) in moves {
        for _ in 0..amount {
            popped.push(stacks[from].pop().unwrap());
        }
        for pop in popped.drain(..).rev() {

            stacks[to].push(pop);
        }
    }

    // Pop an item from each stack, and combine them to the answer
    stacks.iter_mut()
    .map(|stack| stack.pop().expect("expected char"))
    .collect::<String>()
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    // Parse sample and challenge input
    let sample = parse_input(&read_file("sample"));
    let input = parse_input(&read_file("input"));

    // Part 1
    // Define sample answer
    let sample_answer_part_1 = "CMZ";

    // Evaluate sample input, and compare with sample answer
    assert_eq!(format!("{}", part1(&sample)), sample_answer_part_1);

    // If sample input evaluated correctly, print output of part 1 with 
    // challenge output.
    formatted_print("1", part1(&input));

    // Part 2
    // Define sample answer
    let sample_answer_part_2 = "MCD";

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