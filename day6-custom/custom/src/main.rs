use std::error::Error;
use std::fs;
use std::path::Path;

// [in]     Path of file to read personal details from
// [out     Arrary of Hashmaps, each Hashmap stores full details of a single person
fn load_file<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Box<dyn Error>> {
    let input = fs::read_to_string(path)?;
    //println!("read in content:\n{}", input);

    let mut out = Vec::new();
    for person in input.split("\n\n") {
        //println!("read in peron details:\n{}", person);
        out.push(person.to_string());
    }
    Ok(out)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file("../input.txt")?;

    println!("{:#?}", data);

    Ok(())
}
