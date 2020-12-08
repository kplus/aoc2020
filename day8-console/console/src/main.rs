use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug)]
enum Instr {
    nop, //No OPeration - it does nothing
    acc, //increases or decreases accumulator
    jmp, //jumps to a new instruction relative to itself
}
#[derive(Debug)]
struct Command {
    executed: bool,
    ins: Instr,
    parameter: i16,
}
#[derive(Debug)]
struct ConsoleProm {
    accum: usize,
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
}

//TODO: Find the first time program get into infinite loop
// [in]     ConsoleProgram to analayse
// [out]    Value of accumlator
fn find_loop(prog: ConsoleProm) -> usize {
    7
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
    println!("{:#?}", data);
    let console_program = ConsoleProm::new(data);
    //println!("CP is {:#?}", console_program);
    let out = find_loop(console_program);
    println!(
        "The accuulator is {} when entering loop for first time.",
        out
    );
    Ok(())
}
