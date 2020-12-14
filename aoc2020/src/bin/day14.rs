use std::collections::HashMap;
use std::error::Error;

use aoc2020::*;

// Only used in question 2
fn update_mem2(s: String, mem: &mut HashMap<u64, u64>, mask: &str) {
    let to_update: Vec<u64> = s // Vec of 2 elements, first to be postion, second to be value
        .split(|c| c == '[' || c == ']' || c == ' ')
        .filter_map(|c| c.parse().ok())
        .collect();

    let mut address_base = to_update[0];
    let mut x_position = Vec::new();
    for (i, c) in mask.chars().rev().enumerate() {
        if c == 'X' {
            x_position.push(i);
            address_base &= !(1 << i);
        } else {
            address_base |= (c.to_digit(10).unwrap() as u64) << i;
        }
    }

    // get list of all possible addresses
    let mut addresses = Vec::new();
    addresses.push(address_base);
    for i in x_position {
        let mut double: Vec<u64> = addresses.iter().map(|v| *v | 1 << i).collect();
        addresses.append(&mut double);
    }
    //println!("Finally addresses list is {:#?}", addresses);

    for a in addresses {
        mem.insert(a, to_update[1]);
    }
}

fn question2(data: Vec<String>) -> Result<u64, &'static str> {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask = String::new();

    for line in data {
        if line.starts_with("mask") {
            mask = line.split_whitespace().nth(2).unwrap().to_string();
        } else {
            update_mem2(line, &mut mem, &mask);
        }
    }

    let sum = mem.values().sum();
    Ok(sum)
}

// Update memeory based on current line String
// Only used in questions 1
// [in]     String contains memory location and value to update
// [out]    Update hashmap with all memory values in
fn update_mem(s: String, mem: &mut HashMap<u64, u64>, mask_one: &u64, mask_zero: &u64) {
    let to_update: Vec<u64> = s
        .split(|c| c == '[' || c == ']' || c == ' ')
        .filter_map(|c| c.parse().ok())
        .collect();

    let value = (to_update[1] | mask_one) & mask_zero;
    mem.insert(to_update[0], value);
}

// Update mask one/zero accroding to current line String
// Only used in question 1
// [in]     String contains mask to update
// [out]    2 seperate masks for one and zero
fn update_masks(s: String) -> (u64, u64) {
    const BITS_36: u64 = 0xFFFFFFFFF; // 36 bits mask
    let mask = s.split_whitespace().nth(2).unwrap();
    let mut mask_one = 1;
    let mut mask_zero = 1;

    for c in mask.chars() {
        mask_one <<= 1;
        mask_zero <<= 1;
        match c {
            '1' => {
                mask_one += 1;
                mask_zero += 1;
            }
            'X' => mask_zero += 1,
            _ => {}
        }
    }

    (mask_one & BITS_36, mask_zero & BITS_36)
}
fn question1(data: Vec<String>) -> Result<u64, &'static str> {
    let mut mask_one = 0;
    let mut mask_zero = 0;
    let mut mem: HashMap<u64, u64> = HashMap::new();

    for line in data {
        if line.starts_with("mask") {
            let (one, zero) = update_masks(line);
            mask_one = one;
            mask_zero = zero;
        } else {
            update_mem(line, &mut mem, &mask_one, &mask_zero);
        }
    }
    let sum = mem.values().sum();
    Ok(sum)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file()?;
    //println!("{:#?}", data);
    match question1(data.to_owned()) {
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

    static TEST_INPUT: &str = r"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    mem[8] = 11
    mem[7] = 101
    mem[8] = 0
    mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX01
    mem[10] = 11
    mem[5] = 101
    mem[10] = 0";

    static TEST_INPUT2: &str = r"mask = 000000000000000000000000000000X1001X
    mem[42] = 100
    mask = 00000000000000000000000000000000X0XX
    mem[26] = 1";

    #[test]
    fn test_question1() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Ok(267), question1(data));
    }
    #[test]
    fn test_question2() {
        let data: Vec<String> = TEST_INPUT2.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Ok(208), question2(data));
    }
}
