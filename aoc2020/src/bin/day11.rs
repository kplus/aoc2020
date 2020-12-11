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

    fn first_seat(
        &self,
        m: &Vec<Vec<SEAT>>,
        row_step: &i32,
        col_step: &i32,
        question: usize,
        before: bool,
    ) -> u8 {
        let row = (self.row as i32 + *row_step) as usize;
        let col = (self.col as i32 + *col_step) as usize;

        match if before {
            self.get_old_state()
        } else {
            self.get_state()
        } {
            STATE::Edge => 0,
            STATE::Occupied => 1,
            STATE::Empty => 0,
            STATE::Floor => {
                if question == 1 {
                    0
                } else {
                    m[row][col].first_seat(m, row_step, col_step, question, before)
                }
            }
        }
    }

    fn pre_update(&mut self, m: Vec<Vec<SEAT>>, question: usize) -> u8 {
        self.old_state = self.state;
        let mut adj = 0;
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        for (x, y) in directions.iter() {
            let row = (self.row as i32 + *x) as usize;
            let col = (self.col as i32 + *y) as usize;
            let mut before = false;
            if *x < 0 || (*x == 0 && *y < 0) {
                before = true
            }
            adj += m[row][col].first_seat(&m, x, y, question, before);
        }
        adj
    }
    fn update(&mut self, m: Vec<Vec<SEAT>>, question: usize) -> bool {
        let tolerant = question + 3;
        match self.state {
            STATE::Empty => {
                if self.pre_update(m, question) == 0 {
                    self.state = STATE::Occupied;
                }
            }
            STATE::Occupied => {
                if self.pre_update(m, question) >= tolerant as u8 {
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

fn flip(mx: &mut Vec<Vec<SEAT>>, question: usize) -> bool {
    let mut unstable = false;
    let row = mx.len();
    let col = mx[0].len();
    for r in 0..row {
        for c in 0..col {
            let new_matrix = mx.to_owned();
            unstable |= mx[r][c].update(new_matrix, question);
        }
    }
    //for seat in mx.iter_mut().flatten() {
    // }
    unstable
}

fn init_matrix(v: Vec<String>) -> Vec<Vec<SEAT>> {
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
    matrix
}

fn question(v: Vec<String>, q: usize) -> Result<usize, &'static str> {
    let mut matrix = init_matrix(v);
    //println!("matrix is {:#?}", matrix);

    let mut round = 0;
    while flip(&mut matrix, q) {
        round += 1;
    }
    println!("Question {}: It takes {} rounds to get stable", q, round);

    Ok(matrix
        .iter_mut()
        .flatten()
        .filter(|x| x.get_state() == STATE::Occupied)
        .count())
}
fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file()?;
    //println!("{:#?}", data);
    match question(data.to_owned(), 1) {
        Ok(x) => {
            println!("The result for question 1 is {}", x);
        }
        Err(x) => eprintln!("Error processing the input data: {:?}", x),
    };
    match question(data, 2) {
        Ok(x) => {
            println!("The result for question 2 is {}", x);
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
    fn test_question() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Ok(37), question(data.to_owned(), 1));
        assert_eq!(Ok(26), question(data, 2));
    }
}
