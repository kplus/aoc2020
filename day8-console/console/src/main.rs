use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
enum Instr {
    nop, //No OPeration - it does nothing
    acc, //increases or decreases accumulator
    jmp, //jumps to a new instruction relative to itself
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
            "nop" => Instr::nop,
            "acc" => Instr::acc,
            "jmp" => Instr::jmp,
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
            Instr::acc => {
                prom.accum += self.parameter;
                prom.pos += 1;
            }
            Instr::nop => {
                prom.pos += 1;
            }
            Instr::jmp => {
                let mut pos = prom.pos as i16;
                pos += self.parameter;
                if pos > 0 {
                    prom.pos = pos as usize;
                } else {
                    eprintln!("Error: pos changed to {}", pos);
                }
            }
        }
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

    //TODO: Run the program
    fn run(&mut self) {
        let pos = self.pos;
        let command = self.program[pos].to_owned();
        command.execute(self)
    }

    // Get the current accumlator
    fn get_accumlator(&self) -> i16 {
        self.accum
    }

    // Set current command to be executed
    fn set_flag(&mut self) {
        self.program[self.pos].executed = true;
    }
}

// Find the first time program get into infinite loop
// [in]     ConsoleProgram to analayse
// [out]    Value of accumlator
fn find_loop(mut prog: ConsoleProm) -> i16 {
    while !prog.meet_twice() {
        //        println!("haven't meet twice, carry on running");
        prog.run();
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

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file("../input.txt")?;
    //println!("{:#?}", data);
    let console_program = ConsoleProm::new(data);
    println!("CP is {:#?}", console_program);
    let out = find_loop(console_program);
    println!(
        "The accuulator is {} when entering loop for first time.",
        out
    );
    Ok(())
}
