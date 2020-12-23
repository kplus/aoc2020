use std::error::Error;

use aoc2020::*;

fn question2(data: &str) -> Result<usize, &'static str> {
    Err("Cannot find second number.")
}

fn question1(data: &str) -> Result<String, &'static str> {
    Err("Cannot find first number.")
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = "538914762";
    println!("{:#?}", data);
    match question1(data) {
        Ok(x) => println!("The result for question 1 is {}", x),
        Err(x) => eprintln!("Error processing the input data: {:?}", x),
    };
    match question2(data) {
        Ok(x) => println!("The result for question 2 is {}", x),
        Err(x) => eprintln!("Error processing the input data: {:?}", x),
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"32415";

    #[test]
    fn test_question1() {
        assert_eq!(Err("Cannot find first number."), question1(TEST_INPUT));
    }
    #[test]
    fn test_question2() {
        assert_eq!(Err("Cannot find second number."), question2(TEST_INPUT));
    }
}
