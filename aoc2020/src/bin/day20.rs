use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::hash::Hash;

// `use` trait to get functionality
use integer_sqrt::IntegerSquareRoot;

use aoc2020::*;

#[derive(PartialEq, Debug)]
enum BorderDirection {
    LEFT,
    RIGHT,
    TOP,
    BOTTUM,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct TILE {
    id: usize,
    horizon: Vec<String>,
    vertical: Vec<String>,
}

impl TILE {
    // Create TILE from block strings
    fn from_str(s: String) -> Self {
        println!("create tile from string {:?}", s);
        let v: Vec<&str> = s.lines().map(|s| s.trim()).collect();

        let id = v[0]
            .split(|c| c == ' ' || c == ':')
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let grade = v.len() - 1;
        let mut horizon = Vec::new();
        let mut vertical = Vec::new();
        let top = v[1].to_string();
        let bottum = v[grade].to_string();
        let mut left = String::new();
        let mut right = String::new();
        for line in v.iter().skip(1) {
            let c: Vec<char> = line.chars().collect();
            left.push(c[0]);
            right.push(c[grade - 1]);
        }
        horizon.push(left);
        horizon.push(right);
        vertical.push(top);
        vertical.push(bottum);
        TILE {
            id,
            horizon,
            vertical,
        }
    }
    /* seems we don't really need these method at moment, as we supply the right front line directly
        //todo: Rotate TILE by multiple of 90 degrees
        fn rotate(&mut self, angle: usize) {}

        //todo: Flip TILE in horizon direction or vertical direction
        fn flip(&mut self, horizon: bool) {}
    */
    fn tile_id(&self) -> usize {
        self.id
    }
    // we don't really care about which border is it, as long as it's on same direction
    fn get_border(&mut self, border: &BorderDirection) -> String {
        match border {
            BorderDirection::LEFT | BorderDirection::RIGHT => {
                let ret = self.horizon[0].to_owned();
                self.horizon.remove(0);
                ret
            }
            BorderDirection::TOP | BorderDirection::BOTTUM => {
                if !self.vertical.is_empty() {
                    let ret = self.vertical[0].to_owned();
                    self.vertical.remove(0);
                    ret
                } else {
                    let ret = self.horizon[0].to_owned();
                    self.horizon.remove(0);
                    ret
                }
            }
        }
    }

    // Check if current tile lines up with the front
    // Return opposite line if lined up or none if not line up
    fn line_up(&mut self, s: &String) -> Option<String> {
        println!("checking string {} with tile {}", s, self.id);
        let mut ret = String::new();
        if self.horizon.contains(s) {
            if self.horizon[0] == *s {
                ret.push_str(self.horizon[1].as_str());
            } else {
                ret.push_str(self.horizon[0].as_str());
            }
            self.horizon.clear();
            return Some(ret);
        } else if self.vertical.contains(s) {
            if self.vertical[0] == *s {
                ret.push_str(self.vertical[1].as_str());
            } else {
                ret.push_str(self.vertical[0].as_str());
            }
            self.vertical.clear();
            return Some(ret);
        }

        let r: String = s.chars().rev().collect();
        println!("checking reversed string {} with tile {}", r, self.id);
        if self.horizon.contains(&r) {
            if self.horizon[0] == *r {
                ret = self.horizon[1].to_owned();
            } else {
                ret = self.horizon[0].to_owned();
            }
            self.horizon.clear();
            return Some(ret);
        } else if self.vertical.contains(&r) {
            if self.vertical[0] == *r {
                ret = self.vertical[1].to_owned();
            } else {
                ret = self.vertical[0].to_owned();
            }
            self.vertical.clear();
            return Some(ret);
        }
        None
    }
}

#[derive(Debug)]
struct IMAGE {
    x_shift: isize, // image coordinate origin related to first tile in x-axis
    y_shift: isize, // image coordinate origin related to first tile in y-axis
    grade: usize,   // grade of image
    map: HashMap<(isize, isize), TILE>, // map for confirmed TILEs
}

impl IMAGE {
    //todo: Initialization of Image
    fn new(grade: usize) -> Self {
        IMAGE {
            x_shift: 0,
            y_shift: 0,
            grade,
            map: HashMap::new(),
        }
    }

    fn get_end(&self, b: &BorderDirection) -> isize {
        match b {
            BorderDirection::LEFT => self.x_shift,
            BorderDirection::RIGHT => self.grade as isize + self.x_shift - 1,
            BorderDirection::TOP => self.grade as isize + self.y_shift - 1,
            BorderDirection::BOTTUM => self.y_shift,
        }
    }
    fn set_range(&mut self, end: isize, b: &BorderDirection) {
        match b {
            BorderDirection::RIGHT => self.x_shift = end + 1 - self.grade as isize,
            BorderDirection::TOP => self.y_shift = end + 1 - self.grade as isize,
            _ => {}
        };
        //println!(
        //    "passed in end is {}, x shift set to {}, y shift set to {}",
        //    end, self.x_shift, self.y_shift
        //);
    }
    fn get_id(&self, x: isize, y: isize) -> usize {
        println!("x is {}, y is {}", x, y);
        self.map.get(&(x, y)).unwrap().tile_id()
    }
    fn get_front(&mut self, x: isize, b: &BorderDirection) -> String {
        if self.map.is_empty() {
            return String::from("");
        }
        //println!("try to get border line {:?} for {}-0", b, x);
        self.map.get_mut(&(x, 0)).unwrap().get_border(b)
    }

    fn store(&mut self, tile: TILE, x: isize, i: isize, b: &BorderDirection) {
        if *b == BorderDirection::LEFT || *b == BorderDirection::RIGHT {
            self.map.insert((i, 0), tile);
        } else {
            self.map.insert((x, i), tile);
        }
    }

    fn map_is_empty(&self) -> bool {
        self.map.is_empty()
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
    //println!("end is {}, x is {}", end, x);
    let mut front = image.get_front(x, &direction);
    //println!("front is {:?}", front);

    for i in 0..=end.abs() {
        let mut index = i;
        if end < 0 {
            index *= -1;
        }
        //println!("now at postion {}, front is {:?}", index, front);
        let mut added_new_tile = false;
        for tile in tiles_pool.iter().cloned() {
            if image.map_is_empty() {
                // println!("put first tile in");
                image.store(tile.to_owned(), 0, 0, &direction);
                added_new_tile = true;
                front = image.get_front(0, &direction);
                tiles_pool.remove(&tile);
                break;
            }
            if i == 0 {
                added_new_tile = true;
                break;
            }
            let mut tile_change = tile.to_owned();
            match tile_change.line_up(&front) {
                Some(f) => {
                    //println!(
                    //    "line up found, update front to {:?}, tile stored to {} - {}",
                    //    f, x, index
                    // );
                    front = f;
                    image.store(tile_change, x, index, &direction);
                    added_new_tile = true;
                    tiles_pool.remove(&tile);
                    // println!("i is {} at end of filling cycle", index);
                    break;
                } // break into next position in the image
                None => {
                    //println!("no line up found, move to next tile");
                }
            }
        }
        if !added_new_tile {
            image.set_range(index - 1, &direction);
            break;
        }
        image.set_range(index, &direction);
    }
}

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
    //println!("grade is {}", grade);
    let mut image = IMAGE::new(grade);

    fill_one_direction(&mut tiles_pool, &mut image, BorderDirection::RIGHT, 0);
    let x_start = image.get_end(&BorderDirection::LEFT);
    //println!("x start is {}", x_start);
    if x_start < 0 {
        // if there is a shift
        fill_one_direction(&mut tiles_pool, &mut image, BorderDirection::LEFT, 0);
    }
    let x_end = grade as isize + x_start - 1;
    println!("x start is {}, end is {}", x_start, x_end);
    for i in x_start..=x_end {
        println!("start filling colum {}", i);
        fill_one_direction(&mut tiles_pool, &mut image, BorderDirection::TOP, i);
        let y_start = image.get_end(&BorderDirection::BOTTUM);
        if y_start < 0 {
            // if there is a shift
            fill_one_direction(&mut tiles_pool, &mut image, BorderDirection::BOTTUM, i);
        }
    }

    println!("The tile pool is {:#?}, image is {:#?}", tiles_pool, image);
    let y_start = image.get_end(&BorderDirection::BOTTUM);
    let y_end = grade as isize + y_start - 1;
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
