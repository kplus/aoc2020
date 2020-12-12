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
    wp_east: i32,
    wp_north: i32,
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
            wp_east: 10,
            wp_north: 1,
            east_position: 0,
            north_position: 0,
        }
    }

    fn action(&mut self, instr: Instructions) {
        match instr {
            Instructions::East(x) => self.wp_east += x,
            Instructions::North(x) => self.wp_north += x,
            Instructions::Turn(x) => match x / 90 {
                0 | 4 => {}
                1 => {
                    let tmp = self.wp_north;
                    self.wp_north = self.wp_east;
                    self.wp_east = -tmp;
                }
                2 => {
                    self.wp_east = -self.wp_east;
                    self.wp_north = -self.wp_north;
                }
                3 => {
                    let tmp = self.wp_north;
                    self.wp_north = -self.wp_east;
                    self.wp_east = tmp;
                }
                _ => eprintln!("Invalid angle found."),
            },
            Instructions::Forward(x) => {
                self.east_position += x * self.wp_east;
                self.north_position += x * self.wp_north;
            }
        }
    }
}

fn question(v: Vec<String>) -> Result<i32, &'static str> {
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
    match question(data) {
        Ok(x) => {
            println!("The result for question 2 is {}", x);
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
    fn test_question() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Ok(286), question(data));
    }
}
