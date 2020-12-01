use std::fs::File;
use std::io::prelude::*;

static SUM: i32 = 2020;

// return the last group of 3 numbers found and the count of group meets the requirement
// there should be exactly 1 group to be found, otherwise print error message
fn find_match(contents: String) -> (i32, i32, i32, usize) {
    let vec: Vec<i32> = contents.lines().filter_map(|s| s.parse().ok()).collect();
    let mut out = (0, 0, 0, 0);
    for first in 0..vec.len() {
        for second in first + 1..vec.len() {
            for third in second + 1..vec.len() {
                if vec[first] + vec[second] + vec[third] == SUM {
                    out.3 += 1;
                    out.0 = vec[first];
                    out.1 = vec[second];
                    out.2 = vec[third];
                    println!("{:#?}", out);
                }
            }
        }
    }

    out
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("../input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let (x, y, z, i) = find_match(contents);
    if i != 1 {
        println!("Invaid input found {}", i);
    } else {
        println!("{}", x * y * z);
    }

    Ok(())
}
