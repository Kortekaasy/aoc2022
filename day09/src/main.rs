use std::fmt::Display;
use std::io::stdin;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::collections::HashSet;

// ========================= Challenge Logic ============================
// Define your own output type here for the `parse_input` function.
#[derive(Debug)]
pub enum Move {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coord<const N: usize>([i32; N]);

impl<const N: usize> Add<Coord<N>> for Coord<N> {
    type Output = Coord<N>;

    fn add(mut self, rhs: Coord<N>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<const N: usize> AddAssign<Coord<N>> for Coord<N> {
    fn add_assign(&mut self, rhs: Coord<N>) {
        self.0.iter_mut().zip(&rhs.0).for_each(|(l, r)| *l += r);
    }
}

impl<const N: usize> Sub<Coord<N>> for Coord<N> {
    type Output = Coord<N>;

    fn sub(mut self, rhs: Coord<N>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<const N: usize> SubAssign<Coord<N>> for Coord<N> {
    fn sub_assign(&mut self, rhs: Coord<N>) {
        self.0.iter_mut().zip(&rhs.0).for_each(|(l, r)| *l -= r);
    }
}

impl<const N: usize> Coord<N> {

    pub fn abs(&self) -> Coord<N> {
        Coord(self.0.map(|x| x.abs()))
    }
}


type ParsedInput = Vec<(Move, i32)>;

pub fn parse_input(input: &str) -> ParsedInput {
    input.lines()
    .map(|l| 
        match l.split_once(' ') {
            Some(("U", amt)) => (Move::Up, amt.parse::<i32>().expect("invalid amount for move")),
            Some(("R", amt)) => (Move::Right, amt.parse::<i32>().expect("invalid amount for move")),
            Some(("D", amt)) => (Move::Down, amt.parse::<i32>().expect("invalid amount for move")),
            Some(("L", amt)) => (Move::Left, amt.parse::<i32>().expect("invalid amount for move")),
            _ => panic!("Invalid input!")
        }
    )
    .collect()
}

pub fn print_board<const N: usize>(knots: &[Coord<2>; N], visited: &HashSet<Coord<2>>) {
    for j in (0..5).rev() {
        'inner: for i in 0..6 {
            let curr = Coord([i, j]);
            if curr == knots[0] {
                print!("H");
            } else {
                for i in 1..N {
                    if curr == knots[i] {
                        print!("{}", i);
                        continue 'inner;
                    }
                }
                if curr == Coord([0, 0]) {
                    print!("s");
                } else if visited.contains(&curr) {
                    // print!("#");
                    print!(".");
                } else {
                    print!(".");
                }
            }
        }
        println!("");
    }
}

pub fn simulate_rope<const num_knots: usize>(input: &Vec<(Move, i32)>) -> HashSet<Coord<2>>{
    let mut visited: HashSet<Coord<2>> = HashSet::new();
    
    let mut knots = [Coord([0; 2]); num_knots];

    for (m, by) in input {
        for _ in 0..*by {
            match m {
                Move::Up    => knots[0] += Coord([ 0,  1]),
                Move::Right => knots[0] += Coord([ 1,  0]),
                Move::Down  => knots[0] += Coord([ 0, -1]),
                Move::Left  => knots[0] += Coord([-1,  0]),
            }
            
            for i in 1..num_knots {
                match knots[i-1] - knots[i] {
                    Coord([x, y]) if x >  1 => knots[i] += Coord([x-1, y.clamp(-1, 1)]),
                    Coord([x, y]) if x < -1 => knots[i] += Coord([x+1, y.clamp(-1, 1)]),
                    Coord([x, y]) if y >  1 => knots[i] += Coord([x.clamp(-1, 1), y-1]),
                    Coord([x, y]) if y < -1 => knots[i] += Coord([x.clamp(-1, 1), y+1]),
                    Coord(_) => (),
                }
            }
            
            visited.insert(knots[num_knots - 1]);
        }
    }

    visited
}

pub fn part1(input: &ParsedInput) -> impl Display {
    let visited = simulate_rope::<2>(input);

    visited.len()
}

pub fn part2(input: &ParsedInput) -> impl Display {
    let visited = simulate_rope::<10>(input);

    visited.len()
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    // Parse sample and challenge input
    let sample = parse_input(&read_file("sample"));
    let input = parse_input(&read_file("input"));

    // Part 1
    // Define sample answer
    let sample_answer_part_1 = "88";

    // Evaluate sample input, and compare with sample answer
    assert_eq!(format!("{}", part1(&sample)), sample_answer_part_1);

    // If sample input evaluated correctly, print output of part 1 with 
    // challenge output.
    formatted_print("1", part1(&input));

    // Part 2
    // Define sample answer
    let sample_answer_part_2 = "36";

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