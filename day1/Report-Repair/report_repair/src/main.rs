use std::fs::File;
use std::io::prelude::*;

static SUM: i32 = 2020;

fn find_match(contents: String) -> (i32, i32) {
    let vec: Vec<i32> = contents.lines().filter_map(|s| s.parse().ok()).collect();

    for first in 0..vec.len() {
        for second in first + 1..vec.len() {
            if vec[first] + vec[second] == SUM {
                return (vec[first], vec[second]);
            }
        }
    }
    (0, 0)
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("../input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let (x, y) = find_match(contents);
    println!("{}", x * y);
    Ok(())
}
