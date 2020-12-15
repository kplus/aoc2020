use std::error::Error;

const END1: usize = 30000000;
//const END1: usize = 2020;
fn question(data: Vec<usize>) -> Result<usize, &'static str> {
    let start = data.len() - 1;
    let mut game = vec![usize::MAX; END1];

    for i in 0..start {
        game[data[i]] = i;
    }
    //for i in 0..20 {
    //    println!("position {} is {}", i, game[i]);
    //}
    let mut next = data[start];
    for count in start..END1 - 1 {
        //println!(
        //    "checking number {}, it's last position is {} ,count is {}, ",
        //    next, game[next], count
        //);
        match game[next] {
            usize::MAX => {
                game[next] = count;
                next = 0;
            }
            x => {
                game[next] = count;
                next = count - x;
            }
        }
    }
    Ok(next)
}

fn main() -> Result<(), Box<dyn Error>> {
    const DATA: &str = r"6,13,1,15,2,0";
    //println!("{:#?}", DATA);

    let start_numbers: Vec<usize> = DATA.split(',').map(|s| s.parse().unwrap()).collect();
    match question(start_numbers.to_owned()) {
        Ok(x) => println!("The result for question 1 is {}", x),
        Err(x) => eprintln!("Error processing the input data: {:?}", x),
    };
    /*
    match question(start_numbers, END2) {
        Ok(x) => println!("The sequency from position {}", x),
        Err(x) => eprintln!("Error processing the input data: {:?}", x),
    };
    */
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
            assert_eq!(Ok(target), question(start));
        }
    }
    /*
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
            assert_eq!(Ok(target), question(start));
        }
    }
    */
}
