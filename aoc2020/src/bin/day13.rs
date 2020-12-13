use std::error::Error;

use aoc2020::*;

fn question2() -> Result<usize, &'static str> {
    Err("Cannot find second number.")
}

fn question1(data: Vec<String>) -> Result<u32, &'static str> {
    let time: u32 = data[0].parse().unwrap();
    let buses: Vec<u32> = data[1]
        .split(|c| c == 'x' || c == ',')
        .filter_map(|s| s.parse().ok())
        .collect();
    //println!("time is {}, buuses are {:#?}", time, buses);

    let mut wait = u32::MAX;
    let mut bus = 0;
    for b in buses.iter() {
        let gap = b - time % b;
        if gap < wait {
            wait = gap;
            bus = *b;
        };
    }

    println!("The earliest bus is {}, you have to wait for {}", bus, wait);
    Ok(wait * bus)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file()?;
    println!("{:#?}", data);
    match question1(data) {
        Ok(x) => {
            println!("The result for question 1 is {}", x);
        }
        Err(x) => eprintln!("Error processing the input data: {:?}", x),
    };
    match question2() {
        Ok(x) => {
            println!("The sequency from position {}", x);
        }
        Err(x) => eprintln!("Error processing the input data: {:?}", x),
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"939
    7,13,x,x,59,x,31,19";

    #[test]
    fn test_question1() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Ok(295), question1(data));
    }
    #[test]
    fn test_question2() {
        let _data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Err("Cannot find second number."), question2());
    }
}
