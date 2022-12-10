use std::fmt::Display;
use std::collections::HashMap;

// ========================= Challenge Logic ============================
// Define your own output type here for the `parse_input` function.

#[derive(Debug)]
pub enum DirEntry {
    Directory(String),
    File(String, u64)
}

type Path = String;
type ParsedInput = HashMap<Path, Vec<DirEntry>>;

pub fn parse_input(input: &str) -> ParsedInput {
    // Create result directory
    let mut result: ParsedInput = HashMap::with_capacity(256);
    let mut path = String::from("");
    
    // Loop through input; match on first part of line
    for l in input.lines() {
        match l.split_once(" ") {
            // In case of `cd /`, change path to ""
            Some(("$", "cd /")) => path = String::from(""),
            // In case of `cd ..`, split path on right-most '/', and set path to left side of split
            Some(("$", "cd ..")) => path = format!("{}", path.rsplit_once("/").unwrap().0),
            // In case of `ls`, do nothing
            Some(("$", "ls")) => (), // ls command, do nothing for now
            // In case of `cd [dirname]`, set path to current_path/[dirname]
            Some(("$", cd_dirname)) => path = format!("{}/{}", path, &cd_dirname[3..]),
            // In case we see a dir entry (after a `$ ls`), add a directory entry
            // to the result hashmap with the path of the directory
            Some(("dir", dirname)) => {
                result.entry(path.clone())
                .or_default()
                .push(DirEntry::Directory(format!("{}/{}", path, dirname)));
            }
            // In case we see a file entry (after a `$ ls`), add a file entry
            // to the result hashmap with the path of the directory
            Some((size, filename)) => {
                result.entry(path.clone())
                .or_default()
                .push(DirEntry::File(format!("{}/{}", path, filename), size.parse::<u64>().unwrap()));
            },
            // In case our split_once could not split do nothing
            None => println!("Nothing here"),
        };
    }
    // println!("{}", result.len());
    
    result
}

pub fn rec_compute_sizes(fs: &ParsedInput, path: &str, cache: &mut HashMap<String, u64>) -> u64 {
    // Compute size of directory `path` in fs.
    let size = fs.get(path).map_or(0, |entries|
    entries.into_iter().map(|e| 
        match e {
            DirEntry::Directory(d) => rec_compute_sizes(fs, d, cache),
            DirEntry::File(_p, s) => *s,
        }
    ).sum::<u64>());

    // Insert size into cache
    cache.insert(path.into(), size);

    // Return size
    size
}

pub fn compute_sizes(fs: &ParsedInput) -> HashMap<String, u64> {
    // Create cache with capacity
    let mut cache = HashMap::with_capacity(256);
    
    // Fill cache recursively
    rec_compute_sizes(fs, "", &mut cache);
    
    // Return cache
    cache
}

pub fn part1(input: &ParsedInput) -> impl Display {
    // Compute cache
    let cache = compute_sizes(input);

    // Loop over directories & sizes
    cache.into_iter()
    // Forget about directory path
    .map(|(_k, v)| v)
    // filter out directories larger than 100.000
    .filter(|&s| s <= 100_000)
    // sum up sizes
    .sum::<u64>()
}

pub fn part2(input: &ParsedInput) -> impl Display {
    // Compute cache
    let cache = compute_sizes(input);

    // Compute space we need to free up
    let fs_size = 70000000;
    let space_needed = 30000000;

    let space_available = fs_size - cache[""];
    let to_free = space_needed - space_available;

    // Loop over directories & sizes
    cache.into_iter()
    // Forget about directory path
    .map(|(_k, v)| v)
    // filter out directories that don't free up enough space
    .filter(|&s| s >= to_free)
    // Get the smallest directory size that frees up enough space
    .min().unwrap()
}

// =========================== Main Function ============================
#[allow(dead_code)]
fn main() {
    // Parse sample and challenge input
    let sample = parse_input(&read_file("sample"));

    // Part 1
    // Define sample answer
    let sample_answer_part_1 = "95437";

    // Evaluate sample input, and compare with sample answer
    assert_eq!(format!("{}", part1(&sample)), sample_answer_part_1);

    let input = parse_input(&read_file("input"));

    // If sample input evaluated correctly, print output of part 1 with 
    // challenge output.
    formatted_print("1", part1(&input));

    // Part 2
    // Define sample answer
    let sample_answer_part_2 = "24933642";

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