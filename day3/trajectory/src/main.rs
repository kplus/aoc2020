use std::fs::File;
use std::io::prelude::*;

// change function call to check_pass for question 1
// and change it to check_pass2 for question 2
fn main() -> std::io::Result<()> {
    let mut file = File::open("../input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    for line in contents.lines() {
        println!("{}", line);
    }

    Ok(())
}
