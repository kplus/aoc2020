use std::error::Error;

use aoc2020::*;

fn question2(data: Vec<String>) -> Result<usize, &'static str> {
    Err("Cannot find second number.")
}

// Run a round to compare and move cards
fn run(p1: &mut Vec<usize>, p2: &mut Vec<usize>) {
    let p1_head = p1.remove(0);
    let p2_head = p2.remove(0);
    if p1_head > p2_head {
        p1.push(p1_head);
        p1.push(p2_head);
    } else {
        p2.push(p2_head);
        p2.push(p1_head);
    }
    //println!("stacks are:\n p1: {:?}\n p2: {:?}", p1, p2);
}
fn get_stack(mut player1: Vec<String>) -> (Vec<usize>, Vec<usize>) {
    let len = player1.len();
    let player2 = player1.split_off(len / 2 + 1);

    let player1: Vec<usize> = player1
        .iter()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();
    let player2: Vec<usize> = player2
        .iter()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();
    //println!("player 1 is {:#?}", player1);
    //println!("player 2 is {:#?}", player2);
    (player1, player2)
}
fn output(player1: Vec<usize>, player2: Vec<usize>) -> usize {
    let mut winner = player1;
    if player2.is_empty() {
        println!("Winner is player 1");
    } else {
        println!("Winner is player 2");
        winner = player2;
    }
    let len = winner.len();
    //println!("final stack is {:#?}", winner);
    winner.iter().enumerate().map(|(i, v)| v * (len - i)).sum()
}

fn question1(data: Vec<String>) -> Result<usize, &'static str> {
    let (mut player1, mut player2) = get_stack(data);

    while !player1.is_empty() && !player2.is_empty() {
        run(&mut player1, &mut player2);
    }

    Ok(output(player1, player2))
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

        assert_eq!(Ok(291), question2(data));
    }
}
