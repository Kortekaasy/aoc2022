use std::{fmt::Display, ops::Deref, cmp::Ordering};
use pest_derive::Parser;
use pest::{Parser, iterators::Pair};


// ========================= Challenge Logic ============================
// Define your own output type here for the `parse_input` function.
#[derive(Parser)]
#[grammar = "../grammar.pest"]
struct ListPairParser;

#[derive(Clone, Debug)]
pub enum ListItem {
    Integer(i32),
    List(Box<List>)
}

#[derive(Clone, Debug, Default)]
pub struct List {
    items: Vec<ListItem>
}

impl List {
    pub fn new() -> List {
        List { items: Vec::new() }
    }

    pub fn from_item(item: ListItem) -> List {
        List { items: vec![item] }
    }

    pub fn add_integer(&mut self, value: i32) {
        self.items.push(ListItem::Integer(value))
    }

    pub fn add_list(&mut self, list: List) {
        self.items.push(ListItem::List(Box::from(list)))
    }
}

impl Deref for List {
    type Target = [ListItem];

    fn deref(&self) -> &Self::Target {
        &self.items[..]
    }
}

type ParsedInput = Vec<List>;

pub fn parse_list(pair: Pair<Rule>) -> List {
    let mut list = List::new();
    for r in pair.into_inner() {
        match r.as_rule() {
            Rule::num => list.add_integer(r.as_str().parse().expect("Expected num")),
            Rule::list => list.add_list(parse_list(r)),
            Rule::LPAREN |
            Rule::RPAREN |
            Rule::lists |
            Rule::EOI |
            Rule::WHITESPACE |
            Rule::newline => panic!("Found {}", r),
        }
    }

    list
}

pub fn parse_input(input: &str) -> ParsedInput {
    let mut pairs = ListPairParser::parse(Rule::lists, input).unwrap_or_else(|e| panic!("{}", e));
    let root = pairs.next().unwrap();
    root.into_inner().map(|r| parse_list(r)).collect::<Vec<List>>()
}

pub fn rec_comp(l1: &List, l2: &List) -> Ordering {
    for (l, r) in l1.iter().zip(l2.iter()) {
        match (match (l, r) {
            (ListItem::Integer(li), ListItem::Integer(ri)) => li.cmp(&ri),
            (ListItem::Integer(li), ListItem::List(rl)) => rec_comp(&List::from_item(ListItem::Integer(*li)), rl),
            (ListItem::List(ll), ListItem::Integer(ri)) => rec_comp(ll, &List::from_item(ListItem::Integer(*ri))),
            (ListItem::List(ll), ListItem::List(rl)) => rec_comp(ll, rl),
        }) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => (),
        }
    }

    // Ordering::Equal
    l1.len().cmp(&l2.len())
}

pub fn part1(input: &ParsedInput) -> impl Display {
    input.chunks_exact(2)
    .enumerate()
    .filter(|(_i, pair)| rec_comp(&pair[0], &pair[1]) != Ordering::Greater)
    .map(|(i, _pair)| i + 1)
    .sum::<usize>()
}

pub fn part2(input: &ParsedInput) -> impl Display {

    let div1 = List::from_item(ListItem::List(Box::new(List::from_item(ListItem::Integer(2)))));
    let div2 = List::from_item(ListItem::List(Box::new(List::from_item(ListItem::Integer(6)))));
    // let div1 = parse_input("[[2]]").into_iter().next().unwrap();
    // let div2 = parse_input("[[6]]").into_iter().next().unwrap();

    // let mut l = input.iter().chain(&[div1.clone(), div2.clone()]).cloned().collect::<Vec<List>>();
    let mut l = input.clone();
    l.push(div1.clone());
    l.push(div2.clone());
    l.sort_by(|a, b| rec_comp(a, b));

    let vals = l.iter()
    .enumerate()
    .filter(|(_i, l)| 
        rec_comp(l, &div1) == Ordering::Equal || rec_comp(l, &div2) == Ordering::Equal
    )
    .map(|(i, _l)| i )
    .collect::<Vec<usize>>();
    vals[0] * vals[1]
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    // Parse sample and challenge input
    let sample = parse_input(&read_file("sample"));
    let input = parse_input(&read_file("input"));

    // Part 1
    // Define sample answer
    let sample_answer_part_1 = "13";

    // Evaluate sample input, and compare with sample answer
    assert_eq!(format!("{}", part1(&sample)), sample_answer_part_1);

    // If sample input evaluated correctly, print output of part 1 with 
    // challenge output.
    formatted_print("1", part1(&input));

    // Part 2
    // Define sample answer
    let sample_answer_part_2 = "140";

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