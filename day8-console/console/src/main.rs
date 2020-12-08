use std::error::Error;
use std::fs;
use std::path::Path;

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

#[derive(Debug)]
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

// Find the first time program get into infinite loop
// [in]     ConsoleProgram to analayse
// [out]    Value of accumlator
fn find_end(prog: &mut ConsoleProm, lines: usize) -> usize {
    while prog.get_pos() < lines && !prog.meet_twice() {
        println!(
            "haven't meet twice, carry on running, position is {},  acc is {},line is {}",
            prog.get_pos(),
            prog.get_accumlator(),
            lines
        );
        prog.run();
    }
    prog.get_pos()
}

struct Cache {
    pos: usize,
    acc: i16,
}

// Reset pos and accumlator to cached value, also call swap function
// [in]     Cache structure cached the pos and acc to return to
fn reset_pos(cache: &Cache, prog: &mut ConsoleProm) {
    prog.pos = cache.pos;
    prog.accum = cache.acc;
    prog.swap();
}
// Find the position where jmp or nop should be swapped, so that
// the program can run to the end
// [in]     ConsoleProgram to analayse
// [out]    Value of accumlator
fn find_bug(mut prog: ConsoleProm, lines: usize) -> i16 {
    let mut cache = Cache { pos: 0, acc: 0 };
    while prog.get_pos() != lines {
        let mut check = false;
        println!(
            "The pos is {}, acc is {}, line is {}",
            cache.pos, cache.acc, lines
        );
        if prog.get_instr(prog.get_pos()) != Instr::Acc {
            check = true;
        }
        prog.run();
        if check {
            let tmp_pos = prog.get_pos();
            let tmp_acc = prog.get_accumlator();
            println!("after run once, pos is {}, acc is {}", tmp_pos, tmp_acc);
            reset_pos(&cache, &mut prog);
            cache = Cache {
                pos: tmp_pos,
                acc: tmp_acc,
            };
            find_end(&mut prog, lines);
        }
    }
    prog.get_accumlator()
}

// [in]     Path of file to read details from
// [out     Arrary of String for each lines
fn load_file<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Box<dyn Error>> {
    let input = fs::read_to_string(path)?;
    //println!("read in content:\n{}", input);
    let mut out = Vec::new();
    for line in input.lines() {
        //println!("read in peron details:\n{}", person);
        out.push(line.to_string());
    }
    Ok(out)
}

// Question 1 uses find_end, and question 2 uses find_bug
fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file("../input.txt")?;
    let lines = data.len();
    //println!("{:#?}", data);
    let mut console_program = ConsoleProm::new(data);
    //println!("CP is {:#?}", console_program);
    //find_end(&mut console_program, lines);
    //let out = console_program.get_accumlator();
    let out = find_bug(console_program, lines);
    println!("The accuulator is {}", out);
    Ok(())
}
