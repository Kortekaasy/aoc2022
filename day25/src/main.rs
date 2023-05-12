use std::collections::VecDeque;
use std::fmt::{Display, Write}; 
use std::ops::{Deref, DerefMut};

// ========================= Challenge Logic ============================
// Define your own output type here for the `parse_input` function.
#[derive(Copy, Clone, Debug)]
#[repr(usize)]
pub enum SNAFUDigit {
    MIN_TWO = 0,
    MIN_ONE = 1,
    ZERO = 2,
    ONE = 3,
    TWO = 4
}

#[derive(Clone, Debug)]
pub struct SNAFU(VecDeque<SNAFUDigit>);

impl SNAFU {
    
    pub fn new() -> SNAFU {
        SNAFU(VecDeque::new())
    }

    pub fn with_capacity(capacity: usize) -> SNAFU {
        SNAFU(VecDeque::with_capacity(capacity))
    }
}

impl Deref for SNAFU {
    type Target = VecDeque<SNAFUDigit>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SNAFU {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<SNAFU> for i64 {
    fn from(val: SNAFU) -> Self {
        let l = val.len();
        val.iter()
        .enumerate()
        .map(|(i, &digit)|
            LUT[l-i-1][digit as usize]
        )
        .sum::<i64>()
    }
}

impl From<&SNAFU> for i64 {
    fn from(val: &SNAFU) -> Self {
        let l = val.len();
        val.iter()
        .enumerate()
        .map(|(i, &digit)|
            LUT[l-i-1][digit as usize]
        )
        .sum::<i64>()
    }
}

impl From<i64> for SNAFU {
    fn from(mut val: i64) -> Self {
        let mut carry = 0;
        let mut s = SNAFU::with_capacity(32);

        while val != 0 {
            // Compute current left-most digit in to unbalanced quinary (r)
            let (d, r) = (val.div_euclid(5), val.rem_euclid(5));

            // Convert to balanced quinary, by adding SNAFUDigit::TWO, _with carry_
            // in the unbalanced 'domain', and then subtracting SNAFUDigit::TWO, 
            // _without borrow_
            match r + carry {
                0 => { carry = 0; s.push_front(SNAFUDigit::ZERO) },
                1 => { carry = 0; s.push_front(SNAFUDigit::ONE) },
                2 => { carry = 0; s.push_front(SNAFUDigit::TWO) },
                3 => { carry = 1; s.push_front(SNAFUDigit::MIN_TWO)},
                4 => { carry = 1; s.push_front(SNAFUDigit::MIN_ONE)},
                5 => { carry = 1; s.push_front(SNAFUDigit::ZERO)},
                _ => panic!("Invalid digit!")
            }

            // Set division as new val
            val = d;
        }
        if carry == 1 {
            s.push_front(SNAFUDigit::ONE);
        }
        s
    }
}

impl FromIterator<SNAFUDigit> for SNAFU {
    fn from_iter<T: IntoIterator<Item = SNAFUDigit>>(iter: T) -> Self {
        SNAFU(VecDeque::from_iter(iter))
    }
}

impl Display for SNAFUDigit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SNAFUDigit::MIN_TWO => f.write_char('='),
            SNAFUDigit::MIN_ONE => f.write_char('-'),
            SNAFUDigit::ZERO => f.write_char('0'),
            SNAFUDigit::ONE => f.write_char('1'),
            SNAFUDigit::TWO => f.write_char('2'),
        }
    }
}

impl Display for SNAFU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.iter().fold(Ok(()), |r, d| {
            r.and_then(|_| write!(f, "{}", d))
        })
    }
}

type ParsedInput = Vec<SNAFU>;

const LUT: [[i64; 5]; 27] = {
    let mut i = 0;
    let mut pow_5 = 1;
    let mut _lut = [[0; 5]; 27];
    
    while i < 27 {
        _lut[i][SNAFUDigit::MIN_TWO as usize] = -2 * pow_5;
        _lut[i][SNAFUDigit::MIN_ONE as usize] = -1 * pow_5;
        _lut[i][SNAFUDigit::ZERO as usize] = 0 * pow_5;
        _lut[i][SNAFUDigit::ONE as usize] = 1 * pow_5;
        _lut[i][SNAFUDigit::TWO as usize] = 2 * pow_5;
        
        i += 1;
        pow_5 *= 5;
    } 

    _lut
};

pub fn parse_input(input: &str) -> ParsedInput {
    input.lines()
    .map(|l|
        l.as_bytes()
        .into_iter()
        .map(|c| 
            match c {
                &b'=' => SNAFUDigit::MIN_TWO,
                &b'-' => SNAFUDigit::MIN_ONE,
                &b'0' => SNAFUDigit::ZERO,
                &b'1' => SNAFUDigit::ONE,
                &b'2' => SNAFUDigit::TWO,
                _ => panic!("Invalid character {}!", c),
            }
        ).collect()
    )
    .collect()
}

pub fn part1(input: &ParsedInput) -> impl Display {
    SNAFU::from(input.into_iter()
    .map(|number| 
        i64::from(number)
    )
    .sum::<i64>())
}

pub fn part2(input: &ParsedInput) -> impl Display {
    format!("Part2 not implemented!")
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    // Parse sample and challenge input
    let sample = parse_input(&read_file("sample"));
    let input = parse_input(&read_file("input"));

    // Part 1
    // Define sample answer
    let sample_answer_part_1 = "2=-1=0";

    // Evaluate sample input, and compare with sample answer
    assert_eq!(format!("{}", part1(&sample)), sample_answer_part_1);

    // If sample input evaluated correctly, print output of part 1 with 
    // challenge output.
    formatted_print("1", part1(&input));

    // Part 2
    // Define sample answer
    let sample_answer_part_2 = "";

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