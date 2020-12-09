use std::error::Error;

use aoc2020::*;
#[derive(Debug, Clone, PartialEq)]
enum Instr {
    Nop, //No OPeration - it does nothing
    Acc, //increases or decreases accumulator
    Jmp, //jumps to a new instruction relative to itself
}

#[derive(Debug, Clone)]
struct Command {
    executed: bool,
    ins: Instr,
    parameter: i16,
}

#[derive(Debug, Clone)]
struct ConsoleProm {
    accum: i16,
    pos: usize,
    program: Vec<Command>,
}
impl Instr {
    fn from_str(st: &str) -> Self {
        match st {
            "nop" => Instr::Nop,
            "acc" => Instr::Acc,
            "jmp" => Instr::Jmp,
            _ => panic!(),
        }
    }
}
impl Command {
    fn new(ins: &str, parameter: i16) -> Self {
        let ins = Instr::from_str(ins);
        Command {
            executed: false,
            ins,
            parameter,
        }
    }

    // Execute the command
    fn execute(&self, prom: &mut ConsoleProm) {
        prom.set_flag();
        match self.ins {
            Instr::Acc => {
                prom.accum += self.parameter;
                prom.pos += 1;
            }
            Instr::Nop => {
                prom.pos += 1;
            }
            Instr::Jmp => {
                let mut pos = prom.pos as i16;
                pos += self.parameter;
                if pos >= 0 {
                    prom.pos = pos as usize;
                } else {
                    eprintln!("Error: pos changed to {}", pos);
                }
            }
        }
    }

    fn swap(&mut self) {
        match self.ins {
            Instr::Nop => self.ins = Instr::Jmp,
            Instr::Jmp => self.ins = Instr::Nop,
            Instr::Acc => println!("acc command doesn't need to swap"),
        }
    }

    fn get_instr(&self) -> Instr {
        self.ins.to_owned()
    }
}

impl ConsoleProm {
    fn new(input: Vec<String>) -> Self {
        let mut prom = ConsoleProm {
            accum: 0,
            pos: 0,
            program: Vec::new(),
        };
        for line in input {
            let i: Vec<&str> = line.split(' ').collect();
            let param = i[1].parse().unwrap();
            let c = Command::new(i[0], param);
            prom.program.push(c);
        }
        prom
    }

    // Check if the command has been reached twice
    fn meet_twice(&self) -> bool {
        self.program[self.pos].executed
    }

    // Run the program
    fn run(&mut self) {
        let pos = self.pos;
        let command = self.program[pos].to_owned();
        command.execute(self);
    }

    // Get the current accumlator
    fn get_accumlator(&self) -> i16 {
        self.accum
    }

    // Get the current accumlator
    fn get_pos(&self) -> usize {
        self.pos
    }

    // Set current command to be executed
    fn set_flag(&mut self) {
        self.program[self.pos].executed = true;
    }

    // Swap instruction
    fn swap(&mut self) {
        self.program[self.pos].swap();
    }

    // get instruction
    fn get_instr(&self, pos: usize) -> Instr {
        self.program[pos].get_instr()
    }
}

// Run program until hit infinite loop or end
// [in]     ConsoleProgram to analayse
fn find_end(mut prog: ConsoleProm, lines: usize) -> (usize, i16) {
    while prog.get_pos() < lines && !prog.meet_twice() {
        //println!(
        //    "haven't meet twice, carry on running, position is {},  acc is {},line is {}",
        //    prog.get_pos(),
        //    prog.get_accumlator(),
        //    lines
        //);
        prog.run();
    }
    (prog.get_pos(), prog.get_accumlator())
}

// Find the position where jmp or nop should be swapped, so that
// the program can run to the end
// [in]     ConsoleProgram to analayse
// [out]    Value of accumlator
fn find_bug(mut prog: ConsoleProm, lines: usize) -> i16 {
    while prog.get_pos() != lines {
        if prog.get_instr(prog.get_pos()) != Instr::Acc {
            let mut test_prom = prog.to_owned();
            test_prom.swap();
            let (pos, acc) = find_end(test_prom, lines);
            //println!("hitted end position is {}", pos);
            if pos == lines {
                return acc;
            }
        }
        prog.run();
    }
    prog.get_accumlator()
}

// Question 1 uses find_end, and question 2 uses find_bug
fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file()?;
    let lines = data.len();
    //println!("{:#?}", data);
    let console_program = ConsoleProm::new(data);
    //println!("CP is {:#?}", console_program);
    //find_end(&mut console_program, lines);
    //let out = console_program.get_accumlator();
    let out = find_bug(console_program, lines);
    println!("The accuulator is {}", out);
    Ok(())
}
