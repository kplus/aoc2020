use std::error::Error;

use aoc2020::*;

fn question2(data: Vec<String>) -> Result<usize, &'static str> {
    Err("Cannot find second number.")
}

//todo: Run a round to compare and move cards
fn run(p1: &mut Vec<String>, p2: &mut Vec<String>) {}

fn question1(mut player1: Vec<String>) -> Result<usize, &'static str> {
    let len = player1.len();
    let mut player2 = player1.split_off(len / 2 + 1);

    player1.remove(0);
    player1.pop();
    player2.remove(0);

    println!("player 1 is {:#?}", player1);
    println!("player 2 is {:#?}", player2);

    while !player1.is_empty() && !player2.is_empty() {
        run(&mut player1, &mut player2);
    }

    let mut winner = player1;
    if player2.is_empty() {
        println!("Winner is player 1");
    } else {
        println!("Winner is player 2");
        winner = player2;
    }
    let len = winner.len();
    Ok(winner
        .iter()
        .enumerate()
        .map(|(i, v)| v.parse::<usize>().unwrap() * (len - i))
        .sum())
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

    static TEST_INPUT: &str = r"Player 1:
    9
    2
    6
    3
    1
    
    Player 2:
    5
    8
    4
    7
    10";

    #[test]
    fn test_question1() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Ok(306), question1(data));
    }
    #[test]
    fn test_question2() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Err("Cannot find second number."), question2(data));
    }
}
