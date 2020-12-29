use std::cmp::PartialEq;
use std::collections::HashSet;
use std::error::Error;
use std::hash::Hash;
use std::mem::swap;

// `use` trait to get functionality
use integer_sqrt::IntegerSquareRoot;
use simple_grid::Grid;

use aoc2020::*;

// We use a generic object IMAGE to represent an object
// with pixels, it can be square or line, both the tile
// and final image are entity of this object.(They can be
// extended if bigger image needs to be generated from current
// image.)
//
// a single pixel theoretically should be this object as
// well, but we use native method for it
//
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct IMAGE {
    grid: Grid<char>, //content of the square, as 2 dimension array(use Grid crate)
    id: usize,        //ID for this IMAGE, it's the unique ID for square, or prodoct
                      // of these IDs for generated IMAGEs
}

impl IMAGE {
    fn grid(&self) -> &Grid<char> {
        &self.grid
    }
    fn height(&self) -> usize {
        self.grid.height()
    }
    fn width(&self) -> usize {
        self.grid.width()
    }
    fn id(&self) -> usize {
        self.id
    }

    // rotate grid clockwise by 90 degrees
    fn rotate(&self) -> Self {
        Self {
            grid: self.grid.rotate_cw(),
            id: self.id,
        }
    }

    // Check if current tile lines up with the front
    // also rotate/flip the tile to make the lined up line to
    // be the top of tile, as we grew to bottum
    fn line_up(&self, s: &[char]) -> Option<Grid<char>> {
        let top: Vec<char> = self.grid.row_iter(0).cloned().collect();
        let buttom: Vec<char> = self.grid.row_iter(self.height() - 1).cloned().collect();
        let left: Vec<char> = self.grid.column_iter(0).cloned().collect();
        let right: Vec<char> = self.grid.column_iter(self.width() - 1).cloned().collect();
        let mut s = s.to_owned();
        if s == top {
            return Some(self.grid().to_owned());
        } else if s == buttom {
            return Some(self.grid().flip_vertically());
        } else if s == left {
            return Some(self.grid().transpose());
        } else if s == right {
            return Some(self.grid().rotate_ccw());
        }
        s.reverse();
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

    // Build from initial input block string
    // Will create a square object
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

        Self { grid, id }
    }

    // Build from line pool
    // Will create a square object
    fn from_line(l_pool: HashSet<IMAGE>) -> Self {
        let base_line = l_pool.iter().next().unwrap(); // randomly pick an element from the pool to start filling
        let mut remove_cache: Vec<IMAGE> = Vec::new();
        remove_cache.push(base_line.to_owned());

        // lines can only build up in one direction
        Self::fill_one_direction(base_line.to_owned(), &l_pool, &mut remove_cache)
    }

    // Build a line from square pool, the first item to use is randomly picked
    // It's possible a wrong direction was picked at first, if so, the cached
    // grid and square_to_remove will be dropped, and re-run the building with
    // the square rotated.
    fn from_square(s_pool: &mut HashSet<IMAGE>, num: usize) -> Self {
        let base_square = s_pool.iter().next().unwrap(); // randomly pick an element from the pool to start filling
        let mut remove_cache: Vec<IMAGE> = Vec::new();
        remove_cache.push(base_square.to_owned());

        // first try to build with one direction
        let mut line =
            IMAGE::fill_one_direction(base_square.to_owned(), &s_pool, &mut remove_cache);

        // if first build doesn't fit, try anohter build
        // with base rotated, it should guranteed to succeed
        if remove_cache.len() != num {
            remove_cache.clear();
            remove_cache.push(base_square.to_owned());

            line = IMAGE::fill_one_direction(base_square.rotate(), &s_pool, &mut remove_cache);
        }
        for s in remove_cache {
            s_pool.remove(&s);
        }
        //println!("grid got is {:?}", grid);
        let grid = line.grid().transpose();
        let id = line.id();
        Self { grid, id }
    }

    fn fill_one_direction(
        base: IMAGE,
        pool: &HashSet<IMAGE>,
        remove_cache: &mut Vec<IMAGE>,
    ) -> IMAGE {
        let mut head_id = base.id();
        let mut tail_id = base.id();
        let mut grid = base.grid().to_owned();
        let mut front: Vec<char> = grid.row_iter(base.height() - 1).cloned().collect();
        let back: Vec<char> = grid.row_iter(0).cloned().collect();
        let mut reversed = false;
        'next_postion: loop {
            for entity in pool.iter() {
                if remove_cache.contains(entity) {
                    continue;
                }
                if let Some(mut e_grid) = entity.line_up(&front) {
                    grid.remove_row(grid.height() - 1);
                    e_grid.remove_row(0);
                    let whole_vec: Vec<char> = grid
                        .cell_iter()
                        .chain(e_grid.cell_iter())
                        .cloned()
                        .collect();
                    grid = Grid::new(grid.width(), grid.height() + e_grid.height(), whole_vec);
                    front = e_grid.row_iter(e_grid.height() - 1).cloned().collect();
                    head_id = entity.id();
                    remove_cache.push(entity.to_owned());
                    continue 'next_postion;
                }
            } //get to the edge of one direction
            if !reversed {
                reversed = true;
                grid = grid.flip_vertically().to_owned();
                front = back.to_owned();
                swap(&mut head_id, &mut tail_id);
                continue;
            }
            break;
        }
        IMAGE {
            grid,
            id: tail_id * head_id,
        }
    }

    fn trim(&mut self) {
        self.grid.remove_row(self.height() - 1);
        self.grid.remove_row(0);
        self.grid.remove_column(self.width() - 1);
        self.grid.remove_column(0);
    }
}

//todo: Get the monster pattern index
// [in]     degree of image
// [out]    vector of indexs of all pattern postions related to first #
fn get_monster_pattern(width: usize) -> (Vec<usize>, usize) {
    (Vec::new(), 0)
}

//todo: Check if current postion starts a monster
fn find_monster(pos: &usize, grid: &Grid<char>, pattern: &[usize]) -> bool {
    true
}
fn question(data: Vec<String>) -> Result<usize, &'static str> {
    let mut s_pool = HashSet::new(); // SQUARE pool, in this case it contains tiles
    for s in data {
        s_pool.insert(IMAGE::from_str(s));
    }

    let grade = s_pool.len().integer_sqrt();

    let mut l_pool = HashSet::new(); // LINE pool, store the LINEs generated from the SQUARE pool
    for _i in 0..grade {
        l_pool.insert(IMAGE::from_square(&mut s_pool, grade));
    }
    let mut image = IMAGE::from_line(l_pool);
    image.trim();
    //println!("image is {:?}", image);
    let (pattern, num) = get_monster_pattern(image.height());
    let grid = image.grid();
    let mut pos = 0;
    let mut monster = 0;
    loop {
        if pos >= grid.area() - 1 {
            break;
        }
        if find_monster(&pos, &grid, &pattern) {
            monster += 1;
        }
        pos += 1;
    }
    let habitat = grid.cell_iter().filter(|c| **c == '#').count() - num * monster;
    Ok(habitat)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file_by_p()?;
    //println!("{:#?}", data);
    match question(data) {
        Ok(x) => println!("The result for question 1 is {}", x),
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

    /*
    #[test]
    fn test_question1() {
        let data: Vec<String> = TEST_INPUT
            .split("\n    \n")
            .map(|s| s.trim().to_string())
            .collect();
        assert_eq!(Ok(20899048083289), question1(data));
    }
    */
    #[test]
    fn test_question2() {
        let data: Vec<String> = TEST_INPUT
            .split("\n    \n")
            .map(|s| s.trim().to_string())
            .collect();
        assert_eq!(Ok(273), question(data));
    }
}
