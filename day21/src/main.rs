use std::fmt::Display;
use std::collections::HashMap;

// ========================= Challenge Logic ============================
// Define your own output type here for the `parse_input` function.
#[derive(Copy, Clone, Debug)]
pub enum Yell {
    Constant(i64),
    Add(u32, u32),
    Sub(u32, u32),
    Mul(u32, u32),
    Div(u32, u32),
}

type ParsedInput = HashMap<u32, Yell>;

#[inline(always)]
pub fn slice_to_identifier(slice: &[u8]) -> u32 {
    u32::from_be_bytes(slice.try_into().expect("Could not convert slice to identifier!"))
}

pub fn parse_input(input: &str) -> ParsedInput {
    input.lines()
    .map(
        |l| {
            let bytes = l.as_bytes();
            match bytes.get(11) {
                Some(b'+') => (
                    slice_to_identifier(&bytes[0..4]), 
                    Yell::Add(
                        slice_to_identifier(&bytes[ 6..10]), 
                        slice_to_identifier(&bytes[13..17])
                    )
                ),
                Some(b'-') => (
                    slice_to_identifier(&bytes[0..4]), 
                    Yell::Sub(
                        slice_to_identifier(&bytes[ 6..10]), 
                        slice_to_identifier(&bytes[13..17])
                    )
                ),
                Some(b'*') => (
                    slice_to_identifier(&bytes[0..4]), 
                    Yell::Mul(
                        slice_to_identifier(&bytes[ 6..10]), 
                        slice_to_identifier(&bytes[13..17])
                    )
                ),
                Some(b'/') => (
                    slice_to_identifier(&bytes[0..4]), 
                    Yell::Div(
                        slice_to_identifier(&bytes[ 6..10]), 
                        slice_to_identifier(&bytes[13..17])
                    )
                ),
                _ => (
                    slice_to_identifier(&bytes[0..4]),
                    Yell::Constant(l[6..].parse::<i64>().unwrap())
                )
            }
        }
    ).collect()
}

#[derive(Debug)]
pub enum Error {
    NotFound,
}

type Result<T> = std::result::Result<T, Error>;

pub fn get_and_update(key: &u32, map: &mut HashMap<u32, Yell>) -> Result<i64> {
    match map.get(key) {
        Some(&Yell::Constant(c)) => Ok(c),
        Some(&Yell::Add(k1, k2)) => {
            let r1 = get_and_update(&k1, map);
            let r2 = get_and_update(&k2, map);
            match (r1, r2) {
                (Ok(v1), Ok(v2)) => {
                    let v = v1 + v2;
                    map.insert(*key, Yell::Constant(v));
                    Ok(v)
                },
                _ => Err(Error::NotFound)
            }
        },
        Some(&Yell::Sub(k1, k2)) => {
            let r1 = get_and_update(&k1, map);
            let r2 = get_and_update(&k2, map);
            match (r1, r2) {
                (Ok(v1), Ok(v2)) => {
                    let v = v1 - v2;
                    map.insert(*key, Yell::Constant(v));
                    Ok(v)
                },
                _ => Err(Error::NotFound)
            }
        },
        Some(&Yell::Mul(k1, k2)) => {
            let r1 = get_and_update(&k1, map);
            let r2 = get_and_update(&k2, map);
            match (r1, r2) {
                (Ok(v1), Ok(v2)) => {
                    let v = v1 * v2;
                    map.insert(*key, Yell::Constant(v));
                    Ok(v)
                },
                _ => Err(Error::NotFound)
            }
        },
        Some(&Yell::Div(k1, k2)) => {
            let r1 = get_and_update(&k1, map);
            let r2 = get_and_update(&k2, map);
            match (r1, r2) {
                (Ok(v1), Ok(v2)) => {
                    let v = v1 / v2;
                    map.insert(*key, Yell::Constant(v));
                    Ok(v)
                },
                _ => Err(Error::NotFound)
            }
        },
        None => Err(Error::NotFound),
    }
}

pub fn part1(input: &ParsedInput) -> impl Display {
    let root = u32::from_be_bytes([b'r', b'o', b'o', b't']);
    let mut map = input.clone();
    get_and_update(&root, &mut map).unwrap()
}

pub fn part2(input: &ParsedInput) -> impl Display {
    let root = u32::from_be_bytes([b'r', b'o', b'o', b't']);
    let human = u32::from_be_bytes([b'h', b'u', b'm', b'n']);
    
    let mut map = input.clone();
    map.remove(&human);

    let mut key = root;
    let mut val = -1;

    if let Some(&Yell::Add(l, r)) = map.get(&root) {
        let left = get_and_update(&l, &mut map);
        let right = get_and_update(&r, &mut map);
        if let Ok(c) = left {
            val = c;
            key = r;
        } else if let Ok(c) = right {
            val = c;
            key = l;
        }
    }


    while key != human {
        match map[&key] {
            Yell::Constant(_) => todo!(),
            Yell::Add(l, r) => {
                match (map.get(&l), map.get(&r)) {
                    (Some(Yell::Constant(c)), _) => { val = val - c; key = r; } ,
                    (_, Some(Yell::Constant(c))) => { val = val - c; key = l; } ,
                    (_, _) => todo!(),
                };
            },
            Yell::Sub(l, r) => {
                match (map.get(&l), map.get(&r)) {
                    (Some(Yell::Constant(c)), _) => { val = c - val; key = r; } ,
                    (_, Some(Yell::Constant(c))) => { val = val + c; key = l; } ,
                    (_, _) => todo!(),
                };
            },
            Yell::Mul(l, r) => {
                match (map.get(&l), map.get(&r)) {
                    (Some(Yell::Constant(c)), _) => { val = val / c; key = r; } ,
                    (_, Some(Yell::Constant(c))) => { val = val / c; key = l; } ,
                    (_, _) => todo!(),
                };
            },
            Yell::Div(l, r) => {
                match (map.get(&l), map.get(&r)) {
                    (Some(Yell::Constant(c)), _) => { val = c / val; key = r; } ,
                    (_, Some(Yell::Constant(c))) => { val = val * c; key = l; } ,
                    (_, _) => todo!(),
                };
            },
        };

    }

    val
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    // Parse sample and challenge input
    let sample = parse_input(&read_file("sample"));
    let input = parse_input(&read_file("input"));

    // Part 1
    // Define sample answer
    let sample_answer_part_1 = "152";

    // Evaluate sample input, and compare with sample answer
    assert_eq!(format!("{}", part1(&sample)), sample_answer_part_1);

    // If sample input evaluated correctly, print output of part 1 with 
    // challenge output.
    formatted_print("1", part1(&input));

    // Part 2
    // Define sample answer
    let sample_answer_part_2 = "301";

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