use std::error::Error;

use aoc2020::*;

#[derive(Clone, Copy, PartialEq, Debug)]
enum STATE {
    Occupied,
    Empty,
    Floor,
    Edge,
}

#[derive(Clone, Copy, Debug)]
struct SEAT {
    state: STATE,
    old_state: STATE,
    changed: bool,
    row: usize,
    col: usize,
}

impl SEAT {
    fn from_char(c: char, row: usize, col: usize) -> Self {
        let state = match c {
            'L' => STATE::Empty,
            '.' => STATE::Floor,
            _ => STATE::Edge,
        };
        SEAT {
            state,
            old_state: state,
            changed: false,
            row,
            col,
        }
    }
    fn get_state(&self) -> STATE {
        self.state
    }
    fn get_old_state(&self) -> STATE {
        self.old_state
    }
    fn _check_changed(&self) -> bool {
        self.changed
    }

    fn get_state_count(&self) -> u8 {
        (self.state == STATE::Occupied) as u8
    }
    fn get_old_state_count(&self) -> u8 {
        (self.old_state == STATE::Occupied) as u8
    }

    fn pre_update(&mut self, m: Vec<Vec<SEAT>>) -> u8 {
        self.old_state = self.state;
        let mut adj = 0;
        for i in 0..3 {
            adj += m[self.row - 1][self.col + 1 - i].get_old_state_count();
            adj += m[self.row + 1][self.col + 1 - i].get_state_count();
        }
        adj += m[self.row][self.col - 1].get_old_state_count();
        adj += m[self.row][self.col + 1].get_state_count();
        adj
    }

    fn update(&mut self, m: Vec<Vec<SEAT>>) -> bool {
        match self.state {
            STATE::Empty => {
                if self.pre_update(m) == 0 {
                    self.state = STATE::Occupied;
                }
            }
            STATE::Occupied => {
                if self.pre_update(m) >= 4 {
                    self.state = STATE::Empty;
                }
            }
            _ => {
                self.changed = false;
            }
        }
        self.changed = self.state != self.old_state;
        self.changed
    }

    fn first_seat_before(&self, m: &Vec<Vec<SEAT>>, row_step: &i32, col_step: &i32) -> u8 {
        let row = (self.row as i32 + *row_step) as usize;
        let col = (self.col as i32 + *col_step) as usize;

        // println!(
        //    "steps are {}, {}, and row is {}, col is {}, self row and col are: {} {}",
        //     row_step, col_step, row, col, self.row, self.col
        // );
        match self.get_old_state() {
            STATE::Edge => 0,
            STATE::Occupied => 1,
            STATE::Empty => 0,
            STATE::Floor => m[row][col].first_seat_before(m, row_step, col_step),
        }
    }
    fn first_seat_after(&self, m: &Vec<Vec<SEAT>>, row_step: &i32, col_step: &i32) -> u8 {
        let row = (self.row as i32 + *row_step) as usize;
        let col = (self.col as i32 + *col_step) as usize;

        // println!(
        //     "steps are {}, {}, and row is {}, col is {}, self row and col are: {} {}",
        //     row_step, col_step, row, col, self.row, self.col
        //);
        match self.get_state() {
            STATE::Edge => 0,
            STATE::Occupied => 1,
            STATE::Empty => 0,
            STATE::Floor => m[row][col].first_seat_after(m, row_step, col_step),
        }
    }
    fn pre_update2(&mut self, m: Vec<Vec<SEAT>>) -> u8 {
        self.old_state = self.state;
        let mut adj = 0;
        let before = [(-1, -1), (-1, 0), (-1, 1), (0, -1)];
        let after = [(0, 1), (1, -1), (1, 0), (1, 1)];
        for (x, y) in before.iter() {
            let row = (self.row as i32 + *x) as usize;
            let col = (self.col as i32 + *y) as usize;
            //println!("x, y are {}, {}, and row is {}, col is {}", x, y, row, col);
            adj += m[row][col].first_seat_before(&m, x, y);
        }
        for (x, y) in after.iter() {
            let row = (self.row as i32 + *x) as usize;
            let col = (self.col as i32 + *y) as usize;
            //println!("x, y are {}, {}, and row is {}, col is {}", x, y, row, col);
            adj += m[row][col].first_seat_after(&m, x, y);
        }
        adj
    }

    fn update2(&mut self, m: Vec<Vec<SEAT>>) -> bool {
        match self.state {
            STATE::Empty => {
                if self.pre_update2(m) == 0 {
                    self.state = STATE::Occupied;
                }
            }
            STATE::Occupied => {
                if self.pre_update2(m) >= 5 {
                    self.state = STATE::Empty;
                }
            }
            _ => {
                self.changed = false;
            }
        }
        self.changed = self.state != self.old_state;
        self.changed
    }
}

fn flip(mx: &mut Vec<Vec<SEAT>>) -> bool {
    let mut unstable = false;
    let row = mx.len();
    let col = mx[0].len();
    for r in 0..row {
        for c in 0..col {
            let new_matrix = mx.to_owned();
            unstable |= mx[r][c].update(new_matrix);
        }
    }
    //for seat in mx.iter_mut().flatten() {
    // }
    unstable
}

fn flip2(mx: &mut Vec<Vec<SEAT>>) -> bool {
    let mut unstable = false;
    let row = mx.len();
    let col = mx[0].len();
    for r in 0..row {
        for c in 0..col {
            let new_matrix = mx.to_owned();
            unstable |= mx[r][c].update2(new_matrix);
        }
    }
    //for seat in mx.iter_mut().flatten() {
    // }
    unstable
}
fn question2(v: Vec<String>) -> Result<usize, &'static str> {
    let row = v.len() + 2;
    let col = v[0].len() + 2;
    let mut matrix: Vec<Vec<SEAT>> = vec![vec![SEAT::from_char('E', 0, 0); col]; row];

    // initialise matrix
    for r in 1..row - 1 {
        for c in 1..col - 1 {
            let ch = v[r - 1].chars().nth(c - 1).unwrap();
            matrix[r][c] = SEAT::from_char(ch, r, c);
        }
    }
    //println!("matrix is {:#?}", matrix);

    let mut round = 0;
    while flip2(&mut matrix) {
        round += 1;
    }
    println!("It takes {} rounds to get stable", round);

    let occupied = matrix
        .iter_mut()
        .flatten()
        .filter(|x| x.get_state() == STATE::Occupied)
        .count();
    Ok(occupied)

    //todo: use iterator fileter to get count of valid entries
}
fn question1(v: Vec<String>) -> Result<usize, &'static str> {
    let row = v.len() + 2;
    let col = v[0].len() + 2;
    let mut matrix: Vec<Vec<SEAT>> = vec![vec![SEAT::from_char('.', 0, 0); col]; row];

    // initialise matrix
    for r in 1..row - 1 {
        for c in 1..col - 1 {
            let ch = v[r - 1].chars().nth(c - 1).unwrap();
            matrix[r][c] = SEAT::from_char(ch, r, c);
        }
    }
    //println!("matrix is {:#?}", matrix);

    let mut round = 0;
    while flip(&mut matrix) {
        round += 1;
    }
    println!("It takes {} rounds to get stable", round);

    let occupied = matrix
        .iter_mut()
        .flatten()
        .filter(|x| x.get_state() == STATE::Occupied)
        .count();
    Ok(occupied)

    //todo: use iterator fileter to get count of valid entries
}
fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file()?;
    //println!("{:#?}", data);
    match question1(data.to_owned()) {
        Ok(x) => {
            println!("The result for question 1 is {}", x);
        }
        Err(x) => eprintln!("Error processing the input data: {:?}", x),
    };
    match question2(data) {
        Ok(x) => {
            println!("The sequency from position {}", x);
        }
        Err(x) => eprintln!("Error processing the input data: {:?}", x),
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_question1() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Ok(37), question1(data));
    }
    #[test]
    fn test_question2() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Ok(26), question2(data));
    }
}
