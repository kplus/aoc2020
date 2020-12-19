use std::error::Error;

use aoc2020::*;

fn question2(data: Vec<String>) -> Result<usize, &'static str> {
    Err("Cannot find second number.")
}

//doing: Build rule 0 from input string of rules section
// [in]     Paragraph of input strings contain all rules
// [in]     Mutable rules table
// [in]     Index of rule to build
// [out]    Rule 0 as vector of all patterns
fn build_rules(p: Vec<(usize, String)>, rules: &mut [Vec<String>]) -> Vec<String> {
    println!("the paragraph is {:#?}", p);

    let pattern = String::new();
    let mut rule = Vec::new();
    rule.push(pattern);
    rule
}

fn question1(data: Vec<String>) -> Result<usize, &'static str> {
    println!("Input data are: {:#?}", data);
    let rule_string: Vec<(usize, String)> = data[0]
        .lines()
        .map(|s| {
            let tmp: Vec<&str> = s.trim().split(':').collect();
            let index: usize = tmp[0].parse().unwrap();
            let s_out = tmp[1].to_owned();
            (index, s_out)
        })
        .collect();
    println!("Generated rule String is {:#?}", rule_string);
    /*
    let rule_string: Vec<(usize, String)> = data[0]
        .lines()
        .map(|s| {
            let tmp: Vec<&str> = s.split(':').collect();
            (tmp[0].parse().unwrap(), tmp[1].to_owned())
        })
        .collect();

    println!("Generated rule String is {:#?}", rule_string);
    let mut rules: Vec<Vec<String>> = vec![Vec::new(); rule_string.len()];
    let rule0 = build_rules(rule_string, &mut rules);

    let mut count = 0;
    for msg in data[1].lines() {
        if rule0.contains(&msg.to_string()) {
            count += 1;
        }
    }
    */
    let mut count = 0;
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
