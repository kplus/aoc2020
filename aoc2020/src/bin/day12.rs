use std::error::Error;

use aoc2020::*;

#[derive(Debug)]
enum Instructions {
    North(i32),
    East(i32),
    Turn(i32),
    Forward(i32),
}
struct VM {
    angle: i32,
    east_position: i32,
    north_position: i32,
}

impl Instructions {
    fn from_str(s: String) -> Result<Instructions, &'static str> {
        let (instr, argu) = s.split_at(1);
        let argu = argu.parse().unwrap();
        match instr {
            "N" => Ok(Instructions::North(argu)),
            "S" => Ok(Instructions::North(-argu)),
            "E" => Ok(Instructions::East(argu)),
            "W" => Ok(Instructions::East(-argu)),
            "L" => Ok(Instructions::Turn(argu)),
            "R" => Ok(Instructions::Turn(360 - argu)),
            "F" => Ok(Instructions::Forward(argu)),
            _ => Err("Invalid instruction found"),
        }
    }
}

impl VM {
    fn new() -> VM {
        VM {
            angle: 0,
            east_position: 0,
            north_position: 0,
        }
    }

    fn action(&mut self, instr: Instructions) {
        match instr {
            Instructions::East(x) => self.east_position += x,
            Instructions::North(x) => self.north_position += x,
            Instructions::Turn(x) => {
                self.angle += x;
                self.angle %= 360;
            }
            Instructions::Forward(x) => match self.angle / 90 {
                0 => self.east_position += x,
                1 => self.north_position += x,
                2 => self.east_position -= x,
                3 => self.north_position -= x,
                _ => eprintln!("Invalid angle found."),
            },
        }
    }
}
fn question2() -> Result<i32, &'static str> {
    Err("Cannot find second number.")
}

fn question1(v: Vec<String>) -> Result<i32, &'static str> {
    let instr_list: Vec<Instructions> = v
        .iter()
        .map(|s| Instructions::from_str(s.to_string()).unwrap())
        .collect();
    //println!("instr list is {:#?}", instr_list);
    let mut vm = VM::new();
    for instr in instr_list {
        vm.action(instr);
    }
    println!(
        "The end position is at east {}, north {}",
        vm.east_position, vm.north_position
    );
    Ok(vm.east_position.abs() + vm.north_position.abs())
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

    static TEST_INPUT: &str = r"F10
    N3
    F7
    R90
    F11";

    #[test]
    fn test_question1() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Ok(25), question1(data));
    }
    #[test]
    fn test_question2() {
        let _data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Err("Cannot find second number."), question2());
    }
}
