use std::collections::HashMap;
use std::error::Error;

use aoc2020::*;

fn question2(data: Vec<String>) -> Result<usize, &'static str> {
    Err("Cannot find second number.")
}

//todo: Update memeory based on current line String
// [in]     String contains memory location and value to update
// [out]    Update hashmap with all memory values in
fn update_mem(s: String, mem: &mut HashMap) {}

//todo: Update mask one/zero accroding to current line String
// [in]     String contains mask to update
// [out]    2 seperate masks for one and zero
fn update_masks(s: String) -> (u64, u64) {
    (0, 0)
}
fn question1(data: Vec<String>) -> Result<usize, &'static str> {
    let mut mask_one = 0;
    let mut mask_zero = 0;
    let mut mem = HashMap::new();

    for line in data {
        if line.starts_with("mask") {
            let (one, zero) = update_masks(line);
            mask_one = one;
            mask_zero = zero;
        } else {
            update_mem(line, &mem);
        }
    }
    let sum = mem.values().sum();
    Ok(sum)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file()?;
    println!("{:#?}", data);
    match question1(data.to_owned()) {
        Ok(x) => println!("The result for question 1 is {}", x),
        Err(x) => eprintln!("Error processing the input data: {:?}", x),
    };
    match question2(data) {
        Ok(x) => println!("The sequency from position {}", x),
        Err(x) => eprintln!("Error processing the input data: {:?}", x),
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    mem[8] = 11
    mem[7] = 101
    mem[8] = 0";

    #[test]
    fn test_question1() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Ok(165), question1(data));
    }
    #[test]
    fn test_question2() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Err("Cannot find second number."), question2(data));
    }
}
