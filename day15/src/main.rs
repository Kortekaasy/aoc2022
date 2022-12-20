use std::fmt::Display;
use std::ops::{Add, AddAssign, Sub, SubAssign, Range};
use std::cmp::Ordering;

use regex::Regex;
use itertools::Itertools;

// ========================= Challenge Logic ============================
// Define your own output type here for the `parse_input` function.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coord(i32, i32);

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(mut self, rhs: Coord) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign<Coord> for Coord {
    fn add_assign(&mut self, rhs: Coord) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(mut self, rhs: Coord) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign<Coord> for Coord {
    fn sub_assign(&mut self, rhs: Coord) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

// type Range = (i32, i32);
type ParsedInput = Vec<(Coord, Coord)>;

pub fn parse_input(input: &str) -> ParsedInput {
    let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    re.captures_iter(input)
    .map(|caps| (
        Coord(
            caps.get(1).expect("cap 1").as_str().parse::<i32>().expect(&format!("parse 1 {:?}", caps.get(1))),
            caps.get(2).expect("cap 2").as_str().parse::<i32>().expect(&format!("parse 2 {:?}", caps.get(2)))
        ),
        Coord(
            caps.get(3).expect("cap 3").as_str().parse::<i32>().expect(&format!("parse 3 {:?}", caps.get(3))),
            caps.get(4).expect("cap 4").as_str().parse::<i32>().expect(&format!("parse 4 {:?}", caps.get(4)))
        )
    ))
    .collect()
}

pub fn get_range(sensor: &Coord, beacon: &Coord, y: i32) -> Option<Range<i32>> {
    // unpack location of sensor
    let (s_x, s_y) = (sensor.0, sensor.1);
    // unpack location of beacon
    let (b_x, b_y) = (beacon.0, beacon.1);

    // Compute manhattan distance between beacon and sensor
    let dist = (s_x - b_x).abs() + (s_y - b_y).abs();

    // Compute relative height of point at distance `dist` from sensor, at height `y`
    let d_y = (s_y - y).abs();

    // If height y is too far away, return None
    if d_y > dist {
        None
    } else { // Else, return a range of all x-coordinates within `dist` of the sensor, on height `y`
        let d_x = dist - d_y;
        Some(s_x-d_x..s_x+d_x+1)
    }
}

pub fn part1(input: &ParsedInput, y: i32) -> impl Display {
    let mut ranges = input.into_iter()
    .filter_map(
        |(s, b)| get_range(s, b, y)
    )
    .collect::<Vec<Range<i32>>>();

    ranges.sort_unstable_by(|a, b| match a.start.cmp(&b.start) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => a.end.cmp(&b.end),
    });

    // Filter out ranges that 'swallow' other ranges
    let mut j = 0;
    while j < ranges.len() {
        let mut i = j + 1;
        while i < ranges.len() {
            if ranges[j].end >= ranges[i].end {
                ranges.remove(i);
            } else {
                i += 1;
            }
        }
        j += 1;
    }

    // Compute how many ranges contain a beacon
    let penalties = input.into_iter()
    .map(|(_s, b)| b)
    .unique()
    .filter(|b| b.1 == y)
    .filter_map(|b| 
        ranges.iter().find(|r|r.contains(&b.0))
    ).count();


    ranges.iter()
    .zip(ranges.iter().skip(1))
    .fold(0, |acc, (left, right)|
        if left.end > right.start { // There is overlap!
            acc + (left.start..right.start).len()
        } else { // No overlap, all fine
            acc + left.len() + 1
        }
    ) + ranges.last().unwrap().len() - penalties
}

pub fn part2(input: &ParsedInput, max_x: i32, max_y: i32) -> impl Display {
    for y in 0..=max_y {

        let mut ranges = input.into_iter()
        .filter_map(
            |(s, b)| get_range(s, b, y)
        )
        .collect::<Vec<Range<i32>>>();

        ranges.sort_unstable_by(|a, b| match a.start.cmp(&b.start) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => a.end.cmp(&b.end),
        });

        ranges.iter_mut().for_each(|r| {
            if r.start < 0 {
                r.start = 0;
            } 
            if r.end > max_x {
                r.end = max_x;
            }
        });

        // Merge ranges if possible
        let mut left = ranges[0].clone();
        for right in &ranges[1..] {
            // If we can merge two ranges, do it
            if right.start <= left.end && right.end > left.end {
                left.end = right.end;
            // If we cannot merge, we've found a gap, so return the answer
            } else if right.start > left.end {
                // println!("Found gap at ({}, {})!", left.end, y);
                return (left.end as isize) * 4000000 + (y as isize)
            }
        }

    }

    // format!("Part2 not implemented!")
    -1
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    // Parse sample and challenge input
    let sample = parse_input(&read_file("sample"));
    let input = parse_input(&read_file("input"));

    // Part 1
    // Define sample answer
    let sample_answer_part_1 = "26";

    // Evaluate sample input, and compare with sample answer
    assert_eq!(format!("{}", part1(&sample, 10)), sample_answer_part_1);

    // If sample input evaluated correctly, print output of part 1 with 
    // challenge output.
    formatted_print("1", part1(&input, 2_000_000)); // it's not 4847275

    // Part 2
    // Define sample answer
    let sample_answer_part_2 = "56000011";

    // Evaluate sample input, and compare with sample answer
    assert_eq!(format!("{}", part2(&sample, 20, 20)), sample_answer_part_2);

    // If sample input evaluated correctly, print output of part 1 with 
    // challenge output.
    formatted_print("2", part2(&input, 4_000_000, 4_000_000));

}

pub fn read_file(file_name: &str) -> String {
    return std::fs::read_to_string(file_name).expect(format!("File {} not found", file_name).as_str());
}

fn formatted_print<T : Display>(part: &str, output: T) {
    println!("==================== Part {} ======================", part);
    println!("{}", output);
    println!("==================================================");
}