use std::error::Error;

use aoc2020::*;

#[derive(Debug, PartialEq)]
enum SIGN {
    Plus,
    Multip,
}

fn question(
    data: &[char],
    pos: &mut usize,
    from_multip: bool,
    q: usize,
) -> Result<usize, &'static str> {
    let mut result = 0;
    let mut sign = SIGN::Plus;
    while *pos < data.len() {
        let c = data[*pos];
        *pos += 1;
        match c {
            '+' => sign = SIGN::Plus,
            '*' => {
                sign = SIGN::Multip;
                if q == 2 {
                    // multiplication has lower precedence in q2, so we call recursivly to proceed
                    result *= question(&data, pos, true, q).unwrap();
                }
            }
            x if x.is_digit(10) => {
                // as there is only single digits input, we can do this
                let d = c.to_digit(10).unwrap() as usize;
                match sign {
                    SIGN::Multip => result *= d,
                    SIGN::Plus => result += d,
                };
            }
            '(' => match sign {
                SIGN::Multip => result *= question(&data, pos, false, q).unwrap(),
                SIGN::Plus => result += question(&data, pos, false, q).unwrap(),
            },
            ')' => {
                if from_multip && q == 2 {
                    // multiplication will only return either hits close bracket or end
                    // if it hits close bracket, it has to return twice
                    *pos -= 1;
                }
                return Ok(result);
            }
            ' ' => {}
            _ => {
                return Err("Unexpected character in expression.");
            }
        }
    }
    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file()?;
    //println!("{:#?}", data);

    let mut sum = [0, 0];
    for exp in data {
        let mut pos = [0, 0];
        let exp: Vec<char> = exp.chars().collect();
        for i in 0..2 {
            match question(&exp, &mut pos[i], false, i + 1) {
                Ok(x) => sum[i] += x,
                Err(x) => eprintln!("Error processing the input data: {:?}", x),
            };
        }
    }
    println!("The sum for question 1 {}", sum[0]);
    println!("The sum for question 2 {}", sum[1]);
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
        let data_exps: Vec<Vec<char>> = data.iter().map(|line| line.chars().collect()).collect();

        assert_eq!(Ok(71), question(&data_exps[0], &mut 0, false, 1));
        assert_eq!(Ok(26), question(&data_exps[1], &mut 0, false, 1));
        assert_eq!(Ok(437), question(&data_exps[2], &mut 0, false, 1));
        assert_eq!(Ok(12240), question(&data_exps[3], &mut 0, false, 1));
        assert_eq!(Ok(13632), question(&data_exps[4], &mut 0, false, 1));
    }
    #[test]
    fn test_question2() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();
        let data_exps: Vec<Vec<char>> = data.iter().map(|line| line.chars().collect()).collect();

        assert_eq!(Ok(231), question(&data_exps[0], &mut 0, false, 2));
        assert_eq!(Ok(46), question(&data_exps[1], &mut 0, false, 2));
        assert_eq!(Ok(1445), question(&data_exps[2], &mut 0, false, 2));
        assert_eq!(Ok(669060), question(&data_exps[3], &mut 0, false, 2));
        assert_eq!(Ok(23340), question(&data_exps[4], &mut 0, false, 2));
    }
}
