use std::fmt::Display;

// ========================= Challenge Logic ============================
// Define your own output type here for the `parse_input` function.
type ParsedInput = Vec<Vec<i32>>;

pub fn parse_input(input: String) -> ParsedInput {
    // Create a new vector to hold the parsed result
    let mut parsed: ParsedInput = Vec::new();

    // Add a vector for the input of the first elf
    parsed.push(Vec::new());

    // Go through all lines in the input
    for l in input.lines() {
        // If we parsed a number, add it to the last vector in the result vecvec
        if let Ok(i) = l.parse::<i32>() {
            parsed.last_mut().unwrap().push(i);
        } else {
        // If we could not parse a number, it means it is an empty line, so add
        // a new vector for the new elf.
            parsed.push(Vec::new());
        }
    }

    // Return the parsed input vector.
    parsed
}

pub fn part1(input: &ParsedInput) -> impl Display {
    // Sum up all calories of the elves, and take the maximum
    let max = input.iter()
        .map(|e| e.iter().sum::<i32>())
        .max();

    // Return the maximum value
    max.unwrap()
}

#[allow(dead_code)]
pub fn part2_1(input: &ParsedInput) -> impl Display {
    let mut sums = input.iter()
        .map(|e| e.iter().sum::<i32>())
        .collect::<Vec<i32>>();
    sums.sort_by(|a, b| b.cmp(a));
    let top3 = sums.iter().take(3).sum::<i32>();
    top3
}

pub fn part2(input: &ParsedInput) -> impl Display {
    // Sum up all calories of the elves
    let mut sums = input.iter()
        .map(|e| e.iter().sum::<i32>())
        .collect::<Vec<i32>>();
    
    // Make three passed through the calorie vector, get the max, add it to the
    // running sum, and remove that element from the vector.
    let mut sum = 0;
    for _ in 0..3 {
        let (i, max) = sums.iter().enumerate().max_by(|(_, &a), (_, &b)| a.cmp(&b)).unwrap();
        sum += max;
        sums.remove(i);
    }

    // Return the sum of the top three element
    sum
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    // Parse sample and challenge input
    let sample = parse_input(read_file("sample"));
    let input = parse_input(read_file("input"));

    // Part 1
    // Define sample answer
    let sample_answer_part_1 = "24000";

    // Evaluate sample input, and compare with sample answer
    assert_eq!(format!("{}", part1(&sample)), sample_answer_part_1);

    // If sample input evaluated correctly, print output of part 1 with 
    // challenge output.
    formatted_print("1", part1(&input));

    // Part 2
    // Define sample answer
    let sample_answer_part_2 = "45000";

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