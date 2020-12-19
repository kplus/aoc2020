use std::collections::HashMap;
use std::error::Error;
use std::iter::FromIterator;

use aoc2020::*;

#[derive(Clone)]
struct RULE {
    count: usize,
    pattern: Vec<String>,
}

impl RULE {
    fn is_empty(&self) -> bool {
        self.count == 0
    }
    fn new() -> Self {
        RULE {
            count: 0,
            pattern: Vec::new(),
        }
    }
    fn obey(&self, s: &str) -> bool {
        self.pattern.contains(&s.to_string())
    }

    fn update_end(&mut self, s: String) {
        self.count = 1;
        self.pattern.push(s);
    }
}
fn question2(data: Vec<String>) -> Result<usize, &'static str> {
    Err("Cannot find second number.")
}

//doing: Get rules from input string of rules section
// [in]     Paragraph of input strings contain all rules
// [in]     Mutable rules table
// [in]     Index of rule to get
// [out]    Rule indicated by index
fn get_rule(map: HashMap<usize, String>, rules: &mut [RULE], n: usize) -> &RULE {
    //println!("the paragraph is {:#?}", map);
    let mut rule = &mut rules[n];

    if rule.is_empty() {
        let rule_string = map.get(&n).unwrap();
        if rule_string.contains('"') {
            let c = rule_string.split('"').nth(1).unwrap().to_string();
            rule.update_end(c);
        } else {
            //todo: update rule from sub rules
        }
    }

    rule
}

fn question1(data: Vec<String>) -> Result<usize, &'static str> {
    let rule_map: HashMap<usize, String> = HashMap::from_iter(data[0].lines().map(|s| {
        let tmp: Vec<&str> = s.trim().split(':').collect();
        (tmp[0].parse().unwrap(), tmp[1].to_owned())
    }));
    //println!("Generated rule map is {:#?}", rule_map);

    let mut rules: Vec<RULE> = vec![RULE::new(); rule_map.len()];
    let rule0 = get_rule(rule_map, &mut rules, 0);

    let mut count = 0;
    for msg in data[1].lines() {
        if rule0.obey(msg) {
            count += 1;
        }
    }
    Ok(count)
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

    static TEST_INPUT: &str = r#"0: 4 1 5
    1: 2 3 | 3 2
    2: 4 4 | 5 5
    3: 4 5 | 5 4
    4: "a"
    5: "b"
    
    ababbb
    bababa
    abbbab
    aaabbb
    aaaabbb"#;

    #[test]
    fn test_question1() {
        let data: Vec<String> = TEST_INPUT
            .split("\n    \n")
            .map(|s| s.trim().to_string())
            .collect();
        //println!("input data is {:#?}", data);

        assert_eq!(Ok(2), question1(data));
    }
    #[test]
    fn test_question2() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Err("Cannot find second number."), question2(data));
    }
}
