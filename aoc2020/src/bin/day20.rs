use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::hash::Hash;

// `use` trait to get functionality
use integer_sqrt::IntegerSquareRoot;

use aoc2020::*;

#[derive(PartialEq)]
enum BorderDirection {
    LEFT,
    RIGHT,
    TOP,
    BOTTUM,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
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
    fn get_border(&self, border: &BorderDirection) -> String {
        match border {
            BorderDirection::LEFT => self.left.to_owned(),
            BorderDirection::RIGHT => self.right.to_owned(),
            BorderDirection::TOP => self.top.to_owned(),
            BorderDirection::BOTTUM => self.bottum.to_owned(),
        }
    }

    //todo: Check if current tile lines up with the front
    fn line_up(&self, s: &str, b: &BorderDirection) -> Option<String> {
        if *s == self.bottum {
            return Some(self.top.to_owned());
        } else if *s == self.top {
            return Some(self.bottum.to_owned());
        } else if *s == self.left {
            return Some(self.right.to_owned());
        } else if *s == self.right {
            return Some(self.left.to_owned());
        }

        let r: String = s.chars().rev().collect();

        if r == self.bottum {
            return Some(self.top.chars().rev().collect::<String>());
        } else if *s == self.top {
            return Some(self.bottum.chars().rev().collect::<String>());
        } else if *s == self.left {
            return Some(self.right.chars().rev().collect::<String>());
        } else if *s == self.right {
            return Some(self.left.chars().rev().collect::<String>());
        }
        None
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

    fn get_end(&self, b: &BorderDirection) -> isize {
        match b {
            BorderDirection::LEFT => self.x_shift,
            BorderDirection::RIGHT => self.grade as isize + self.x_shift,
            BorderDirection::TOP => self.grade as isize + self.y_shift,
            BorderDirection::BOTTUM => self.y_shift,
        }
    }
    fn set_range(&mut self, end: isize, b: &BorderDirection) {
        match b {
            BorderDirection::LEFT | BorderDirection::RIGHT => {
                self.x_shift = end + 1 - self.grade as isize
            }
            BorderDirection::TOP | BorderDirection::BOTTUM => {
                self.y_shift = end + 1 - self.grade as isize
            }
        };
    }
    fn get_id(&self, x: isize, y: isize) -> usize {
        self.map.get(&(x, y)).unwrap().get_id()
    }
    fn get_front(&self, x: isize, b: &BorderDirection) -> String {
        self.map.get(&(x, 0)).unwrap().get_border(b)
    }

    fn store(&mut self, tile: TILE, x: isize, y: isize, b: &BorderDirection) {
        if *b == BorderDirection::LEFT || *b == BorderDirection::RIGHT {
            self.map.insert((x, 0), tile);
        } else {
            self.map.insert((x, y), tile);
        }
    }
}
fn question2(data: Vec<String>) -> Result<usize, &'static str> {
    Err("Cannot find second number.")
}

// Fill remain TILEs in given dimension
// This is done by go through one direction until the edge,
// and go reverse from starting point to edge on the other end
fn fill_one_direction(
    tiles_pool: &mut HashSet<TILE>,
    image: &mut IMAGE,
    direction: BorderDirection,
    x: isize,
) {
    let end = image.get_end(&direction);
    let mut front = image.get_front(x, &direction);
    for i in 0..end {
        for tile in tiles_pool.iter().cloned() {
            match tile.line_up(&front, &direction) {
                Some(f) => {
                    front = f;
                    image.store(tile.to_owned(), x, i, &direction);
                    tiles_pool.remove(&tile);
                    break;
                } // break into next position in the image
                None => {
                    continue;
                }
            }
        }
        // not line up found, it hits the edge
        image.set_range(i, &direction);
    }
}

fn question1(data: Vec<String>) -> Result<usize, &'static str> {
    let mut product = 1;
    let mut tiles_pool = HashSet::new();
    for s in data {
        tiles_pool.insert(TILE::from_str(s));
    }
    //println!(
    //    "the tiles pool is {:#?}, the length is {}",
    //    tiles_pool,
    //    tiles_pool.len()
    //);

    let grade = tiles_pool.len().integer_sqrt();
    //println!("grade is {}", grade);
    let mut image = IMAGE::new(grade);

    fill_one_direction(&mut tiles_pool, &mut image, BorderDirection::RIGHT, 0);
    let x_start = image.get_end(&BorderDirection::LEFT);
    if x_start < 0 {
        // if there is a shift
        fill_one_direction(&mut tiles_pool, &mut image, BorderDirection::LEFT, 0);
    }
    let x_end = grade as isize + x_start;
    for i in x_start..x_end {
        fill_one_direction(&mut tiles_pool, &mut image, BorderDirection::TOP, i);
        let y_start = image.get_end(&BorderDirection::BOTTUM);
        if y_start < 0 {
            // if there is a shift
            fill_one_direction(&mut tiles_pool, &mut image, BorderDirection::BOTTUM, i);
        }
    }

    let y_start = image.get_end(&BorderDirection::BOTTUM);
    let y_end = grade as isize + y_start;
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
