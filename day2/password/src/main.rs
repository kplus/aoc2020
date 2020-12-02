use std::fs::File;
use std::io::prelude::*;

// return whether the password is valid
// also print out the valid password string
fn check_pass(line: &str) -> bool {
    let whole: Vec<&str> = line.split(' ').collect();
    let range: Vec<&str> = whole[0].split('-').collect();
    let ch: char = whole[1].chars().next().unwrap();

    let m = whole[2].matches(ch).count();
    if m >= range[0].parse::<usize>().unwrap() && m <= range[1].parse::<usize>().unwrap() {
        println!("Find valid password line: {}", line);
        return true;
    }
    false
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("../input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut count = 0;
    for line in contents.lines() {
        if check_pass(line) {
            count += 1;
        };
    }

    println!("There are {} valid passwords.", count);
    Ok(())
}
