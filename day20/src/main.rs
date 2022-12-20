use std::fmt::Display;

// ========================= Challenge Logic ============================
// Define your own output type here for the `parse_input` function.
type ParsedInput = Vec<(usize, i64)>;

pub fn parse_input(input: &str) -> ParsedInput {
    input.lines()
    .map(|l| 
        l.parse::<i64>().expect(&format!("Expected num got {}", l))
    ).enumerate().collect()
}

pub fn part1(input: &ParsedInput) -> impl Display {
    let mut nums = input.clone();

    let mut ptr = 0;

    while let Some(i) = nums.iter()
        .enumerate()
        .find(|(_mixed_index, (i, _v))| *i == ptr )
        .map(|(mixed_index, _v)| mixed_index) {
        // Remove old values
        let val = nums.remove(i);
        
        // Compute new index
        let new_index = (i as i64 + val.1).rem_euclid(nums.len() as i64) as usize;
        
        // Insert new values
        nums.insert(new_index, val);

        // Increase pointer
        ptr += 1;
    }

    // Find index where '0' is located
    let zero_index = nums.iter()
    .enumerate()
    .find(|&(_i, (_og_index, v))| *v == 0)
    .map(|(i, _v)| i).unwrap();

    // Return the answer
    nums[(zero_index + 1000) % nums.len()].1 + 
    nums[(zero_index + 2000) % nums.len()].1 + 
    nums[(zero_index + 3000) % nums.len()].1 
}

pub fn part2(input: &ParsedInput) -> impl Display {

    const DECRYPTION_KEY: i64 = 811589153;

    // Multiply all values with the decryption key
    let mut nums = input.iter()
    .map(|&(i, v)| (i, v * DECRYPTION_KEY))
    .collect::<Vec<(usize, i64)>>();


    for _ in 0..10 {

        let mut ptr = 0;

        while let Some(i) = nums.iter()
            .enumerate()
            .find(|(_mixed_index, (i, _v))| *i == ptr )
            .map(|(mixed_index, _v)| mixed_index) {
            
            // Remove old values
            let val = nums.remove(i);
            
            // Compute new index
            let new_index = (i as i64 + val.1).rem_euclid(nums.len() as i64) as usize;
            
            // Insert new values
            nums.insert(new_index, val);

            // Increase pointer
            ptr += 1;

        }
    }

    // Find index where '0' is located
    let zero_index = nums.iter()
    .enumerate()
    .find(|&(_i, (_og_index, v))| *v == 0)
    .map(|(i, _v)| i).unwrap();

    // Return the answer
    nums[(zero_index + 1000) % nums.len()].1 + 
    nums[(zero_index + 2000) % nums.len()].1 + 
    nums[(zero_index + 3000) % nums.len()].1
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    // Parse sample and challenge input
    let sample = parse_input(&read_file("sample"));
    let input = parse_input(&read_file("input"));

    // Part 1
    // Define sample answer
    let sample_answer_part_1 = "3";

    // Evaluate sample input, and compare with sample answer
    assert_eq!(format!("{}", part1(&sample)), sample_answer_part_1);

    // If sample input evaluated correctly, print output of part 1 with 
    // challenge output.
    formatted_print("1", part1(&input));

    // Part 2
    // Define sample answer
    let sample_answer_part_2 = "1623178306";

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