use std::error::Error;

fn question2(mut _s: Vec<usize>) -> Result<[usize; 3], &'static str> {
    Err("Cannot find the window to fit target number.")
}

fn question1(mut s: Vec<usize>) -> Result<[usize; 3], &'static str> {
    let mut jolts: [usize; 3] = [0; 3];

    s.sort();
    //println!("sorted queue is {:#?}", s);
    jolts[s[0] - 1] += 1;
    for i in 1..s.len() {
        jolts[s[i] - s[i - 1] - 1] += 1;
    }
    jolts[2] += 1;
    Ok(jolts)
}
use aoc2020::*;
fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file()?;
    //println!("{:#?}", data);
    let data: Vec<usize> = data.iter().map(|s| s.parse::<usize>().unwrap()).collect();
    match question1(data.to_owned()) {
        Ok(jolts) => {
            println!("There are {} 1-jolt difference, {} 2-jolts differences and {} 3-jolts differences, and the result of question 1 is {}",jolts[0], jolts[1], jolts[2], jolts[0]*jolts[2]);
        }
        Err(x) => println!("Error processing the input data: {:?}", x),
    };
    match question2(data) {
        Ok(jolts) => {
            println!("The sequency from position {:#?}", jolts);
        }
        Err(x) => println!("Error processing the input data: {:?}", x),
    };
    Ok(())
}
