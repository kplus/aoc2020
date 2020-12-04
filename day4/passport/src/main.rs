use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

fn fill_hash(person: &str) -> HashMap<String, String> {
    let mut details = HashMap::new();
    for items in person.split(' ') {
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
        //    println!("read in peron details:\n{}", person);
        out.push(fill_hash(person));
    }
    Ok(out)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file("../input.txt")?;

    println!("{:#?}", data);

    Ok(())
}
