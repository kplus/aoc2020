use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::hash::Hash;

// `use` trait to get functionality
use integer_sqrt::IntegerSquareRoot;

use aoc2020::*;

#[derive(PartialEq, Eq, Hash, Debug)]
struct TILE {
    id: usize,
    //x: isize,
    //y: isize,
    left: String,
    right: String,
    top: String,
    bottum: String,
}

impl TILE {
    // Create TILE from block strings
    fn from_str(s: String) -> Self {
        //println!("create tile from string {:?}", s);
        let v: Vec<&str> = s.lines().map(|s| s.trim()).collect();

        let id = v[0]
            .split(|c| c == ' ' || c == ':')
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let grade = v.len() - 1;
        let top = v[1].to_string();
        let bottum = v[grade].to_string();
        let mut left = String::new();
        let mut right = String::new();
        for line in v.iter().skip(1) {
            let c: Vec<char> = line.chars().collect();
            left.push(c[0]);
            right.push(c[grade - 1]);
        }
        TILE {
            id,
            left,
            right,
            top,
            bottum,
        }
    }

    //todo: Rotate TILE by multiple of 90 degrees
    fn rotate(&mut self, angle: usize) {}

    //todo: Flip TILE in horizon direction or vertical direction
    fn flip(&mut self, horizon: bool) {}

    fn get_id(&self) -> usize {
        self.id
    }
}

struct IMAGE {
    front: String,                      // the border string to check
    x_shift: isize,                     // image coordinate origin related to first tile in x-axis
    y_shift: isize,                     // image coordinate origin related to first tile in y-axis
    grade: usize,                       // grade of image
    map: HashMap<(isize, isize), TILE>, // map for confirmed TILEs
}

impl IMAGE {
    //todo: Initialization of Image
    fn new(grade: usize) -> Self {
        IMAGE {
            front: String::from(""),
            x_shift: 0,
            y_shift: 0,
            grade,
            map: HashMap::new(),
        }
    }

    fn get_horizon_range(&self) -> (isize, isize) {
        (self.x_shift, self.grade as isize + self.x_shift)
    }
    fn get_vertical_range(&self) -> (isize, isize) {
        (self.y_shift, self.grade as isize + self.y_shift)
    }
    fn get_id(&self, x: isize, y: isize) -> usize {
        self.map.get(&(x, y)).unwrap().get_id()
    }
}
fn question2(data: Vec<String>) -> Result<usize, &'static str> {
    Err("Cannot find second number.")
}

//doing: Fill remain TILEs in given dimension
// This is done by go through one direction until the edge,
// and go reverse from starting point to edge on the other end
fn fill_dimension(tiles_pool: &mut HashSet<TILE>, image: &mut IMAGE, horizon: bool) {}

fn question1(data: Vec<String>) -> Result<usize, &'static str> {
    let mut product = 1;
    let mut tiles_pool = HashSet::new();
    for s in data {
        tiles_pool.insert(TILE::from_str(s));
    }
    println!(
        "the tiles pool is {:#?}, the length is {}",
        tiles_pool,
        tiles_pool.len()
    );

    let grade = tiles_pool.len().integer_sqrt();
    println!("grade is {}", grade);
    let mut image = IMAGE::new(grade);

    fill_dimension(&mut tiles_pool, &mut image, true);

    let (x_start, x_end) = image.get_horizon_range();
    for _i in x_start..x_end {
        fill_dimension(&mut tiles_pool, &mut image, false);
    }

    let (y_start, y_end) = image.get_vertical_range();

    println!(
        "image range is from x {} - {}, y {} - {}",
        x_start, x_end, y_start, y_end
    );
    for (x, y) in [
        (x_start, y_start),
        (x_end, y_start),
        (x_start, y_end),
        (x_end, y_end),
    ]
    .iter()
    {
        product *= image.get_id(*x, *y);
    }
    Ok(product)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file_by_p()?;
    //println!("{:#?}", data);
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

    static TEST_INPUT: &str = r"Tile 2311:
    ..##.#..#.
    ##..#.....
    #...##..#.
    ####.#...#
    ##.##.###.
    ##...#.###
    .#.#.#..##
    ..#....#..
    ###...#.#.
    ..###..###
    
    Tile 1951:
    #.##...##.
    #.####...#
    .....#..##
    #...######
    .##.#....#
    .###.#####
    ###.##.##.
    .###....#.
    ..#.#..#.#
    #...##.#..
    
    Tile 1171:
    ####...##.
    #..##.#..#
    ##.#..#.#.
    .###.####.
    ..###.####
    .##....##.
    .#...####.
    #.##.####.
    ####..#...
    .....##...
    
    Tile 1427:
    ###.##.#..
    .#..#.##..
    .#.##.#..#
    #.#.#.##.#
    ....#...##
    ...##..##.
    ...#.#####
    .#.####.#.
    ..#..###.#
    ..##.#..#.
    
    Tile 1489:
    ##.#.#....
    ..##...#..
    .##..##...
    ..#...#...
    #####...#.
    #..#.#.#.#
    ...#.#.#..
    ##.#...##.
    ..##.##.##
    ###.##.#..
    
    Tile 2473:
    #....####.
    #..#.##...
    #.##..#...
    ######.#.#
    .#...#.#.#
    .#########
    .###.#..#.
    ########.#
    ##...##.#.
    ..###.#.#.
    
    Tile 2971:
    ..#.#....#
    #...###...
    #.#.###...
    ##.##..#..
    .#####..##
    .#..####.#
    #..#.#..#.
    ..####.###
    ..#.#.###.
    ...#.#.#.#
    
    Tile 2729:
    ...#.#.#.#
    ####.#....
    ..#.#.....
    ....#..#.#
    .##..##.#.
    .#.####...
    ####.#.#..
    ##.####...
    ##..#.##..
    #.##...##.
    
    Tile 3079:
    #.#.#####.
    .#..######
    ..#.......
    ######....
    ####.#..#.
    .#...#.##.
    #.#####.##
    ..#.###...
    ..#.......
    ..#.###...";

    #[test]
    fn test_question1() {
        let data: Vec<String> = TEST_INPUT
            .split("\n    \n")
            .map(|s| s.trim().to_string())
            .collect();
        assert_eq!(Ok(20899048083289), question1(data));
    }
    #[test]
    fn test_question2() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Err("Cannot find second number."), question2(data));
    }
}
