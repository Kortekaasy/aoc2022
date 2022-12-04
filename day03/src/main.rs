use std::fmt::Display;

// ========================= Challenge Logic ============================
// Define your own output type here for the `parse_input` function.
type ParsedInput = Vec<String>;

pub fn parse_input(input: &str) -> ParsedInput {
    input.lines().map(|l|l.into()).collect()
}


pub fn char_to_usize(input: char) -> usize {
    // Convert a char to an usize (for shifting) using the mapping specified in the challenge
    if ('a'..='z').contains(&input) {
        input as usize - 'a' as usize
    } else {
        (input as usize - 'A' as usize) + 26
    }
}

pub fn part1(input: &ParsedInput) -> impl Display {

    input.iter().map(|l| {
        // Compute mid-point of current entry
        let mid = l.len() / 2;
        
        // Take chars of first half, convert to a bit-position in an u64, and compute the logical OR of all bit-positions.
        let first = l.chars().take(mid).map(char_to_usize).map(|shift| 1_u64 << shift).reduce(|acc, e| {acc | e}).unwrap();
        // Take chars of second half, convert to a bit-position in an u64, and compute the logical OR of all bit-positions.
        let second = l.chars().skip(mid).map(char_to_usize).map(|shift| 1_u64 << shift).reduce(|acc, e| {acc | e}).unwrap();
        
        // The common bit of the two halves is then the logical AND of the two aggregations
        let common = first & second;

        // Compute score by taking the trailing zero's and adding 1
        common.trailing_zeros() + 1
    }).sum::<u32>()
}

pub fn part2(input: &ParsedInput) -> impl Display {

    // We step through the input in groups of 3 lines by using slice.windows(3), and stepping through the iterator with
    // steps of 3.
    input.windows(3).step_by(3).map(|w| {
        // Take chars of first line, convert to a bit-position in an u64, and compute the logical OR of all bit-positions.
        let first = w[0].chars().map(char_to_usize).map(|shift| 1_u64 << shift).reduce(|acc, e| {acc | e}).unwrap();
        // Take chars of second line, convert to a bit-position in an u64, and compute the logical OR of all bit-positions.
        let second = w[1].chars().map(char_to_usize).map(|shift| 1_u64 << shift).reduce(|acc, e| {acc | e}).unwrap();
        // Take chars of third line, convert to a bit-position in an u64, and compute the logical OR of all bit-positions.
        let third = w[2].chars().map(char_to_usize).map(|shift| 1_u64 << shift).reduce(|acc, e| {acc | e}).unwrap();
        
        // The common bit of the three lines is then the logical AND of the three aggregations
        let common = first & second & third;

        // Compute score by taking the trailing zero's and adding 1
        common.trailing_zeros() + 1
    }).sum::<u32>()
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    // Parse sample and challenge input
    let sample = parse_input(&read_file("sample"));
    let input = parse_input(&read_file("input"));

    // Part 1
    // Define sample answer
    let sample_answer_part_1 = "157";

    // Evaluate sample input, and compare with sample answer
    assert_eq!(format!("{}", part1(&sample)), sample_answer_part_1);

    // If sample input evaluated correctly, print output of part 1 with 
    // challenge output.
    formatted_print("1", part1(&input));

    // Part 2
    // Define sample answer
    let sample_answer_part_2 = "70";

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