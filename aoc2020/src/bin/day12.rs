use std::error::Error;

use aoc2020::*;

enum Instructions {
    North(u32),
    South(u32),
    East(u32),
    West(u32),
    Turn(u32),
    Forward(u32),
}
struct VM {
    angle: u32,
    east_position: u32,
    north_position: u32,
}

impl VM {
    fn new() {}

    fn action() {}
}
fn question2() -> Result<usize, &'static str> {
    Err("Cannot find second number.")
}

fn question1(v: Vec<String>) -> Result<usize, &'static str> {
    Err("Cannot find first number.")
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file()?;
    println!("{:#?}", data);
    match question1(data) {
        Ok(x) => {
            println!("The result for question 1 is {}", x);
        }
        Err(x) => eprintln!("Error processing the input data: {:?}", x),
    };
    match question2() {
        Ok(x) => {
            println!("The sequency from position {}", x);
        }
        Err(x) => eprintln!("Error processing the input data: {:?}", x),
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"F10
    N3
    F7
    R90
    F11";

    #[test]
    fn test_question1() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Err("Cannot find first number."), question1());
    }
    #[test]
    fn test_question2() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Err("Cannot find second number."), question2());
    }
}
