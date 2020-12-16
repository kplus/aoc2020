use std::error::Error;

use aoc2020::*;

enum SETCTIONS {
    Rules,
    YourTicket,
    NearbyTicket,
}

struct RULE {
    field: String,               // string of filed name
    ranges: Vec<(usize, usize)>, // list of valid ranges
}

impl RULE {
    //todo: Fill a RULE structure from string
    fn from_str(s: String) -> Self {
        RULE {
            field: String::from("new"),
            ranges: Vec::new(),
        }
    }
}

fn question2(data: Vec<String>) -> Result<usize, &'static str> {
    Err("Cannot find second number.")
}

//todo: Check if there is invalid value and return if found
fn find_invalid() -> usize {
    0
}

fn question1(data: Vec<String>) -> Result<usize, &'static str> {
    let mut error_rate = 0;
    let mut rules: Vec<RULE> = Vec::new();
    let mut section = SETCTIONS::Rules;
    //let mut my_ticket: Vec<usize> = Vec::new();

    for line in data {
        match section {
            SETCTIONS::Rules => {
                if line.starts_with("your") {
                    section = SETCTIONS::YourTicket;
                    continue;
                }
                rules.push(RULE::from_str(line));
            }
            SETCTIONS::YourTicket => {
                if line.starts_with("nearby") {
                    section = SETCTIONS::NearbyTicket;
                }
                /* don't need to use my ticket in question 1
                my_ticket = line
                    .split(',')
                    .map(|c| c.parse::<usize>().unwrap())
                    .collect();
                */
            }
            SETCTIONS::NearbyTicket => {
                //todo: figure out a good algorithm to find invalid field
                if true {
                    error_rate += find_invalid();
                }
            }
        }
    }

    Ok(error_rate)
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

    static TEST_INPUT: &str = r"class: 1-3 or 5-7
    row: 6-11 or 33-44
    seat: 13-40 or 45-50
    
    your ticket:
    7,1,14
    
    nearby tickets:
    7,3,47
    40,4,50
    55,2,20
    38,6,12";

    #[test]
    fn test_question1() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Ok(71), question1(data));
    }
    #[test]
    fn test_question2() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Err("Cannot find second number."), question2(data));
    }
}
