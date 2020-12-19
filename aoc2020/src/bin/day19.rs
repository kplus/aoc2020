use std::collections::HashMap;
use std::error::Error;
use std::iter::FromIterator;

use aoc2020::*;

#[derive(Clone, Debug)]
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

    fn set(&mut self, other: RULE) {
        self.count = other.count;
        self.pattern = other.pattern;
    }
    fn obey(&self, s: &str) -> bool {
        self.pattern.contains(&s.to_string())
    }

    fn update_end(&mut self, s: String) {
        self.count = 1;
        self.pattern.push(s);
    }

    //Upate current rule with 2 subrules
    fn update_subrule(&mut self, rule_to_add: Vec<RULE>) {
        let len = rule_to_add.len();
        match len {
            1 => {
                for first_pattern in &rule_to_add[0].pattern {
                    let pattern = first_pattern.to_owned();
                    self.pattern.push(pattern);
                }
            }
            2 => {
                for first_pattern in &rule_to_add[0].pattern {
                    for second_pattern in &rule_to_add[1].pattern {
                        let pattern =
                            [first_pattern.to_owned(), second_pattern.to_owned()].concat();
                        self.pattern.push(pattern);
                    }
                }
            }
            3 => {
                for first_pattern in &rule_to_add[0].pattern {
                    for second_pattern in &rule_to_add[1].pattern {
                        for third_pattern in &rule_to_add[2].pattern {
                            let pattern = [
                                first_pattern.to_owned(),
                                second_pattern.to_owned(),
                                third_pattern.to_owned(),
                            ]
                            .concat();
                            self.pattern.push(pattern);
                        }
                    }
                }
            }
            _ => unimplemented!(),
        }
        self.count = self.pattern.len();
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
fn get_rule(map: &HashMap<usize, String>, rules: &mut [RULE], n: usize) -> RULE {
    //println!("the paragraph is {:#?}", map);
    let mut rule = rules[n].to_owned();

    if rule.is_empty() {
        let rule_string = map.get(&n).unwrap();
        if rule_string.contains('"') {
            let c = rule_string.split('"').nth(1).unwrap().to_string();
            rule.update_end(c);
        } else {
            let sub_rules: Vec<&str> = rule_string.split('|').map(|sr| sr.trim()).collect();
            for sub_rule in sub_rules {
                let seq: Vec<usize> = sub_rule
                    .split(' ')
                    .map(|i| i.parse::<usize>().unwrap())
                    .collect();
                let rule_to_add: Vec<RULE> =
                    seq.iter().map(|n| get_rule(&map, rules, *n)).collect();
                rule.update_subrule(rule_to_add);
            }
        }
    }
    //println!("rule {} is to add {:#?}", n, rule);
    rules[n].set(rule);
    rules[n].to_owned()
}

const MAX_RULE: usize = 200;
fn question1(data: Vec<String>) -> Result<usize, &'static str> {
    let rule_map: HashMap<usize, String> = HashMap::from_iter(data[0].lines().map(|s| {
        let tmp: Vec<&str> = s.trim().split(':').collect();
        (tmp[0].parse().unwrap(), tmp[1].to_owned())
    }));
    println!("Generated rule map is {:#?}", rule_map);

    let mut rules: Vec<RULE> = vec![RULE::new(); MAX_RULE]; // quick fix to use fixed max rules count, as there are skipping numbers in example
                                                            //let mut rules: Vec<RULE> = vec![RULE::new(); rule_map.len()];
    let rule0 = get_rule(&rule_map, &mut rules, 0);

    //println!("the rule 0 is {:#?}", rule0);

    let mut count = 0;
    for msg in data[1].lines().map(|s| s.trim()) {
        //     println!("message {}: ", msg);
        if rule0.obey(msg) {
            //       println!("matches.");
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

    static TEST_INPUT2: &str = r#"42: 9 14 | 10 1
    9: 14 27 | 1 26
    10: 23 14 | 28 1
    1: "a"
    11: 42 31
    5: 1 14 | 15 1
    19: 14 1 | 14 14
    12: 24 14 | 19 1
    16: 15 1 | 14 14
    31: 14 17 | 1 13
    6: 14 14 | 1 14
    2: 1 24 | 14 4
    0: 8 11
    13: 14 3 | 1 12
    15: 1 | 14
    17: 14 2 | 1 7
    23: 25 1 | 22 14
    28: 16 1
    4: 1 1
    20: 14 14 | 1 15
    3: 5 14 | 16 1
    27: 1 6 | 14 18
    14: "b"
    21: 14 1 | 1 14
    25: 1 1 | 1 14
    22: 14 14
    8: 42
    26: 14 22 | 1 20
    18: 15 15
    7: 14 5 | 1 21
    24: 14 1
    
    abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
    bbabbbbaabaabba
    babbbbaabbbbbabbbbbbaabaaabaaa
    aaabbbbbbaaaabaababaabababbabaaabbababababaaa
    bbbbbbbaaaabbbbaaabbabaaa
    bbbababbbbaaaaaaaabbababaaababaabab
    ababaaaaaabaaab
    ababaaaaabbbaba
    baabbaaaabbaaaababbaababb
    abbbbabbbbaaaababbbbbbaaaababb
    aaaaabbaabaaaaababaa
    aaaabbaaaabbaaa
    aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
    babaaabbbaaabaababbaabababaaab
    aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

    #[test]
    fn test_question1() {
        let data: Vec<String> = TEST_INPUT
            .split("\n    \n")
            .map(|s| s.trim().to_string())
            .collect();
        //println!("input data is {:#?}", data);

        assert_eq!(Ok(2), question1(data));
        let data: Vec<String> = TEST_INPUT2
            .split("\n    \n")
            .map(|s| s.trim().to_string())
            .collect();

        assert_eq!(Ok(3), question1(data));
    }
    #[test]
    fn test_question2() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Err("Cannot find second number."), question2(data));
    }
}
