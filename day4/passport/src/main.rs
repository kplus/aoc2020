use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

fn fill_hash(person: &str) -> HashMap<String, String> {
    let mut details = HashMap::new();
    for items in person.split_whitespace() {
        let value: Vec<&str> = items.split(':').collect();
        details.insert(value[0].to_string(), value[1].to_string());
    }
    details
}

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

fn check(id: HashMap<String, String>) -> bool {
    let key_to_check = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    //println!("the id to check is {:#?}", id);
    for f in key_to_check.iter() {
        //println!("the key to check is {}", f);

        if !id.contains_key(&f.to_string()) {
            //println!("it missed the valid key, exit");
            return false;
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
            count += 1;
        }
    }

    println!("There are {} valid IDs", count);
    Ok(())
}
