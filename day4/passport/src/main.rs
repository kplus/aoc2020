use std::error::Error;
use std::fs;
use std::path::Path;

struct ID {
    byr: String, //Birth Year
    iyr: String, //Issue Year
    eyr: String, //Expiration Year
    hgt: String, //Height
    hcl: String, //Hair Color
    ecl: String, //Eye Color
    pid: String, //Passport ID
    cid: String, //Country ID
}

fn load_file<P: AsRef<Path>>(path: P) -> Result<Vec<ID>, Box<dyn Error>> {
    let input = fs::read_to_string(path)?;
    //println!("read in content:\n{}", input);

    for person in input.split("\n\n") {
        println!("read in peron details:\n{}", person);
    }
    let id = ID {
        byr: String::from("0"),
        iyr: String::from("0"),
        eyr: String::from("0"),
        hgt: String::from("0cm"),
        hcl: String::from("#fffffd"),
        ecl: String::from("gry"),
        pid: String::from("0"),
        cid: String::from("0"),
    };
    let mut out = Vec::new();
    out.push(id);
    Ok(out)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file("../input.txt")?;

    Ok(())
}
