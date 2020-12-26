use std::cmp::PartialEq;
use std::collections::HashSet;
use std::error::Error;
use std::hash::Hash;

// `use` trait to get functionality
use integer_sqrt::IntegerSquareRoot;
use simple_grid::Grid;

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
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct SQUARE {
    degree: usize,    //number of pixels in one direction
    grid: Grid<char>, //content of the square, as 2 dimension array(use Grid crate)
    id: usize,        //unique ID for this square
}

impl SQUARE {
    // Create an empty SQUARE
    fn new() -> Self {
        Self {
            degree: 0,
            grid: Grid::new(0, 0, [].to_vec()),
            id: 0,
        }
    }

    // Build from initial input block string
    // it has to call from_LINE, and build from LINE object
    fn from_str(s: String) -> Self {
        //   let mut front = Vec::new();
        //   let mut back = Vec::new();
        let st: Vec<&str> = s.split(':').collect();
        let id = st[0]
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let content: Vec<char> = st[1].chars().filter(|s| !s.is_whitespace()).collect();
        let degree = content.len().integer_sqrt();
        let grid = Grid::new(degree, degree, content);

        Self { degree, grid, id }
    }

    // Check if current tile lines up with the front
    // also rotate/flip the tile to make the lined up line to
    // be the top of tile, as we grew to bottum
    fn line_up(&self, s: &[char]) -> Option<Grid<char>> {
        //println!("checking string {:?} with tile {}", s, self.id);
        let top: Vec<char> = self.grid.row_iter(0).cloned().collect();
        let buttom: Vec<char> = self.grid.row_iter(self.degree() - 1).cloned().collect();
        let left: Vec<char> = self.grid.column_iter(0).cloned().collect();
        let right: Vec<char> = self.grid.column_iter(self.degree() - 1).cloned().collect();
        //println!(
        //    "borders of tile are top {:?}\nbuttom {:?}\nleft {:?}\nright {:?}",
        //    top, buttom, left, right
        //);
        let mut s = s.to_owned();
        if s == top {
            //println!("s {:?} matches with top {:?}", s, top);
            return Some(self.grid().to_owned());
        } else if s == buttom {
            //println!("s {:?} matches with buttom {:?}", s, buttom);
            return Some(self.grid().flip_vertically());
        } else if s == left {
            //println!("s {:?} matches with left {:?}", s, left);
            return Some(self.grid().transpose());
        } else if s == right {
            //println!("s {:?} matches with right {:?}", s, right);
            return Some(self.grid().rotate_ccw());
        }
        s.reverse();
        //println!("s reversed is {:?} ", s);
        if s == top {
            return Some(self.grid().flip_horizontally());
        } else if s == buttom {
            return Some(self.grid().flip_horizontally().flip_vertically());
        } else if s == left {
            return Some(self.grid().rotate_cw());
        } else if s == right {
            return Some(self.grid().rotate_ccw().flip_horizontally());
        }
        None
    }

    //todo: Build from line pool
    fn from_line(l_pool: &mut HashSet<LINE>) -> Self {
        Self {
            degree: 0,
            grid: Grid::new(1, 1, ['a'].to_vec()),
            id: 0,
        }
    }

    fn degree(&self) -> usize {
        self.degree
    }
    fn grid(&self) -> &Grid<char> {
        &self.grid
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
    grid: Grid<char>, //content of the LINE, as 2 dimension vectors
}

impl LINE {
    // Build a LINE from SQUARE pool, the first item to use is randomly picked
    fn from_square(s_pool: &mut HashSet<SQUARE>, num: usize) -> Self {
        let mut grid: Grid<char> = Grid::new(0, 0, [].to_vec());
        let mut front = Vec::new();
        let mut back = Vec::new();
        let mut square_to_remove = SQUARE::new();
        for _i in 0..num {
            //println!(
            //    "grid is {:?}, front is {:?}, the square to remove is id {}, s pool has {} squares",
            //    grid,
            //    front,
            //    square_to_remove.id(),
            //    s_pool.len()
            //);
            for square in s_pool.iter() {
                if grid.height() == 0 {
                    //println!("Init LINE with square id {}", square.id());
                    grid = square.grid.to_owned();
                    back = grid.row_iter(0).cloned().collect();
                    front = grid.row_iter(square.degree() - 1).cloned().collect();
                    square_to_remove = square.to_owned();
                    break;
                }
                if let Some(s_grid) = square.line_up(&front) {
                    //println!("line up found with square id {}", square.id());
                    let whole_vec: Vec<char> = grid
                        .cell_iter()
                        .chain(s_grid.cell_iter())
                        .cloned()
                        .collect();
                    grid = Grid::new(grid.width(), grid.height() + s_grid.height(), whole_vec);
                    front = s_grid.row_iter(square.degree() - 1).cloned().collect();
                    square_to_remove = square.to_owned();
                    break;
                }
            } //get to the edge of one direction
            if square_to_remove.degree() != 0 {
                //println!("to remove square id {}", square_to_remove.id());
                s_pool.remove(&square_to_remove);
                square_to_remove = SQUARE::new();
                continue;
            }
            if grid.height() < grid.width() * num {
                //println!("start reverse line up");
                grid.flip_vertically();
                front = back.to_owned();
                for square in s_pool.iter() {
                    if let Some(s_grid) = square.line_up(&front) {
                        //println!("reverse line up found with square id {}", square.id());
                        let whole_vec: Vec<char> = grid
                            .cell_iter()
                            .chain(s_grid.cell_iter())
                            .cloned()
                            .collect();
                        //println!(
                        //    "grid is {:?}\ns_grid is {:?}\nwhole_vec is {:?}",
                        //    grid, s_grid, whole_vec
                        //);
                        grid = Grid::new(grid.width(), grid.height() + s_grid.height(), whole_vec);
                        //println!("grid becomes {:?} afterwards", grid);
                        front = s_grid.row_iter(grid.width() - 1).cloned().collect();
                        square_to_remove = square.to_owned();
                        break;
                    }
                }
                if square_to_remove.degree() != 0 {
                    //println!("to remove square id {}", square_to_remove.id());
                    s_pool.remove(&square_to_remove);
                    square_to_remove = SQUARE::new();
                    continue;
                }
            } // get to edge of other direction
        }
        grid.transpose();
        //println!("grid becomes {:?} afterwards", grid);
        Self { grid }
    }
}

/*


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


*/
fn question2(_data: Vec<String>) -> Result<usize, &'static str> {
    Err("Cannot find second number.")
}
//redoing: use proper modeling
fn question1(data: Vec<String>) -> Result<usize, &'static str> {
    let mut product = 1;
    let mut s_pool = HashSet::new(); // SQUARE pool, in this case it contains tiles
    for s in data {
        s_pool.insert(SQUARE::from_str(s));
    }
    //println!(
    //    "the tiles pool is {:?}, the length is {}",
    //    s_pool,
    //    s_pool.len()
    //);

    let grade = s_pool.len().integer_sqrt();
    //println!("grade is {}", grade);

    let mut l_pool = HashSet::new(); // LINE pool, store the LINEs generated from the SQUARE pool
    for _i in 0..grade {
        l_pool.insert(LINE::from_square(&mut s_pool, grade));
    }
    //println!("get a {} long l pool {:?}", l_pool.len(), l_pool);
    //doing
    let _image = SQUARE::from_line(&mut l_pool);
    //todo: get the product for question 1 based on Image generated
    product += 1;
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
