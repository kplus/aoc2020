use std::error::Error;

fn question2() -> Result<usize, &'static str> {
    Err("Cannot find the window to fit target number.")
}

fn question1() -> Result<usize, &'static str> {
    Err("Cannot find first number.")
}
use aoc2020::*;
fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file()?;
    println!("{:#?}", data);
    match question1() {
        Ok(x) => {
            println!("The result for question 1 is {}", x);
        }
        Err(x) => println!("Error processing the input data: {:?}", x),
    };
    match question2() {
        Ok(x) => {
            println!("The sequency from position {}", x);
        }
        Err(x) => println!("Error processing the input data: {:?}", x),
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"";

    #[test]
    fn test_question1() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Ok([22, 0, 10]), question1());
    }
    #[test]
    fn test_question2() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Ok(19208), question2());
    }
}
