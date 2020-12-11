use std::error::Error;

use aoc2020::*;

#[derive(Clone, Copy, PartialEq, Debug)]
enum STATE {
    Occupied,
    Empty,
    Floor,
    Edge,
}

#[derive(Clone, Debug)]
struct SEAT {
    state: STATE,
    old_state: STATE,
    row: usize,
    col: usize,
    neighbors: Vec<(usize, usize)>,
}

impl SEAT {
    fn from_char(c: char) -> Self {
        let state = match c {
            'L' => STATE::Empty,
            '.' => STATE::Floor,
            _ => STATE::Edge,
        };
        SEAT {
            state,
            old_state: state,
            row: 0,
            col: 0,
            neighbors: Vec::new(),
        }
    }

    fn get_state(&self) -> STATE {
        self.state
    }
    fn get_old_state(&self) -> STATE {
        self.old_state
    }

    fn add_neighbor(&mut self, row: usize, col: usize) {
        self.neighbors.push((row, col));
    }

    fn first_seat(
        &self,
        m: &Vec<Vec<SEAT>>,
        row_step: &i32,
        col_step: &i32,
        question: usize,
    ) -> (usize, usize) {
        match self.get_state() {
            STATE::Edge => (self.row, self.col),
            STATE::Occupied => (self.row, self.col),
            STATE::Empty => (self.row, self.col),
            STATE::Floor => {
                if question == 1 {
                    (self.row, self.col)
                } else {
                    let row_check = (self.row as i32 + *row_step) as usize;
                    let col_check = (self.col as i32 + *col_step) as usize;
                    //                    println!(
                    //                      "row is {}, col is {}, row_check is {}, col_check is {}, self is {:#?}",
                    //                    self.row, self.col, row_check, col_check, self
                    //              );
                    m[row_check][col_check].first_seat(m, row_step, col_step, question)
                }
            }
        }
    }

    fn set_neighbor(m: &mut Vec<Vec<SEAT>>, question: usize, row: usize, col: usize) {
        let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1)];
        for (x, y) in directions.iter() {
            let row_check = (row as i32 + *x) as usize;
            let col_check = (col as i32 + *y) as usize;

            if row_check == m.len() || col_check == m[0].len() {
                continue;
            }

            //       println!(
            //         "row is {}, col is {}, row_check is {}, col_check is {}",
            //       row, col, row_check, col_check
            // );
            let (row_get, col_get) = m[row_check][col_check].first_seat(&m, x, y, question);
            m[row][col].neighbors.push((row_get, col_get));
            m[row_get][col_get].add_neighbor(row, col);
        }
    }

    fn get_neighbor_count(&mut self, m: Vec<Vec<SEAT>>) -> usize {
        self.neighbors
            .iter()
            .filter(|(r, c)| {
                if *r < self.row || (*r == self.row && *c < self.col) {
                    m[*r][*c].get_old_state() == STATE::Occupied
                } else {
                    m[*r][*c].get_state() == STATE::Occupied
                }
            })
            .count()
    }

    // Update a seat depends on the neighbor states, return a boolean
    // to indicate whether the state of seat has been changed.
    fn update(&mut self, m: Vec<Vec<SEAT>>, question: usize) -> bool {
        let tolerant = question + 3;
        self.old_state = self.state;
        match self.state {
            STATE::Empty => {
                if self.get_neighbor_count(m) == 0 {
                    self.state = STATE::Occupied;
                }
            }
            STATE::Occupied => {
                if self.get_neighbor_count(m) >= tolerant {
                    self.state = STATE::Empty;
                }
            }
            _ => {}
        }
        self.state != self.old_state
    }
}

// todo: can I use iterator instead of double loop?
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

//todo: could try to improve the initialization precedure
fn init_matrix(v: Vec<String>, q: usize) -> Vec<Vec<SEAT>> {
    let row = v.len() + 2;
    let col = v[0].len() + 2;
    let mut matrix: Vec<Vec<SEAT>> = vec![vec![SEAT::from_char('E'); col]; row];

    println!("matrix is {:#?}", matrix);
    // initialise matrix
    for r in 1..row {
        for c in 1..col {
            if r < row - 1 && c < col - 1 {
                let ch = v[r - 1].chars().nth(c - 1).unwrap();
                matrix[r][c] = SEAT::from_char(ch);
            }
            SEAT::set_neighbor(&mut matrix, q, r, c);
        }
    }
    println!("matrix is {:#?}", matrix);
    matrix
}

// Get number of seats been occupies while the matrix get stable
fn question(v: Vec<String>, q: usize) -> Result<usize, &'static str> {
    let mut matrix = init_matrix(v, q);
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
