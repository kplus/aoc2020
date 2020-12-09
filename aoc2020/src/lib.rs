use std::env::current_exe;
use std::error::Error;
use std::fs;
use std::path::Path;

// [in]     Path of file to read details from
// [out     Arrary of String for each lines
pub fn load_file() -> Result<Vec<String>, Box<dyn Error>> {
    let full_path = current_exe()?;
    let file = full_path.file_name().unwrap();
    let path = Path::new("../inputs").join(file);
    let input = fs::read_to_string(path)?;
    let mut out = Vec::new();
    for line in input.lines() {
        out.push(line.to_string());
    }
    Ok(out)
}
