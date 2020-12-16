use std::error::Error;

use aoc2020::*;
use regex::Regex;

enum SETCTIONS {
    Rules,
    YourTicket,
    NearbyTicket,
}

#[derive(Debug, Clone, Copy)]
struct RANGE {
    min: usize,
    max: usize,
}
impl RANGE {
    // Fill RANGE with a string matches pattern 'dd-dd'
    fn from_str(s: &str) -> Self {
        let r: Vec<usize> = s.split('-').map(|c| c.parse().unwrap()).collect();
        RANGE {
            min: r[0],
            max: r[1],
        }
    }

    fn no_overlaps(&self, valid: &RANGE) -> bool {
        valid.max < self.min || valid.min > self.max
    }

    fn includes(&self, n: usize) -> bool {
        n <= self.max && n >= self.min
    }
    // Update overlapped valid range with current range
    // return false if there is no overlap
    fn update(&self, valid: &mut Vec<RANGE>) -> bool {
        let mut overlap = false;
        for v in valid {
            if self.no_overlaps(v) {
                continue;
            } else if v.min < self.min && v.max >= self.min && v.max < self.max {
                v.max = self.max;
            } else if v.min >= self.min && v.max <= self.max {
                v.min = self.min;
                v.max = self.max;
            } else if v.min > self.min && v.min <= self.max && v.max > self.max {
                v.min = self.min;
            }
            overlap = true;
        }
        overlap
    }
}

#[derive(Debug)]
struct RULE {
    field: String,      // string of filed name
    ranges: Vec<RANGE>, // list of valid ranges
}

impl RULE {
    // Fill a RULE structure from string
    fn from_str(s: String, re: &Regex) -> Self {
        //println!("Filling rule with string {:#?}", s);
        RULE {
            field: String::from(s.split(':').next().unwrap()),
            ranges: re
                .captures_iter(s.as_str())
                .map(|c| RANGE::from_str(&c[0]))
                .collect(),
        }
    }

    fn get_ranges(&self) -> &Vec<RANGE> {
        &self.ranges
    }
}

fn question2(data: Vec<String>) -> Result<usize, &'static str> {
    Err("Cannot find second number.")
}

fn question1(data: Vec<String>) -> Result<usize, &'static str> {
    let mut error_rate = 0;
    let mut rules: Vec<RULE> = Vec::new();
    let mut section = SETCTIONS::Rules;
    let mut valid_range: Vec<RANGE> = Vec::new();
    let mut my_ticket: Vec<usize> = Vec::new();

    let re = Regex::new(r"\d+-\d+").unwrap();
    for line in data {
        if line.is_empty() {
            continue;
        }
        match section {
            SETCTIONS::Rules => {
                if line.starts_with("your") {
                    section = SETCTIONS::YourTicket;
                    continue;
                }
                rules.push(RULE::from_str(line, &re));
                //println!("current rule is {:#?}", rules);
            }
            SETCTIONS::YourTicket => {
                if line.starts_with("nearby") {
                    section = SETCTIONS::NearbyTicket;

                    // update the valid range as we are going to check against it with nearby tickets
                    for range in rules.iter().map(|r| r.get_ranges()).flatten() {
                        // println!("range to check is: {:?}", range);
                        if !range.update(&mut valid_range) {
                            valid_range.push(*range);
                        }
                    }
                    //println!("The valid_range is {:#?}", valid_range);
                    continue;
                }
                my_ticket = line
                    .split(',')
                    .map(|c| c.parse::<usize>().unwrap())
                    .collect();
                println!("my ticket is {:?}", my_ticket);
            }
            SETCTIONS::NearbyTicket => {
                for num in line.split(',').map(|n| n.parse().unwrap()) {
                    let mut outside = true;
                    for r in &valid_range {
                        if r.includes(num) {
                            outside = false;
                            break;
                        }
                    }
                    if outside {
                        error_rate += num;
                    }
                }
            }
        }
    }

    Ok(error_rate)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file()?;
    //println!("{:#?}", data);
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

    static TEST_INPUT2: &str = r"class: 0-1 or 4-19
    row: 0-5 or 8-19
    seat: 0-13 or 16-19
    
    your ticket:
    11,12,13
    
    nearby tickets:
    3,9,18
    15,1,5
    5,14,9";

    #[test]
    fn test_question1() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Ok(71), question1(data));
    }
    #[test]
    fn test_question2() {
        let data: Vec<String> = TEST_INPUT2.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Ok(0), question2(data));
    }
}
