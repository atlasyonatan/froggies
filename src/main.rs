use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
struct Platform<I, H> {
    position: I,
    height: H,
}

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(path).unwrap();
    let mut platforms = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .flat_map(|s| s.parse::<i32>())
        .enumerate()
        .map(|(position, height)| Platform { height, position });
    let mut paths = Vec::new();

    let mut start = platforms.next().expect("input contains no platforms");
    let mut end = start;
    let mut local_minima = start.height;
    for platform in platforms {
        match platform.height.cmp(&local_minima) {
            Ordering::Less | Ordering::Equal => local_minima = platform.height,
            Ordering::Greater => match platform.height.cmp(&end.height) {
                Ordering::Greater | Ordering::Equal => {}
                Ordering::Less => {
                    paths.push((start, end, local_minima));
                    start = end;
                    local_minima = start.height;
                }
            },
        }
        end = platform;
    }
    paths.push((start, end, local_minima));

    let (start, end, local_minima) = paths
        .iter()
        .max_by_key(|(start, end, _)| end.position - start.position)
        .unwrap();
    let longest_distance = end.position - start.position;
    println!(
        "Found a longest distance between frogs {}, from {:?} to {:?}. Frogs should begin at a local minima of {} in that region.",
        longest_distance, start, end, local_minima
    )
}
