use std::error::Error;

use aoc2020::*;

#[derive(Debug)]
enum SIGN {
    Plus,
    Multip,
}
fn question2(data: Vec<String>) -> Result<usize, &'static str> {
    Err("Cannot find second number.")
}

fn question1(data: &Vec<char>, pos: &mut usize) -> Result<usize, &'static str> {
    let mut result = 0;
    let mut sign = SIGN::Plus;
    while *pos < data.len() {
        let c = data[*pos];
        *pos += 1;
        println!(
            "Current char is {}, at position {}, sign is {:?}, result is {}",
            c, pos, sign, result
        );
        match c {
            '+' => sign = SIGN::Plus,
            '*' => sign = SIGN::Multip,
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => match sign {
                SIGN::Multip => result *= c.to_digit(10).unwrap() as usize,
                SIGN::Plus => result += c.to_digit(10).unwrap() as usize,
            },
            '(' => match sign {
                SIGN::Multip => result *= question1(&data, pos).unwrap(),
                SIGN::Plus => result += question1(&data, pos).unwrap(),
            },
            ')' => {
                return Ok(result);
            }
            ' ' => {}
            _ => {
                return Err("Unexpected character in expression.");
            }
        }
    }
    println!("Got a resutl of {}", result);
    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file()?;
    //println!("{:#?}", data);

    let mut sum = 0;
    for exp in data {
        let mut pos = 0;
        let exp: Vec<char> = exp.chars().collect();
        match question1(&exp, &mut pos) {
            Ok(x) => sum += x,
            Err(x) => eprintln!("Error processing the input data: {:?}", x),
        };
    }

    println!("The sequency from position {}", sum);
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

    static TEST_INPUT: &str = r"1 + 2 * 3 + 4 * 5 + 6
    2 * 3 + (4 * 5)
    5 + (8 * 3 + 9 + 3 * 4 * 3)
    5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) 
    ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    #[test]
    fn test_question1() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Ok(71), question1(&data[0].chars().collect(), &mut 0));
        assert_eq!(Ok(26), question1(&data[1].chars().collect(), &mut 0));
        assert_eq!(Ok(437), question1(&data[2].chars().collect(), &mut 0));
        assert_eq!(Ok(12240), question1(&data[3].chars().collect(), &mut 0));
        assert_eq!(Ok(13632), question1(&data[4].chars().collect(), &mut 0));
    }
    #[test]
    fn test_question2() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Err("Cannot find second number."), question2(data));
    }
}
