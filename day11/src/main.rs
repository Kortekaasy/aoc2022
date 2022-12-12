use std::fmt::Display;

// ========================= Challenge Logic ============================
// Define your own output type here for the `parse_input` function.
#[derive(Clone, Copy, Debug)]
pub enum Op {
    AddSelf,
    AddConst(i64),
    MulSelf,
    MulConst(i64),
}

#[derive(Clone, Debug)]
pub struct Monkey {
    items: Vec<i64>,
    update: Op,
    test: i64,
    pub true_monkey: usize,
    pub false_monkey: usize,
    pub inspected: usize,
    reduction: i64,
}

impl Monkey {
    pub fn new(items: Vec<i64>, update: Op, test: i64, true_monkey: usize, false_monkey: usize) -> Monkey {
        Monkey { 
            items,
            update,
            test, 
            true_monkey,
            false_monkey, 
            inspected: 0,
            reduction: 0,
        }
    }

    pub fn set_reduction(&mut self, reduction: i64) {
        self.reduction = reduction;
    }

    pub fn add_items(&mut self, items: Vec<i64>) {
        self.items.extend(items)
    }

    pub fn action(&mut self) -> (Vec<i64>, Vec<i64>) {
        let mut to_true_monkey = Vec::new();
        let mut to_false_monkey = Vec::new();
        self.inspected += self.items.len();
        for item in self.items.drain(..) {
            let updated = match self.update {
                Op::AddSelf => item + item,
                Op::AddConst(v) => item + v,
                Op::MulSelf => item * item,
                Op::MulConst(v) => item * v,
            } / 3;

            if (updated % self.test) == 0 {
                to_true_monkey.push(updated % 96577);
            } else {
                to_false_monkey.push(updated % 96577);
            }
        } 
        (to_true_monkey, to_false_monkey)
    }

    pub fn action2(&mut self) -> (Vec<i64>, Vec<i64>) {
        let mut to_true_monkey = Vec::new();
        let mut to_false_monkey = Vec::new();
        self.inspected += self.items.len();
        for item in self.items.drain(..) {
            let updated = match self.update {
                Op::AddSelf => item + item,
                Op::AddConst(v) => item + v,
                Op::MulSelf => item * item,
                Op::MulConst(v) => item * v,
            } % self.reduction;

            if (updated % self.test) == 0 {
                to_true_monkey.push(updated);
            } else {
                to_false_monkey.push(updated);
            }
        } 
        (to_true_monkey, to_false_monkey)
    }
}

type ParsedInput = Vec<Monkey>;

pub fn parse_input(input: &str) -> ParsedInput {
    input.split("\n\n")
    .map(|m| {
        let lines = m.lines().collect::<Vec<&str>>();

        let starting_items = lines[1][18..].split(", ")
        .map(|it|
            it.parse::<i64>().expect("item parsing error")
        )
        .collect::<Vec<i64>>();

        let update_split = lines[2].split(" ").collect::<Vec<&str>>();
        let update = match (update_split[6], update_split[7].parse::<i64>()) {
            ("+", Ok(const_val)) => Op::AddConst(const_val),
            ("+", _err) => Op::AddSelf,
            ("*", Ok(const_val)) => Op::MulConst(const_val),
            ("*", _err) => Op::MulSelf,
            _ => panic!("unexpected operator! {} ; {} ; {}", lines[2], update_split[6], update_split[7])
        };

        let test = lines[3].rsplit_once(" ").unwrap().1.parse::<i64>().expect("test");
        let true_monkey = lines[4].rsplit_once(" ").unwrap().1.parse::<usize>().expect("true_monkey");
        let false_monkey = lines[5].rsplit_once(" ").unwrap().1.parse::<usize>().expect("false_monkey");

        Monkey::new(starting_items, update, test, true_monkey, false_monkey)
    })
    .collect()
}

pub fn part1(input: &ParsedInput) -> impl Display {
    let mut monkeys = input.clone();
    let rounds = 20;

    // Compute & set reduction
    let reduction = monkeys.iter().fold(1, |acc, e| acc * e.test);
    monkeys.iter_mut().for_each(|m| m.set_reduction(reduction));

    for _i in 0..rounds {
        for i in 0..monkeys.len() {
            let true_monkey = monkeys[i].true_monkey;
            let false_monkey = monkeys[i].false_monkey;
            let (to_true_monkey, to_false_monkey) = monkeys[i].action();
            monkeys[true_monkey].add_items(to_true_monkey);
            monkeys[false_monkey].add_items(to_false_monkey);
        }

    }
    let mut inspected = monkeys.iter().map(|m| m.inspected).collect::<Vec<usize>>();
    inspected.sort();

    inspected[inspected.len() - 1] * inspected[inspected.len() - 2]
}

pub fn part2(input: &ParsedInput) -> impl Display {
    let mut monkeys = input.clone();
    let rounds = 10_000;

    // Compute & set reduction
    let reduction = monkeys.iter().fold(1, |acc, e| acc * e.test);
    monkeys.iter_mut().for_each(|m| m.set_reduction(reduction));

    for _i in 0..rounds {
        for i in 0..monkeys.len() {
            let true_monkey = monkeys[i].true_monkey;
            let false_monkey = monkeys[i].false_monkey;
            let (to_true_monkey, to_false_monkey) = monkeys[i].action2();
            monkeys[true_monkey].add_items(to_true_monkey);
            monkeys[false_monkey].add_items(to_false_monkey);
        }

    }
    let mut inspected = monkeys.iter().map(|m| m.inspected).collect::<Vec<usize>>();
    inspected.sort();

    inspected[inspected.len() - 1] * inspected[inspected.len() - 2]
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    // Parse sample and challenge input
    let sample = parse_input(&read_file("sample"));
    let input = parse_input(&read_file("input"));

    // Part 1
    // Define sample answer
    let sample_answer_part_1 = "10605";

    // Evaluate sample input, and compare with sample answer
    assert_eq!(format!("{}", part1(&sample)), sample_answer_part_1);

    // If sample input evaluated correctly, print output of part 1 with 
    // challenge output.
    formatted_print("1", part1(&input));

    // Part 2
    // Define sample answer
    let sample_answer_part_2 = "2713310158";

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