use std::fmt::Display;

// ========================= Challenge Logic ============================
// Define your own output type here for the `parse_input` function.
type ParsedInput = Vec<(char, char)>;

pub fn parse_input(input: String) -> ParsedInput {
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
    input.iter().map(|(theirs, ours)| {
        // For every game, compute score based on their move and our move
        match (theirs, ours) {
            ('A', 'X') => 1 + 3, // rock + draw
            ('A', 'Y') => 2 + 6, // paper + win
            ('A', 'Z') => 3 + 0, // scissors + loss
            ('B', 'X') => 1 + 0, // rock + loss
            ('B', 'Y') => 2 + 3, // paper + draw
            ('B', 'Z') => 3 + 6, // scissors + win
            ('C', 'X') => 1 + 6, // rock + win
            ('C', 'Y') => 2 + 0, // paper + loss
            ('C', 'Z') => 3 + 3, // scissors + draw
            ( _ ,  _ ) => panic!("Invalid move!")
        }
    }).sum::<i32>()
}

pub fn part2(input: &ParsedInput) -> impl Display {
    input.iter().map(|(theirs, ours)| {
        // For every game, compute score based on their move and desired outcome
        match (theirs, ours) {
            ('A', 'X') => 3 + 0, // scissors + loss
            ('A', 'Y') => 1 + 3, // rock + draw
            ('A', 'Z') => 2 + 6, // paper + win
            ('B', 'X') => 1 + 0, // rock + loss
            ('B', 'Y') => 2 + 3, // paper + draw
            ('B', 'Z') => 3 + 6, // scissors + win
            ('C', 'X') => 2 + 0, // paper + loss
            ('C', 'Y') => 3 + 3, // scissors + draw
            ('C', 'Z') => 1 + 6, // rock + win
            ( _ ,  _ ) => panic!("Invalid move!")
        }
    }).sum::<i32>()
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    // Parse sample and challenge input
    let sample = parse_input(read_file("sample"));
    let input = parse_input(read_file("input"));

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