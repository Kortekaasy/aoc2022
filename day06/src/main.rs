use std::fmt::Display;

// ========================= Challenge Logic ============================
// Define your own output type here for the `parse_input` function.
type ParsedInput = Vec<u32>;

#[inline(always)]
pub fn char_to_shift(c: &u8) -> usize {
    // Convert a ascii character (u8) to a bitshift where 'a' = 0 and 'z' = 25
    (*c - b'a') as usize
}

pub fn parse_input(input: &str) -> ParsedInput {
    // Take input as byte, iterate over them
    input.as_bytes().iter()
    // Map each character to a bitshift
    .map(char_to_shift)
    // Leftshift 0x00000001 by the calculated bitshift
    .map(|shift| 1_u32 << shift)
    // Collect the shifted values
    .collect()
}

pub fn part1(input: &ParsedInput) -> impl Display {
    // Set size of window as variable
    let window_size: u32 = 4;

    // Go over the input in windows of `window_size`
    let (i, _ ) = input.windows(window_size as usize)
    // For each window, calculate the xor of all bitshifted values in the window
    .map(|window| window.iter().fold( 0, |acc, &e| acc ^ e))
    // Get the index for each entry
    .enumerate()
    // Find the first entry the number of ones is equal to `window_size`
    .find(|&(_i, e)| e.count_ones() == window_size)
    .expect("find");

    // Because we went over the input in windows, we need to add `window_size`
    // to get to the right answer
    i + window_size as usize
}

pub fn part2(input: &ParsedInput) -> impl Display {
    // Set size of window as variable
    let window_size: u32 = 14;

    // Go over the input in windows of `window_size`
    let (i, _ ) = input.windows(window_size as usize)
    // For each window, calculate the xor of all bitshifted values in the window
    .map(|window| window.iter().fold( 0, |acc, &e| acc ^ e))
    // Get the index for each entry
    .enumerate()
    // Find the first entry the number of ones is equal to `window_size`
    .find(|&(_i, e)| e.count_ones() == window_size)
    .expect("find");

    // Because we went over the input in windows, we need to add `window_size`
    // to get to the right answer
    i + window_size as usize
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    // Parse sample and challenge input
    let sample = parse_input(&read_file("sample"));
    let input = parse_input(&read_file("input"));

    // Part 1
    // Define sample answer
    let sample_answer_part_1 = "11";

    // Evaluate sample input, and compare with sample answer
    assert_eq!(format!("{}", part1(&sample)), sample_answer_part_1);

    // If sample input evaluated correctly, print output of part 1 with 
    // challenge output.
    formatted_print("1", part1(&input));

    // Part 2
    // Define sample answer
    let sample_answer_part_2 = "26";

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