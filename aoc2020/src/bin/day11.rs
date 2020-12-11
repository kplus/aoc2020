use std::error::Error;

use aoc2020::*;

#[derive(Clone, Copy, PartialEq, Debug)]
enum STATE {
    Occupied,
    Empty,
    Floor,
}

#[derive(Clone, Copy, Debug)]
struct SEAT {
    state: STATE,
    old_state: STATE,
    changed: bool,
}

impl SEAT {
    fn from_char(c: char) -> Self {
        let state = {
            if c == 'L' {
                STATE::Empty
            } else {
                STATE::Floor
            }
        };
        SEAT {
            state,
            old_state: state,
            changed: false,
        }
    }
    fn _get_state(&self) -> STATE {
        self.state
    }
    fn _get_old_state(&self) -> STATE {
        self.old_state
    }
    fn _check_changed(&self) -> bool {
        self.changed
    }

    fn _get_state_count(&self) -> u8 {
        (self.state == STATE::Occupied) as u8
    }
    fn _get_old_state_count(&self) -> u8 {
        (self.old_state == STATE::Occupied) as u8
    }

    fn pre_update(&mut self) -> u8 {
        //todo: read adj array to get sum
        let adjacent = 0;
        self.changed = self.state != self.old_state;
        self.old_state = self.state;
        adjacent
    }

    fn _update(&mut self) {
        match self.state {
            STATE::Floor => {}
            STATE::Empty => {
                if self.pre_update() == 0 {
                    self.state = STATE::Occupied;
                }
            }
            STATE::Occupied => {
                if self.pre_update() >= 4 {
                    self.state = STATE::Empty;
                }
            }
        }
    }
}
fn question2() -> Result<usize, &'static str> {
    Err("Cannot find second number.")
}

fn question1(v: Vec<String>) -> Result<usize, &'static str> {
    for line in v.iter() {
        let row: Vec<SEAT> = line.to_owned().chars().map(SEAT::from_char).collect();
        println!("row is {:#?}", row);
    }
    Err("Cannot find first number.")
}
fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file()?;
    //println!("{:#?}", data);
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

    static TEST_INPUT: &str = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_question1() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Err("Cannot find first number."), question1(data));
    }
    #[test]
    fn test_question2() {
        let _data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Err("Cannot find second number."), question2());
    }
}
