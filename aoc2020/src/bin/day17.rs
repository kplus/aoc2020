use std::collections::HashMap;
use std::error::Error;

use aoc2020::*;

#[derive(Debug)]
struct CUBE {
    coordinate_x: usize,
    coordinate_y: usize,
    coordinate_z: usize,
    active: bool,
    neigbhor_count: usize,
}
impl CUBE {
    fn new(coordinate_x: usize, coordinate_y: usize, coordinate_z: usize) -> Self {
        CUBE {
            coordinate_x,
            coordinate_y,
            coordinate_z,
            active: false,
            neigbhor_count: 1,
        }
    }

    fn set_active(&mut self) {
        self.active = true;
    }

    fn add_neighbor_count(&mut self) {
        self.neigbhor_count += 1;
    }

    fn propogate(&self, grid: &mut HashMap<(usize, usize, usize), CUBE>) {
        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    if x | y | z != 0 {
                        let coordinate_x = self.coordinate_x + x;
                        let coordinate_y = self.coordinate_y + y;
                        let coordinate_z = self.coordinate_z + z;
                        let key = (coordinate_x, coordinate_y, coordinate_z);
                        match grid.get_mut(&key) {
                            Some(cube) => {
                                cube.add_neighbor_count();
                            }
                            None => {
                                grid.insert(
                                    key,
                                    CUBE::new(coordinate_x, coordinate_y, coordinate_z),
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

fn question2(data: Vec<String>) -> Result<usize, &'static str> {
    Err("Cannot find second number.")
}

// Do a cycle and proprgate the enenry
// [in]     the existing grid
// [out]    new grid after cycle
fn cycle(old_grid: HashMap<(usize, usize, usize), CUBE>) -> HashMap<(usize, usize, usize), CUBE> {
    let mut grid: HashMap<(usize, usize, usize), CUBE> = HashMap::new();

    for cube in old_grid.values() {
        cube.propogate(&mut grid);
    }

    //todo: retain only active ones
    //grid.retain(|&k,&v|);

    grid
}

// Initilise grid from input sting
fn init_grid(data: Vec<String>) -> HashMap<(usize, usize, usize), CUBE> {
    let mut grid: HashMap<(usize, usize, usize), CUBE> = HashMap::new();

    for (y, line_x) in data.iter().enumerate() {
        for (x, state) in line_x.chars().enumerate() {
            grid.insert((x, y, 0), CUBE::new(x, y, 0));
            if state == '#' {
                grid.get_mut(&(x, y, 0)).unwrap().set_active();
            }
        }
    }
    grid
}

fn question1(data: Vec<String>) -> Result<usize, &'static str> {
    const ROUND: usize = 6;
    let mut grid: HashMap<(usize, usize, usize), CUBE> = init_grid(data);
    println!("Init grid is {:#?}", grid);
    for _i in 0..ROUND {
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
