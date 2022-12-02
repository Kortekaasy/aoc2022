use std::fmt::Display;

// ========================= Challenge Logic ============================
// Define your own output type here for the `parse_input` function.
type ParsedInput = Vec<(char, char)>;

pub fn parse_input(input: &str) -> ParsedInput {
    input.lines()
        .map(|l| {
            // Take first and third characters from line as moves
            let theirs = l.chars().next().expect("Expected opponent move!"); 
            let ours = l.chars().skip(2).next().expect("Expected opponent move!");
            (theirs, ours)
        }
        ).collect::<Vec<(char, char)>>()
}

pub fn part1(input: &ParsedInput) -> impl Display {
    // Compute lookup table (LUT) for all 9 different scenarios based on their
    // on their move and our move
    let mut LUT: [i16; 6031] = [0;6031];
    LUT[('A' as usize) * ('X' as usize)] = 1 + 3; // rock + draw;
    LUT[('A' as usize) * ('Y' as usize)] = 2 + 6; // paper + win;
    LUT[('A' as usize) * ('Z' as usize)] = 3 + 0; // scissors + loss;
    LUT[('B' as usize) * ('X' as usize)] = 1 + 0; // rock + loss;
    LUT[('B' as usize) * ('Y' as usize)] = 2 + 3; // paper + draw;
    LUT[('B' as usize) * ('Z' as usize)] = 3 + 6; // scissors + win;
    LUT[('C' as usize) * ('X' as usize)] = 1 + 6; // rock + win;
    LUT[('C' as usize) * ('Y' as usize)] = 2 + 0; // paper + loss;
    LUT[('C' as usize) * ('Z' as usize)] = 3 + 3; // scissors + draw;

    // Loop through input and compute score
    input.iter()
        // Compute index in LUT
        .map(|&(theirs, ours)| (theirs as usize) * (ours as usize))
        // Get value from LUT
        .map(|i| LUT[i])
        // Compute sum
        .sum::<i16>()
}

pub fn part2(input: &ParsedInput) -> impl Display {
    // Compute lookup table (LUT) for all 9 different scenarios based on their
    // on their move and the desired outcome.
    let mut LUT: [i16; 6031] = [0;6031];
    LUT[('A' as usize) * ('X' as usize)] = 3 + 0; // scissors + loss
    LUT[('A' as usize) * ('Y' as usize)] = 1 + 3; // rock + draw
    LUT[('A' as usize) * ('Z' as usize)] = 2 + 6; // paper + win
    LUT[('B' as usize) * ('X' as usize)] = 1 + 0; // rock + loss
    LUT[('B' as usize) * ('Y' as usize)] = 2 + 3; // paper + draw
    LUT[('B' as usize) * ('Z' as usize)] = 3 + 6; // scissors + win
    LUT[('C' as usize) * ('X' as usize)] = 2 + 0; // paper + loss
    LUT[('C' as usize) * ('Y' as usize)] = 3 + 3; // scissors + draw
    LUT[('C' as usize) * ('Z' as usize)] = 1 + 6; // rock + win

    // Loop through input and compute score
    input.iter()
        // Compute index in LUT
        .map(|&(theirs, ours)| (theirs as usize) * (ours as usize))
        // Get value from LUT
        .map(|i| LUT[i])
        // Compute sum
        .sum::<i16>()
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    // Parse sample and challenge input
    let sample = parse_input(&read_file("sample"));
    let input = parse_input(&read_file("input"));

    // Part 1
    // Define sample answer
    let sample_answer_part_1 = "15";

    // Evaluate sample input, and compare with sample answer
    assert_eq!(format!("{}", part1(&sample)), sample_answer_part_1);

    // If sample input evaluated correctly, print output of part 1 with 
    // challenge output.
    formatted_print("1", part1(&input));

    // Part 2
    // Define sample answer
    let sample_answer_part_2 = "12";

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