use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::hash::Hash;

// `use` trait to get functionality
use integer_sqrt::IntegerSquareRoot;

use aoc2020::*;

// We use a generic object SQUARE to represent an object
// with same elements on both directions, both the tile
// and final image are entity of this object.(They can be
// extended if bigger image needs to be generated from current
// image.)
//
// a single pixel theoretically should be this object as
// well, but we use native method for it
//
// It only build from original input(see if we can unify this),
// or from LINE
#[derive(PartialEq, Eq, Hash, Debug)]
struct SQUARE {
    degree: usize,          //number of pixels in one direction
    pixels: Vec<Vec<char>>, //content of the square, as 2 dimension vectors
    front: Vec<char>,       //the front line of border, to check adjacention with other SQUARE
    back: Vec<char>,        //the reverse border to check
    id: usize,              //unique ID for this square
}

impl SQUARE {
    // Build from initial input block string
    // it has to call from_LINE, and build from LINE object
    fn from_str(s: String) -> Self {
        let mut pixels = Vec::new();
        let mut front = Vec::new();
        let mut back = Vec::new();
        let mut iter = s.lines();
        let id = iter
            .next()
            .unwrap()
            .split(|c| c == ' ' || c == ':')
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        for line in iter {
            let line: Vec<char> = line.trim().chars().collect();
            front.push(line[line.len() - 1].to_owned());
            back.push(line[0].to_owned());
            pixels.push(line);
        }
        Self {
            degree: pixels.len(),
            pixels,
            front,
            back,
            id,
        }
    }

    // todo: Check if current tile lines up with the front
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

    //todo: Build from line pool
    fn from_line(l_pool: &mut HashSet<LINE>) -> Self {
        Self {
            degree: 0,
            pixels: Vec::new(),
            front: Vec::new(),
            back: Vec::new(),
            id: 0,
        }
    }

    fn degree(&self) -> usize {
        self.degree
    }
    fn pixels(&self) -> Vec<Vec<char>> {
        self.pixels
    }
    fn front(&self) -> Vec<_> {
        self.front
    }
    fn back(&self) -> Vec<_> {
        self.back
    }
    fn id(&self) -> usize {
        self.id
    }
}

// An one dimension combinatin of SQUAREs, could be one
// string in orignal input file or a full x-axis generated
// from tiles(or image if extended)
//
// It only build from SQUARE
#[derive(PartialEq, Eq, Hash, Debug)]
struct LINE {
    x_degree: usize,
    y_degree: usize,
    pixels: Vec<Vec<char>>, //content of the square, as 2 dimension vectors
    front: Vec<char>,       //the front line of border, to check adjacention with other SQUARE
    back: Vec<char>,        //the reverse border to check
}

impl LINE {
    //todo: Build a LINE from SQUARE pool, the first item to use is randomly picked
    fn from_square(s_pool: &mut HashSet<SQUARE>, num: usize) -> Self {
        let mut pixels = Vec::new();
        let mut front = Vec::new();
        let mut back = Vec::new();
        let mut x_degree = 0;
        let mut y_degree = 0;
        for _i in 0..num {
            for square in s_pool.iter() {
                if front.is_empty() {
                    x_degree = square.degree;
                    y_degree = square.degree * num;
                    pixels = square.pixels;
                    front = square.front;
                    back = square.back;
                    continue;
                }
                if square.lines_up(&front) {
                    //note: may need to tweat a bit if square needs rotating/flipping
                    for (l, line) in square.pixels.iter().enumerate() {
                        let mut line = line.to_owned();
                        pixels[l].append(&mut line);
                    }
                    front = square.front;
                    s_pool.remove(square);
                    continue;
                }
            }
        }
        Self {
            x_degree: 0,
            y_degree: 0,
            pixels: Vec::new(),
            front: Vec::new(),
            back: Vec::new(),
        }
    }
}

#[derive(PartialEq, Debug)]
enum BorderDirection {
    LEFT,
    RIGHT,
    TOP,
    BOTTUM,
}

/*
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct TILE {
    id: usize,
    horizon: Vec<String>,
    vertical: Vec<String>,
}

impl TILE {

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

}

#[derive(Debug)]
struct IMAGE {
    x_shift: isize, // image coordinate origin related to first tile in x-axis
    y_shift: isize, // image coordinate origin related to first tile in y-axis
    grade: usize,   // grade of image
    map: HashMap<(isize, isize), TILE>, // map for confirmed TILEs
}

impl IMAGE {
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
*/
fn question2(data: Vec<String>) -> Result<usize, &'static str> {
    Err("Cannot find second number.")
}
//redoing: use proper modeling
fn question1(data: Vec<String>) -> Result<usize, &'static str> {
    let mut product = 1;
    let mut s_pool = HashSet::new(); // SQUARE pool, in this case it contains tiles
    for s in data {
        s_pool.insert(SQUARE::from_str(s));
        //println!("current s pool is {:#?}", s_pool)
    }
    //println!(
    //    "the tiles pool is {:#?}, the length is {}",
    //    s_pool,
    //    s_pool.len()
    //);

    let grade = s_pool.len().integer_sqrt();
    println!("grade is {}", grade);

    let mut l_pool = HashSet::new(); // LINE pool, store the LINEs generated from the SQUARE pool
    for _i in 0..grade {
        //doing
        l_pool.insert(LINE::from_square(&mut s_pool, grade));
    }
    let image = SQUARE::from_line(&mut l_pool);
    //todo: get the product for question 1 based on Image generated
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
