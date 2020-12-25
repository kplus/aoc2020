use std::error::Error;

use aoc2020::*;

const DIV: usize = 20201227;

fn question2(data: Vec<String>) -> Result<usize, &'static str> {
    Err("Cannot find second number.")
}

fn get_loop_size(p_key: usize) -> usize {
    const SUB: usize = 7;
    let mut key = 1;
    let mut loop_size = 0;
    loop {
        if key == p_key {
            break;
        }
        key = (key * SUB) % DIV;
        loop_size += 1;
    }
    loop_size
}

fn do_loop(d_pub: usize, loop_size: usize) -> usize {
    let mut key = 1;
    for _i in 0..loop_size {
        key = (key * d_pub) % DIV;
    }
    key
}
fn question1(data: Vec<String>) -> Result<usize, &'static str> {
    let card_pub: usize = data[0].parse().unwrap();
    let door_pub: usize = data[1].parse().unwrap();
    //println!("card pub is {}, door pub is {}", card_pub, door_pub);

    let loop_size = get_loop_size(card_pub);
    //println!("loop size for card is {}", loop_size);

    Ok(do_loop(door_pub, loop_size))
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

    static TEST_INPUT: &str = r"5764801
    17807724";

    #[test]
    fn test_question1() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Ok(14897079), question1(data));
    }
    #[test]
    fn test_question2() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Err("Cannot find second number."), question2(data));
    }
}
