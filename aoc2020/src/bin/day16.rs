use std::collections::HashMap;
use std::error::Error;

use aoc2020::*;
use regex::Regex;

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

    fn includes(&self, n: usize) -> bool {
        n <= self.max && n >= self.min
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

    fn obay(&self, num: usize) -> bool {
        let mut ret = false;
        for range in self.ranges.iter() {
            ret = ret || range.includes(num);
        }
        ret
    }
}

fn get_rules(data: String, re: &Regex) -> Vec<RULE> {
    let mut rules: Vec<RULE> = Vec::new();
    for line in data.lines() {
        if line.starts_with("your") {
            break;
        }
        rules.push(RULE::from_str(line.to_string(), &re));
        //println!("current rule is {:#?}", rules);
    }
    rules
}

fn filter_invalid(data: String, rules: Vec<RULE>) -> (Vec<usize>, usize) {
    let mut invalid_lines = Vec::new();
    let mut invalid = 0;
    for (i, line) in data.lines().enumerate().skip(1) {
        for num in line.split(',').map(|n| n.parse().unwrap()) {
            let mut outside = true;
            for r in &rules {
                if r.obay(num) {
                    outside = false;
                    break;
                }
            }
            if outside {
                invalid_lines.push(i - 1);
                invalid += num;
            }
        }
    }
    (invalid_lines, invalid)
}

fn confirm_rule(
    mut matrix: &mut HashMap<String, Vec<usize>>,
    mut positions: &mut HashMap<String, usize>,
    rule_name: String,
    index: usize,
) {
    let change = matrix.get_mut(&rule_name).unwrap();
    change[index] = 0;
    if change.iter().sum::<usize>() == 1 {
        let mut pos = 0;
        for i in change {
            if *i == 0 {
                pos += 1;
            } else {
                break;
            }
        }
        //println!(
        //    "confirmed position {} for rule {} found after {} be removed",
        //    pos, rule_name, index
        //);
        // got a position of rule, update others
        matrix.remove(&rule_name);
        positions.insert(rule_name.to_owned(), pos);

        let matrix_clone = matrix.to_owned();
        for key in matrix_clone.keys() {
            confirm_rule(&mut matrix, &mut positions, key.to_string(), pos);
        }
    }
}
/*
fn question2(data: Vec<String>) -> Result<usize, &'static str> {
    let re = Regex::new(r"\d+-\d+").unwrap();
    let rules = get_rules(data[0].to_owned(), &re);
    let my_ticket: Vec<usize> = data[1]
        .lines()
        .skip(1)
        .collect::<String>()
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect();
    let rule_count = rules.len();
    let mask = vec![1; rule_count];

    // Use a matrix to store masks for all rules, initial all 1's
    let mut matrix: HashMap<String, Vec<usize>> = HashMap::new();
    // Use another hashmap to store confirmed rule position
    let mut positions: HashMap<String, usize> = HashMap::new();
    for rule in &rules {
        matrix.insert(rule.field.to_owned(), mask.to_owned());
    }

    let (invalid_line, _invalid) = filter_invalid(data[2].to_owned(), rules);
    for (i, lines) in data[2].lines().enumerate().skip(1) {
        let index = i - 1;
        if invalid_line.contains(&index) {
            continue;
        }

        // loop through all numbers in a single line
        // check remain rule in matrix
        for (pos, num) in lines
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .enumerate()
        {
            'next_rule: for rule in &rules {
                if positions.contains_key(&rule.field) {
                    continue;
                }
                for range in rule.get_ranges() {
                    if range.includes(num) {
                        continue 'next_rule;
                    }
                }
                let rule_name = rule.field.to_owned();

                confirm_rule(&mut matrix, &mut positions, rule_name, pos);
            }
        }
        if matrix.is_empty() {
            break;
        }
    }
    positions.retain(|k, _| k.starts_with("departure"));

    let mut product = 1;
    for val in positions.values() {
        product *= my_ticket[*val];
    }
    Ok(product)
}
*/
fn question1(data: Vec<String>) -> Result<usize, &'static str> {
    let re = Regex::new(r"\d+-\d+").unwrap();
    let rules = get_rules(data[0].to_owned(), &re);
    let (_invalid_line, invalid) = filter_invalid(data[2].to_owned(), rules);

    Ok(invalid)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file_by_p()?;
    //println!("{:#?}", data);
    match question1(data.to_owned()) {
        Ok(x) => println!("The result for question 1 is {}", x),
        Err(x) => eprintln!("Error processing the input data: {:?}", x),
    };
    /*
    match question2(data) {
        Ok(x) => println!("The sequency from position {}", x),
        Err(x) => eprintln!("Error processing the input data: {:?}", x),
    };
    */
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
        assert_eq!(Ok(71), question1(data));
    }
    /*
    #[test]
    fn test_question2() {
        let data: Vec<String> = TEST_INPUT2
            .split("\n\n")
            .map(|s| s.trim().to_string())
            .collect();

        assert_eq!(Ok(1), question2(data));
    }
    */
}
