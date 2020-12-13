use std::error::Error;

use aoc2020::*;

// find the least common multiple of 2 intergers
// use Euclid's algorithm to find gcd, and got lcm = m*n/gcd
fn get_lcm(m: u64, n: u64) -> u64 {
    let mut big = if m >= n { m } else { n };
    let mut small = if m < n { m } else { n };
    let mut r = big % small;
    while r != 0 {
        big = small;
        small = r;
        r = big % small;
    }
    m * n / small
}

// get the next timestamp with new bus added into consideration
fn add_bus(lcm: &u64, last_match: &u64, bus: u64, timestamp: u64) -> u64 {
    let mut count = 0;
    while (lcm * count + last_match + timestamp) % bus != 0 {
        count += 1;
    }
    lcm * count + last_match
}

fn question2(data: Vec<String>) -> Result<u64, &'static str> {
    let mut lcm = 1;
    let mut first_match = 0;
    for (timestamp, bus) in data[1].split(',').enumerate() {
        if bus != "x" {
            let bus = bus.parse::<u64>().unwrap();
            first_match = add_bus(&lcm, &first_match, bus, timestamp as u64);
            lcm = get_lcm(lcm, bus);
        }
    }

    Ok(first_match)
}

fn question1(data: Vec<String>) -> Result<u64, &'static str> {
    let time: u64 = data[0].parse().unwrap();
    let buses: Vec<u64> = data[1]
        .split(|c| c == 'x' || c == ',')
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut wait = u64::MAX;
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
    //println!("{:#?}", data);
    match question1(data.to_owned()) {
        Ok(x) => println!("The result for question 1 is {}", x),
        Err(x) => eprintln!("Error processing the input data: {:?}", x),
    };
    match question2(data) {
        Ok(x) => println!("The result for question 2 is {}", x),
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
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Ok(1068781), question2(data));
    }

    #[test]
    fn test_get_lcm() {
        assert_eq!(1, get_lcm(1, 1));
        assert_eq!(100, get_lcm(1, 100));
        assert_eq!(142, get_lcm(71, 142));
        assert_eq!(360, get_lcm(72, 30));
        assert_eq!(2201, get_lcm(71, 31));
    }
}
