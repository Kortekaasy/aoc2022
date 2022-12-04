use std::fmt::Display;

// ========================= Challenge Logic ============================
// Define your own output type here for the `parse_input` function.
type ParsedInput = Vec<((usize, usize), (usize, usize))>;

pub fn parse_input(input: &str) -> ParsedInput {
    input.lines()
        .map(|l| l.split_once(",").expect("Expected delim"))
        .map(|(left, right)| (left.split_once("-").expect("expected dash"), right.split_once("-").expect("expected dash")))
        .map(|((ll, lr), (rl, rr))| {
            (
                (ll.parse().expect("first coordinate"), lr.parse().expect("second coordinate")),
                (rl.parse().expect("third coordinate"), rr.parse().expect("fourth coordinate")),
            )
        }).collect()
}

pub fn part1(input: &ParsedInput) -> impl Display {
    input.iter()
    .filter(|((ll, lr), (rl,rr))| {
           (ll <= rl && lr >= rr) // right interval is contained in left interval
        || (rl <= ll && rr >= lr) // left interval is contained in right interval
    })
    .count()
}

pub fn part2(input: &ParsedInput) -> impl Display {
    input.iter()
    .filter(|((ll, lr), (rl,rr))| {
           (ll <= rl && rl <= lr) // left edge of right interval is contained in left interval
        || (ll <= rr && rr <= lr) // right edge of right interval is contained in left interval
        || (rl <= ll && ll <= rr) // left edge of left interval is contained in right interval
        || (rl <= lr && lr <= rr) // right edge of left interval is contained in right interval
    })
    .count()
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    // Parse sample and challenge input
    let sample = parse_input(&read_file("sample"));
    let input = parse_input(&read_file("input"));

    // Part 1
    // Define sample answer
    let sample_answer_part_1 = "2";

    // Evaluate sample input, and compare with sample answer
    assert_eq!(format!("{}", part1(&sample)), sample_answer_part_1);

    // If sample input evaluated correctly, print output of part 1 with 
    // challenge output.
    formatted_print("1", part1(&input));

    // Part 2
    // Define sample answer
    let sample_answer_part_2 = "4";

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