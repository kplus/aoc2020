use std::error::Error;
use std::fs;
use std::path::Path;

fn question1(data: Vec<String>) -> Result<usize, &'static str> {
    const PREAMBLE: usize = 25;
    let whole_array: Vec<usize> = data.iter().map(|s| s.parse::<usize>().unwrap()).collect();
    //println!("The whole array is {:#?}", whole_array);
    let check = &whole_array[PREAMBLE..];
    'next_num: for (i, &num) in check.iter().enumerate() {
        let preamble = &whole_array[i..i + PREAMBLE];
        //println!("Checking {} with preamble {:#?}", num, preamble);
        for (j, first) in preamble.iter().enumerate() {
            for second in &preamble[j + 1..] {
                if (first + second) == num {
                    //println!(
                    //    "The number been checked is {} at postion {}, it can be made up by {} + {}",
                    //    num, i, first, second
                    //);
                    continue 'next_num;
                }
            }
        }
        //println!("The first number is found: {}", num);
        return Ok(num);
    }
    Err("Cannot find first number.")
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
    //println!("{:#?}", data);

    match question1(data) {
        Ok(x) => println!("The result is {}", x),
        Err(x) => println!("Error processing the input data: {:?}", x),
    };
    Ok(())
}
