use std::fmt::Display;
use itertools::Itertools;

// ========================= Challenge Logic ============================
// Define your own output type here for the `parse_input` function.
type ParsedInput<'a> = &'a str;

// #[inline(always)]
// pub fn char_to_shift(c: &u8) -> usize {
//     // Convert a ascii character (u8) to a bitshift where 'a' = 0 and 'z' = 25
//     (*c - b'a') as usize
// }

pub fn parse_input(input: &str) -> ParsedInput {
    // // Take input as byte, iterate over them
    // input.as_bytes().iter()
    // // Map each character to a bitshift
    // .map(char_to_shift)
    // // Leftshift 0x00000001 by the calculated bitshift
    // .map(|shift| 1_u32 << shift)
    // // Collect the shifted values
    // .collect()
    input
}

fn find_marker(input: &str, n: usize) -> usize {
    let input = input.trim().chars().collect_vec();
    input.windows(n).enumerate()
        .filter(|(_, window)| window.into_iter().all_unique())
        .map(|(i, _)| i + n)
        .next().unwrap()
}
pub fn part1(input: ParsedInput) -> impl Display {
    find_marker(input, 4)
}

pub fn part2(input: ParsedInput) -> impl Display {
    find_marker(input, 14)
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    // Parse sample and challenge input
    let sample_string = read_file("sample");
    let sample = parse_input(&sample_string);
    let input_string = read_file("input");
    let input = parse_input(&input_string);

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