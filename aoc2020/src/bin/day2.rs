use std::fs::File;
use std::io::prelude::*;

// return whether the password is valid
// also print out the valid password string
fn _check_pass(line: &str) -> bool {
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

// return whether the password is valid
// also print out the valid password string
// for question 2
fn check_pass2(line: &str) -> bool {
    let whole: Vec<_> = line.split(|c| c == ' ' || c == ':' || c == '-').collect();
    let first: usize = whole[0].parse().unwrap();
    let second: usize = whole[1].parse().unwrap();
    let first_c = whole[4].chars().nth(first - 1).unwrap();
    let second_c = whole[4].chars().nth(second - 1).unwrap();
    let ch: char = whole[2].chars().next().unwrap();

    if first_c == ch {
        if second_c != ch {
            println!("Find valid password line: {}", line);
            return true;
        }
    } else if second_c == ch {
        println!("Find valid password line: {}", line);
        return true;
    }
    false
}

// change function call to check_pass for question 1
// and change it to check_pass2 for question 2
fn main() -> std::io::Result<()> {
    let mut file = File::open("../inputs/day2")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut count = 0;
    for line in contents.lines() {
        if check_pass2(line) {
            count += 1;
        };
    }

    println!("There are {} valid passwords.", count);
    Ok(())
}
