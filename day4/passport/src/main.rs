use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

// [in]     A single person's full details
// [out]    A Hashmap stored all the information for a single person
fn fill_hash(person: &str) -> HashMap<String, String> {
    let mut details = HashMap::new();
    for items in person.split_whitespace() {
        let value: Vec<&str> = items.split(':').collect();
        details.insert(value[0].to_string(), value[1].to_string());
    }
    details
}

// [in]     Path of file to read personal details from
// [out     Arrary of Hashmaps, each Hashmap stores full details of a single person
fn load_file<P: AsRef<Path>>(path: P) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    let input = fs::read_to_string(path)?;
    //println!("read in content:\n{}", input);

    let mut out = Vec::new();
    for person in input.split("\n\n") {
        //println!("read in peron details:\n{}", person);
        out.push(fill_hash(person));
    }
    Ok(out)
}

// validation functions for each items
fn val_yr(val: &str, min: i16, max: i16) -> bool {
    let val: i16 = val.parse().unwrap();
    val >= min && val <= max
}

fn val_hgt(h: &str) -> bool {
    const MIN_HGT_CM: i16 = 150;
    const MAX_HGT_CM: i16 = 193;
    const MIN_HGT_IN: i16 = 59;
    const MAX_HGT_IN: i16 = 76;

    let min;
    let max;
    let val: i16;
    if h.ends_with("cm") && h.len() == 5 {
        //println!("Using cm");
        min = MIN_HGT_CM;
        max = MAX_HGT_CM;
        val = h.get(0..3).unwrap().parse().unwrap();
    } else if h.ends_with("in") && h.len() == 4 {
        println!("Using inch");
        min = MIN_HGT_IN;
        max = MAX_HGT_IN;
        val = h.get(0..2).unwrap().parse().unwrap();
    } else {
        eprintln!("hgt doesn't have valid unit");
        return false;
    }
    //println!("hight is {}", val);
    val >= min && val <= max
}

fn val_hcl(val: &str) -> bool {
    if !val.starts_with('#') {
        eprintln!("hcl doesn't start with #");
        return false;
    }
    if val.len() != 7 {
        eprintln!("hcl is not 7 digits long");
        return false;
    }
    for c in val[1..].chars() {
        if !c.is_digit(16) {
            eprintln!("hcl contains no-hex char");
            return false;
        }
    }
    true
}

fn val_ecl(val: &str) -> bool {
    const EYE_COLER: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    for c in EYE_COLER.iter() {
        if val.contains(c) {
            return true;
        }
    }
    eprintln!("ecl has invalid value");
    false
}
fn val_pid(val: &str) -> bool {
    if val.len() != 9 {
        return false;
    }

    for c in val.chars() {
        if !c.is_digit(10) {
            eprintln!("pid has non-decimal number");
            return false;
        }
    }
    true
}

fn check(id: HashMap<String, String>) -> bool {
    const KEYS_TO_CHECK: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    const MIN_BYR: i16 = 1920;
    const MAX_BYR: i16 = 2002;
    const MIN_IYR: i16 = 2010;
    const MAX_IYR: i16 = 2020;
    const MIN_EYR: i16 = 2020;
    const MAX_EYR: i16 = 2030;
    //println!("the id to check is {:#?}", id);
    for f in KEYS_TO_CHECK.iter() {
        //println!("the key to check is {}", f);

        if !id.contains_key(&f.to_string()) {
            //println!("The key is not included");
            return false;
        }
        match *f {
            "byr" => {
                if !val_yr(id.get(&f.to_string()).unwrap(), MIN_BYR, MAX_BYR) {
                    return false;
                }
            }
            "iyr" => {
                if !val_yr(id.get(&f.to_string()).unwrap(), MIN_IYR, MAX_IYR) {
                    return false;
                }
            }
            "eyr" => {
                if !val_yr(id.get(&f.to_string()).unwrap(), MIN_EYR, MAX_EYR) {
                    return false;
                }
            }
            "hgt" => {
                if !val_hgt(id.get(&f.to_string()).unwrap()) {
                    return false;
                }
            }
            "hcl" => {
                if !val_hcl(id.get(&f.to_string()).unwrap()) {
                    return false;
                }
            }
            "ecl" => {
                if !val_ecl(id.get(&f.to_string()).unwrap()) {
                    return false;
                }
            }
            "pid" => {
                if !val_pid(id.get(&f.to_string()).unwrap()) {
                    return false;
                }
            }
            _ => panic!(),
        }
    }
    //println!("it seems all esstential key are here");
    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file("../input.txt")?;

    //println!("{:#?}", data);

    let mut count = 0;
    for id in data {
        if check(id) {
            //println!("got a valid ID");
            count += 1;
        }
    }

    println!("There are {} valid IDs", count);
    Ok(())
}
