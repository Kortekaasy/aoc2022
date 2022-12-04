use std::fmt::Display;
use std::collections::{HashMap, HashSet};

// ========================= Challenge Logic ============================
// Define your own output type here for the `parse_input` function.
type ParsedInput = Vec<String>;

pub fn parse_input(input: &str) -> ParsedInput {
    input.lines().map(|l|l.into()).collect()
}

pub fn part1(input: &ParsedInput) -> impl Display {
    let mut priority_map: HashMap<char,i32> = HashMap::new();
    for i in 0..26 {
        priority_map.insert((('a' as u8) + i) as char, (i + 1).into());
    }
    for i in 0..26 {
        priority_map.insert((('A' as u8) + i) as char, (i + 1 + 26).into());
    }

    input.iter().map(|l| {
        let mid = l.len() / 2;
        let first = l.chars().take(mid).collect::<HashSet<char>>();
        let second = l.chars().skip(mid).collect::<HashSet<char>>();
        let common = first.intersection(&second).next().expect("Expected common element!");
        priority_map[common]
    }).sum::<i32>()
}

pub fn part2(input: &ParsedInput) -> impl Display {
    let mut priority_map: HashMap<char,i32> = HashMap::new();
    for i in 0..26 {
        priority_map.insert((('a' as u8) + i) as char, (i + 1).into());
    }
    for i in 0..26 {
        priority_map.insert((('A' as u8) + i) as char, (i + 1 + 26).into());
    }

    input.windows(3).step_by(3).map(|w| {
        let first = w[0].chars().collect::<HashSet<char>>();
        let second = w[1].chars().collect::<HashSet<char>>();
        let third = w[2].chars().collect::<HashSet<char>>();
        let cmn = second.intersection(&third).cloned().collect::<HashSet<char>>();
        let mut common = first.intersection(&cmn);
        let badge = common.next().expect("Expected common element");
        priority_map[badge]
    }).sum::<i32>()
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