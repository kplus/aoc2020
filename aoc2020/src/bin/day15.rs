use std::collections::HashMap;
use std::error::Error;

fn question(data: Vec<usize>, end: usize) -> Result<usize, &'static str> {
    // the weight of numbers are congest at lower numbers
    // and there are roughly 1/8 duty cycle
    // store lowest 1/10 numbers in vector enjoys most of the benifit
    // of O(1) lookup without using more space
    let split_try = end / 10;
    let start = data.len() - 1;
    let split = if split_try > start { split_try } else { start };

    let mut game_vec = vec![usize::MAX; split];
    let mut game_hash: HashMap<usize, usize> = HashMap::new();
    for i in 0..start {
        game_vec[data[i]] = i;
    }
    //println!("initial map is {:#?}", game);
    let mut next = data[start];
    for count in start..end - 1 {
        if next < split {
            match game_vec[next] {
                usize::MAX => {
                    game_vec[next] = count;
                    next = 0;
                }
                x => {
                    game_vec[next] = count;
                    next = count - x;
                }
            }
        } else {
            match game_hash.get_mut(&next) {
                Some(x) => {
                    next = count - *x;
                    *x = count;
                }
                None => {
                    game_hash.insert(next, count);
                    next = 0;
                }
            }
        }
    }
    Ok(next)
}

fn main() -> Result<(), Box<dyn Error>> {
    const DATA: &str = r"6,13,1,15,2,0";
    const END1: usize = 2020;
    const END2: usize = 3000_0000;
    //println!("{:#?}", DATA);

    let start_numbers: Vec<usize> = DATA.split(',').map(|s| s.parse().unwrap()).collect();
    match question(start_numbers.to_owned(), END1) {
        Ok(x) => println!("The result for question 1 is {}", x),
        Err(x) => eprintln!("Error processing the input data: {:?}", x),
    };
    match question(start_numbers, END2) {
        Ok(x) => println!("The sequency from position {}", x),
        Err(x) => eprintln!("Error processing the input data: {:?}", x),
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question1() {
        let input = vec![
            (vec![0, 3, 6], 436),
            (vec![1, 3, 2], 1),
            (vec![2, 1, 3], 10),
            (vec![1, 2, 3], 27),
            (vec![2, 3, 1], 78),
            (vec![3, 2, 1], 438),
            (vec![3, 1, 2], 1836),
        ];
        for (start, target) in input {
            assert_eq!(Ok(target), question(start, 2020));
        }
    }
    #[test]
    fn test_question2() {
        let input = vec![
            (vec![0, 3, 6], 175594),
            (vec![1, 3, 2], 2578),
            (vec![2, 1, 3], 3544142),
            (vec![1, 2, 3], 261214),
            (vec![2, 3, 1], 6895259),
            (vec![3, 2, 1], 18),
            (vec![3, 1, 2], 362),
        ];
        for (start, target) in input {
            assert_eq!(1, 1);
            assert_eq!(Ok(target), question(start, 3000_0000));
        }
    }
}
