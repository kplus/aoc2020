use std::cmp::PartialEq;
use std::collections::HashSet;
use std::error::Error;
use std::hash::Hash;

// `use` trait to get functionality
use integer_sqrt::IntegerSquareRoot;
use simple_grid::Grid;

use aoc2020::*;

trait HasGrid{
    fn grid(&self) -> &Grid<char>;
    fn degree(&self) -> usize;
    fn line_up(&self, s: &[char]) -> Option<Grid<char>>;
}
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

impl HasGrid for SQUARE{        
    fn grid(&self) -> &Grid<char> {
        &self.grid
    }

    fn degree(&self) -> usize {
        self.degree
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

    fn rotate(&self) -> Self {
        Self {
            degree: self.degree,
            grid: self.grid.rotate_cw(),
            id: self.id,
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


    // Build from line pool
    fn from_line(l_pool: &mut HashSet<LINE>, num: usize) -> Self {
        let base_line = l_pool.iter().next().unwrap(); // randomly pick an element from the pool to start filling
        let mut front_ids = (base_line.head_id(), base_line.tail_id());
        let mut back_ids = (base_line.head_id(), base_line.tail_id());
        let mut remove_cache: Vec<LINE> = Vec::new();
        remove_cache.push(base_line.to_owned());

        let mut grid: Grid<char> = Grid::new(0, 0, [].to_vec());
        let mut front :Vec<char>= Vec::new();
        let mut back:Vec<char> = Vec::new();


        // first try to build with one direction
        let mut grid = fill_one_direction(
            base_line.to_owned(),
            &l_pool,
            &mut remove_cache,
            &mut front_ids,
            &mut back_ids)
        //println!("grid becomes {:?} afterwards", grid);
        let id = front_ids.0 * front_ids.1 * back_ids.0 * back_ids.1;
        println!("ids are {:?}, {:?}", front_ids, back_ids);
        Self {
            degree: grid.height(),
            grid,
            id,
        }
    }

    fn id(&self) -> usize {
        self.id
    }
}

fn fill_one_direction<T:HasGrid + PartialEq + Clone>(
    base: T,
    pool: &HashSet<T>,
    remove_cache: &mut Vec<T>,
    front_ids: &mut (usize, usize),
    back_ids: &mut (usize, usize),
) -> Grid<char> {
    let mut grid = base.grid().to_owned();
    let mut front: Vec<char> = grid.row_iter(base.degree() - 1).cloned().collect();
    let back: Vec<char> = grid.row_iter(0).cloned().collect();
    let mut reversed = false;
    'next_postion: loop {
        //println!(
        //   "grid is {:?}, front is {:?}, the square to remove is id {}, s pool has {} squares",
        //    grid,
        //    front,
        //    square_to_remove.id(),
        //    s_pool.len()
        //);
        for entity in pool.iter() {
            if remove_cache.contains(entity) {
                continue;
            }
            if let Some(e_grid) = entity.line_up(&front) {
                println!("line up found with square id {}", entity.id());
                let whole_vec: Vec<char> = grid
                    .cell_iter()
                    .chain(e_grid.cell_iter())
                    .cloned()
                    .collect();
                grid = Grid::new(grid.width(), grid.height() + e_grid.height(), whole_vec);
                front = e_grid.row_iter(entity.degree() - 1).cloned().collect();
                front_ids = (entity.id(), entity.id());
                remove_cache.push(entity.to_owned());
                continue 'next_postion;
            }
        } //get to the edge of one direction
        if !reversed {
            reversed = true;
            println!("start reverse line up");
            grid = grid.flip_vertically().to_owned();
            front = back.to_owned();
            back_ids = front_ids;
            continue;
        }
        break;
    }
    grid
}
// An one dimension combinatin of SQUAREs, could be one
// string in orignal input file or a full x-axis generated
// from tiles(or image if extended)
//
// It only build from SQUARE
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct LINE {
    grid: Grid<char>, //content of the LINE, as 2 dimension vectors
    head_id: usize,
    tail_id: usize,
}
impl HasGrid for LINE{        
    fn grid(&self) -> &Grid<char>{
 &self.grid}

    fn degree(&self) -> usize {
        self.grid.height()
    }
    fn line_up(&self, s: &[char]) -> Option<Grid<char>> {
        //println!("checking string {:?} with tile {}", s, self.id);
        let top: Vec<char> = self.grid.row_iter(0).cloned().collect();
        let buttom: Vec<char> = self.grid.row_iter(self.height() - 1).cloned().collect();
        //println!(
        //    "borders of tile are top {:?}\nbuttom {:?}\nleft {:?}\nright {:?}",
        //    top, buttom, left, right
        //);
        let mut s = s.to_owned();
        if s == top {
            return Some(self.grid().to_owned());
        } else if s == buttom {
            return Some(self.grid().flip_vertically());
        }
        s.reverse();
        if s == top {
            return Some(self.grid().flip_horizontally());
        } else if s == buttom {
            return Some(self.grid().flip_horizontally().flip_vertically());
        }
        None
    }
}

impl LINE {
    fn new() -> Self {
        LINE {
            grid: Grid::new(0, 0, [].to_vec()),
            head_id: 0,
            tail_id: 0,
        }
    }

    fn height(&self) -> usize {
        self.grid.height()
    }
    fn grid(&self) -> &Grid<char> {
        &self.grid
    }
    fn head_id(&self) -> usize {
        self.head_id
    }
    fn tail_id(&self) -> usize {
        self.tail_id
    }


    // Build a LINE from SQUARE pool, the first item to use is randomly picked
    fn from_square(s_pool: &mut HashSet<SQUARE>, num: usize) -> Self {
        let base_square = s_pool.iter().next().unwrap(); // randomly pick an element from the pool to start filling
        let mut head_id = base_square.id();
        let mut tail_id = base_square.id();
        let mut remove_cache: Vec<SQUARE> = Vec::new();
        remove_cache.push(base_square.to_owned());

        // first try to build with one direction
        let mut grid = fill_one_direction(
            base_square.to_owned(),
            &s_pool,
            &mut remove_cache,
            &mut head_id,
            &mut tail_id,
        );

        // if first build doesn't fit, try anohter build
        // with base rotated, it should guranteed to succeed
        if remove_cache.len() != num {
            println!("wrong direction, start new scan!");
            head_id = base_square.id();
            tail_id = base_square.id();
            remove_cache.clear();
            remove_cache.push(base_square.to_owned());

            grid = fill_one_direction(
                base_square.rotate(),
                &s_pool,
                &mut remove_cache,
                &mut head_id,
                &mut tail_id,
            );
        }
        for s in remove_cache {
            s_pool.remove(&s);
        }
        //println!("grid got is {:?}", grid);
        grid = grid.transpose();
        Self {
            grid,
            head_id,
            tail_id,
        }
    }
}

fn question2(_data: Vec<String>) -> Result<usize, &'static str> {
    Err("Cannot find second number.")
}
//redoing: use proper modeling
fn question1(data: Vec<String>) -> Result<usize, &'static str> {
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
    println!("get a {} long l pool {:?}", l_pool.len(), l_pool);
    let image = SQUARE::from_line(&mut l_pool, grade);
    println!("image is {:?}", image);
    Ok(image.id())
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
