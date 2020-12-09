use std::error::Error;

use aoc2020::*;
// Question 1 uses find_end, and question 2 uses find_bug
fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file("input.txt")?;
    println!("{:#?}", data);

    Ok(())
}
