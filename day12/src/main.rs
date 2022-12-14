use std::fmt::{Display, Debug};
use std::collections::{HashMap, BinaryHeap};
use std::ops::Index;

// ========================= Challenge Logic ============================
// Define your own output type here for the `parse_input` function.


#[derive(Clone, Debug)]
pub struct Graph<const N: usize> {
    nodes: [i32; N],
    rev_edges: [[Option<(usize, i32)>; 4]; N],
    start: usize,
    end: usize,
}

impl<const N: usize> Graph<N> {

    pub fn new() -> Graph<N> {
        Graph { nodes: [0; N], 
            rev_edges: [[None; 4]; N], start: 0, end: 0 }
    }

    pub fn add_nodes(&mut self, vals: &[i32]) {
        for (i, &val) in vals.iter().enumerate() {
            let val_i32 = if val == 83 {
                self.start = i;
                0
            } else if val == 69 {
                self.end = i;
                25
            } else {
                val - 97
            };
            self.nodes[i] = val_i32;
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: i32, i: usize) {
        self.rev_edges[to][i] = Some((from, weight));
        // push((from, weight));
    }

    pub fn shortest_paths(&self) -> Vec<i32> {
        let mut dist = vec![i32::MAX; self.nodes.len()];
        dist[self.end] = 0;

        let mut queue: PriorityQueue<usize> = PriorityQueue::with_capacity(64);
        queue.push(self.end, 0);

        while let Some(u) = queue.head() {
            for neighbour in self.rev_edges[u].iter() {
                if let Some((from, weight)) = neighbour {
                    let alt = dist[u] + weight;
                    if alt < dist[*from] {
                        dist[*from] = alt;
                        queue.push(*from, alt);
                    }
                }
            }
        }

        // println!("{:?}", dist);
        dist
    }
}

impl<const N: usize> Index<usize> for Graph<N> {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

#[derive(Debug)]
pub struct QueueItem<T> {
    item: T,
    priority: i32
}

impl<T> QueueItem<T> {
    pub fn new(item: T, priority: i32) -> QueueItem<T> {
        QueueItem { item, priority }
    }

    pub fn unwrap(self) -> T {
        self.item
    }
}

impl<T> Ord for QueueItem<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl<T> PartialOrd for QueueItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match other.priority.partial_cmp(&self.priority) {
            ord => return ord,
        }
    }
}

impl<T> PartialEq for QueueItem<T> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<T> Eq for QueueItem<T> {}

#[derive(Debug)]
pub struct PriorityQueue<T> {
    items: BinaryHeap<QueueItem<T>>
}

impl<T> PriorityQueue<T> {

    pub fn new() -> PriorityQueue<T> {
        PriorityQueue { items: BinaryHeap::new() }
    }

    pub fn with_capacity(capacity: usize) -> PriorityQueue<T> {
        PriorityQueue { items: BinaryHeap::with_capacity(capacity) }
    }

    pub fn push(&mut self, val: T, priority: i32) {
        self.items.push(QueueItem::new(val, priority))
    }
    
    pub fn head(&mut self) -> Option<T> {
        self.items.pop().map(|qi| qi.unwrap())
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

type ParsedInput<const N: usize> = Graph<N>;

pub fn parse_input<const N: usize>(input: &str) -> ParsedInput<N> {
    let mut g = Graph::<N>::new();

    let nodes: [i32; N] = input.lines().map(|l|
        l.as_bytes().iter().map(|&b|
            b as i32
        )
        .collect::<Vec<i32>>()
    )
    .flatten()
    .collect::<Vec<i32>>().try_into().unwrap();

    g.add_nodes(&nodes);

    let h = input.lines().count();
    let w = N / h;

    for j in 0..h {
        for i in 0..w {
            if j != 0 && (g[(j-1) * w + i] - g[j * w + i]) < 2 {
                // println!("({}, {}) -> ({}, {})", i, j, i, j + 1);
                g.add_edge(j * w + i, (j-1) * w + i, 1, 0)
            }

            if j != h - 1 && (g[(j+1) * w + i] - g[j * w + i]) < 2 {
                // println!("({}, {}) -> ({}, {})", i, j, i, j + 1);
                g.add_edge(j * w + i, (j+1) * w + i, 1, 2)
            }

            if i != 0 && (g[j * w + i - 1] - g[j * w + i]) < 2 {
                // println!("({}, {}) -> ({}, {})", i, j, i, j + 1);
                g.add_edge(j * w + i, j * w + i - 1, 1, 1)
            }

            if i != w - 1 && (g[j * w + i + 1] - g[j * w + i]) < 2 {
                // println!("({}, {}) -> ({}, {})", i, j, i, j + 1);
                g.add_edge(j * w + i, j * w + i + 1, 1, 3)
            }
        }
    }
    
    g
}

pub fn part1<const N: usize>(g: &ParsedInput<N>) -> impl Display {
    g.shortest_paths()[g.start]
}

pub fn part2<const N: usize>(g: &ParsedInput<N>) -> impl Display {
    g.shortest_paths().into_iter()
    .enumerate()
    .filter(|&(i, _e)| g[i] == 0)
    .map(|(_i, e)| e)
    .min().unwrap()
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    // Parse sample and challenge input
    let sample = parse_input::<40>(&read_file("sample"));
    let input = parse_input::<6847>(&read_file("input"));

    // Part 1
    // Define sample answer
    let sample_answer_part_1 = "31";

    // Evaluate sample input, and compare with sample answer
    assert_eq!(format!("{}", part1(&sample)), sample_answer_part_1);

    // If sample input evaluated correctly, print output of part 1 with 
    // challenge output.

    formatted_print("1", part1(&input));

    // Part 2
    // Define sample answer
    let sample_answer_part_2 = "29";

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