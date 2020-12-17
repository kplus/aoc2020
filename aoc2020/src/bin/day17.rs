use std::collections::HashMap;
use std::error::Error;

use aoc2020::*;

struct CUBE {
    coordinate_x: usize,
    coordinate_y: usize,
    coordinate_z: usize,
    active: bool,
    neigbhor_count: usize,
}

fn question2(data: Vec<String>) -> Result<usize, &'static str> {
    Err("Cannot find second number.")
}

//todo: Do a cycle and proprgate the enenry
// [in]     the existing grid
// [out]    new grid after cycle
fn cycle(old_grid: HashMap<(usize, usize, usize), CUBE>) -> HashMap<(usize, usize, usize), CUBE> {
    let grid: HashMap<(usize, usize, usize), CUBE> = cycle(old_grid);
    grid
}

//todo: Initilise grid from input sting
fn init_grid(data: Vec<String>) -> HashMap<(usize, usize, usize), CUBE> {
    let mut grid: HashMap<(usize, usize, usize), CUBE> = HashMap::new();
    grid
}

fn question1(data: Vec<String>) -> Result<usize, &'static str> {
    const ROUND: usize = 6;
    let mut grid: HashMap<(usize, usize, usize), CUBE> = init_grid(data);
    for i in 0..ROUND {
        grid = cycle(grid);
    }

    Ok(grid.len())
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file()?;
    println!("{:#?}", data);
    match question1(data.to_owned()) {
        Ok(x) => println!("The result for question 1 is {}", x),
        Err(x) => eprintln!("Error processing the input data: {:?}", x),
    };
    match question2(data) {
        Ok(x) => println!("The sequency from position {}", x),
        Err(x) => eprintln!("Error processing the input data: {:?}", x),
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r".#.
    ..#
    ###";

    #[test]
    fn test_question1() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Ok(112), question1(data));
    }
    #[test]
    fn test_question2() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Err("Cannot find second number."), question2(data));
    }
}
