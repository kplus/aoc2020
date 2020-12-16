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
    // return false if there is no overlap.
    // it can't return early as it has to update all valid ranges
    fn update(&self, valid: &mut Vec<RANGE>) -> bool {
        let mut overlap = false;
        for v in valid {
            if !self.no_overlaps(v) {
                if self.includes(v.min) {
                    v.min = self.min;
                }
                if self.includes(v.max) {
                    v.max = self.max;
                }
                overlap = true;
            }
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

fn get_rules(data: String, re: &Regex) -> (Vec<RULE>, Vec<RANGE>) {
    let mut rules: Vec<RULE> = Vec::new();
    let mut valid_range: Vec<RANGE> = Vec::new();
    for line in data.lines() {
        if line.starts_with("your") {
            break;
        }
        rules.push(RULE::from_str(line.to_string(), &re));
        //println!("current rule is {:#?}", rules);
    }
    // update the valid range as we are going to check against it with nearby tickets
    for range in rules.iter().map(|r| r.get_ranges()).flatten() {
        if !range.update(&mut valid_range) {
            valid_range.push(*range);
        }
    }
    (rules, valid_range)
}

fn filter_invalid(data: String, valid_range: Vec<RANGE>) -> Vec<usize> {
    let mut invalid = Vec::new();
    for line in data.lines().skip(1) {
        for num in line.split(',').map(|n| n.parse().unwrap()) {
            let mut outside = true;
            for r in &valid_range {
                if r.includes(num) {
                    outside = false;
                    break;
                }
            }
            if outside {
                invalid.push(num);
            }
        }
    }
    invalid
}
fn question2(data: Vec<String>) -> Result<usize, &'static str> {
    let re = Regex::new(r"\d+-\d+").unwrap();
    let (rules, valid_range) = get_rules(data[0].to_owned(), &re);
    let my_ticket: Vec<usize> = data[1]
        .lines()
        .skip(1)
        .collect::<String>()
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect();
    //println!("my ticket is {:?}", my_ticket);

    Err("Cannot find second number.")
}

fn question1(data: Vec<String>) -> Result<usize, &'static str> {
    let re = Regex::new(r"\d+-\d+").unwrap();
    let (_rules, valid_range) = get_rules(data[0].to_owned(), &re);
    let invalid = filter_invalid(data[2].to_owned(), valid_range);

    Ok(invalid.iter().sum())
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file_by_p()?;
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
        let data: Vec<String> = TEST_INPUT
            .split("\n\n")
            .map(|s| s.trim().to_string())
            .collect();
        println!("data is {:#?}", data);
        assert_eq!(Ok(71), question1(data));
    }
    #[test]
    fn test_question2() {
        let data: Vec<String> = TEST_INPUT2.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Ok(0), question2(data));
    }
}
