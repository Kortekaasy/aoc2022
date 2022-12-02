use std::fmt::Display;

mod main;

pub fn part1() -> impl Display {
    let raw_input = include_str!("../input").to_owned();
    let parsed_input = main::parse_input(&raw_input);
    main::part1(&parsed_input)
}

pub fn part2() -> impl Display {
    let raw_input = include_str!("../input").to_owned();
    let parsed_input = main::parse_input(&raw_input);
    main::part2(&parsed_input)
}