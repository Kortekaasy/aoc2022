use std::fmt::Display;

// ========================= Challenge Logic ============================
// Define your own output type here for the `parse_input` function.
const NORTH: usize = 0;
const EAST: usize = 1;
const SOUTH: usize = 2;
const WEST: usize = 3;

#[derive(Copy, Clone, Debug)]
pub struct Tree {
    pub height: u8,
    pub heights: [u8; 4],
    pub visible: [bool; 4],
}

impl Tree {
    pub fn from_u8(h: u8) -> Tree {
        Tree {
            height: h,
            heights: [h; 4],
            visible: [false; 4],
        }
    }
}

type ParsedInput<const N: usize> = [[Tree; N]; N];

pub fn parse_input<const N: usize>(input: &str) -> ParsedInput<N> {
    input.lines()
    .map(|l| 
        l.as_bytes().into_iter()
        .map(|&c| Tree::from_u8(c - b'0'))
        .collect::<Vec<Tree>>().try_into().expect("Expected [Tree; N]")
    )
    .collect::<Vec<[Tree; N]>>().try_into().expect("Expected [[Tree; N]; N]")
}


pub fn part1<const N: usize>(input: &ParsedInput<N>) -> impl Display {
    // Compute which trees are visible from which directions
    let mut field = input.clone();
    let h = input.len();
    let w = input[0].len();

    // north to south
    // First row
    for i in 0..w {
        field[0][i].visible[NORTH] = true;
    }

    for j in 1..h { 
        for i in 0..w {
            field[j][i].visible[NORTH] = field[j-1][i].heights[NORTH] < field[j][i].height;
            field[j][i].heights[NORTH] = field[j-1][i].heights[NORTH].max(field[j][i].height);
        }
    }

    // south to north
    // First row
    for i in 0..w {
        field[h-1][i].visible[SOUTH] = true;
    }

    for j in (0..h-1).rev() { 
        for i in 0..w {
            field[j][i].visible[SOUTH] = field[j+1][i].heights[SOUTH] < field[j][i].height;
            field[j][i].heights[SOUTH] = field[j+1][i].heights[SOUTH].max(field[j][i].height);
        }
    }

    // west to east
    // First row
    for j in 0..h {
        field[j][0].visible[WEST] = true;
    }

    for j in 0..h { 
        for i in 1..w {
            field[j][i].visible[WEST] = field[j][i-1].heights[WEST] < field[j][i].height;
            field[j][i].heights[WEST] = field[j][i-1].heights[WEST].max(field[j][i].height);
        }
    }

    // east to west
    // First row
    for j in 0..h {
        field[j][w-1].visible[EAST] = true;
    }

    for j in 0..h { 
        for i in (0..w-1).rev() {
            field[j][i].visible[EAST] = field[j][i+1].heights[EAST] < field[j][i].height;
            field[j][i].heights[EAST] = field[j][i+1].heights[EAST].max(field[j][i].height);
        }
    }

    // Count the number of visible trees
    field.into_iter()
    .map(|r| 
        r.into_iter()
        .filter(|t| 
            t.visible[NORTH] | t.visible[EAST] | t.visible[SOUTH] | t.visible[WEST]
        )
        .count()
    ).sum::<usize>()
}

pub fn part2<const N: usize>(input: &ParsedInput<N>) -> impl Display {
    let field = input.clone();
    let h = input.len();
    let w = input[0].len();

    let mut scores: [[usize; N]; N] = [[1; N]; N];

    for y in 0..h {
        for x in 0..w {
            // up
            let mut up = 0;
            for j in (0..y).rev() {
                up += 1;
                if field[j][x].height >= field[y][x].height {
                    break;
                }
            }

            // right
            let mut right = 0;
            for i in x+1..w {
                right += 1;
                if field[y][i].height >= field[y][x].height {
                    break;
                }
            }

            // down
            let mut down = 0;
            for j in y+1..h {
                down += 1;
                if field[j][x].height >= field[y][x].height {
                    break;
                }
            }

            // left
            let mut left = 0;
            for i in (0..x).rev() {
                left += 1;
                if field[y][i].height >= field[y][x].height {        
                    break;
                }
            }

            scores[y][x] = up * left * right * down;
        }
    }

    // println!("{}", scores[1][2]);
    // println!("{}", scores[3][2]);
    // Get highest entry in scores array 
    scores.into_iter()
    .map(|r| 
        r.into_iter()
        .max()
        .unwrap()
    ).max().unwrap()
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    // Parse sample and challenge input
    let sample = parse_input::<5>(&read_file("sample"));
    let input = parse_input::<99>(&read_file("input"));

    // Part 1
    // Define sample answer
    let sample_answer_part_1 = "21";

    // Evaluate sample input, and compare with sample answer
    assert_eq!(format!("{}", part1(&sample)), sample_answer_part_1);

    // If sample input evaluated correctly, print output of part 1 with 
    // challenge output.
    formatted_print("1", part1(&input));

    // Part 2
    // Define sample answer
    let sample_answer_part_2 = "8";

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