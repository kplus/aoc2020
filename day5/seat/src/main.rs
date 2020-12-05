use std::error::Error;
use std::fs;
use std::path::Path;

// [in]     Path of file to read boarding passes from
// [out     Arrary of String, each String is a boarding ID
fn load_file<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Box<dyn Error>> {
    let input = fs::read_to_string(path)?;
    println!("read in content:\n{}", input);

    let mut v: Vec<String> = Vec::new();

    for line in input.lines() {
        v.push(line.to_string());
    }

    Ok(v)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file("../input.txt")?;

    //println!("{:#?}", data);

    let mut count = 0;

    println!("There are {} valid IDs", count);
    Ok(())
}
