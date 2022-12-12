use std::fmt::Display;

// ========================= Challenge Logic ============================
// Define your own output type here for the `parse_input` function.
#[derive(Debug)]
pub enum Instr {
    Add(i32),
    NoOp,
}

type ParsedInput = Vec<Instr>;

pub fn parse_input(input: &str) -> ParsedInput {
    input.lines().map(|l|
        match l.split_once(' ') {
            Some(("addx", amt)) => Instr::Add(amt.parse::<i32>().expect("expected amount")),
            None => Instr::NoOp,
            Some((l1, l2)) => panic!("unexpected instruction! ({} {})", l1, l2) 
        }
    )
    .collect()
}

pub fn part1(input: &ParsedInput) -> impl Display {
    // let mut cycle = 0;
    let mut history: Vec<i32> = Vec::with_capacity(300);
    let mut current = 1;
    history.push(current);
    
    for instr in input {
        match instr {
            Instr::Add(amt) => {
                history.push(current);
                history.push(current);
                current += amt;
            },
            Instr::NoOp => history.push(current),
        }
    
    }

    20 * history[20] + 
    60 * history[60] + 
    100 * history[100] + 
    140 * history[140] + 
    180 * history[180] + 
    220 * history[220] 
}

pub fn part2(input: &ParsedInput) -> impl Display {
    let mut history: Vec<i32> = Vec::with_capacity(300);
    let mut current = 1;
    history.push(current);
    
    for instr in input {
        match instr {
            Instr::Add(amt) => {
                history.push(current);
                history.push(current);
                current += amt;
            },
            Instr::NoOp => history.push(current),
        }
    }

    let mut crt: Vec<char> = Vec::with_capacity(6*41);
    for j in 0..6 {
        for i in 1..=40 {
            let signal = history[(j * 40 + i) as usize];
            if (signal..signal+3).contains(&i) {
                crt.push('█');
            } else {
                crt.push(' ');
            }
        }
        crt.push('\n');
    }
    crt.iter().collect::<String>()
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    // Parse sample and challenge input
    let sample = parse_input(&read_file("sample"));
    let input = parse_input(&read_file("input"));

    // Part 1
    // Define sample answer
    let sample_answer_part_1 = "13140";

    // Evaluate sample input, and compare with sample answer
    assert_eq!(format!("{}", part1(&sample)), sample_answer_part_1);

    // If sample input evaluated correctly, print output of part 1 with 
    // challenge output.
    formatted_print("1", part1(&input));

    // Part 2
    // Define sample answer
    let sample_answer_part_2 = "██  ██  ██  ██  ██  ██  ██  ██  ██  ██  
███   ███   ███   ███   ███   ███   ███ 
████    ████    ████    ████    ████    
█████     █████     █████     █████     
██████      ██████      ██████      ████
███████       ███████       ███████     
";

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