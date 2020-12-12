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
    row: i32,
    col: i32,
    neighbors: Vec<(i32, i32)>,
}

impl SEAT {
    fn from_char(c: char, row: i32, col: i32) -> Self {
        let state = match c {
            'L' => STATE::Empty,
            '.' => STATE::Floor,
            _ => STATE::Edge,
        };
        SEAT {
            state,
            row,
            col,
            neighbors: Vec::new(),
        }
    }

    fn get_state(&self) -> STATE {
        self.state
    }

    // add neighbor row/col into the neighbor array
    fn add_neighbor(&mut self, row: i32, col: i32) {
        self.neighbors.push((row, col));
    }

    // find the neightbor seat in the specified direction
    fn find_neighbor_seat(&self, m: &[Vec<SEAT>], row_step: i32, col_step: i32) -> (i32, i32) {
        match self.get_state() {
            STATE::Floor => m[(self.row + row_step) as usize][(self.col + col_step) as usize]
                .find_neighbor_seat(m, row_step, col_step),
            _ => (self.row, self.col),
        }
    }

    // find the neighbors in backwords directions
    fn set_neighbor(m: &mut Vec<Vec<SEAT>>, question: usize, row: i32, col: i32) {
        let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1)];
        for (x, y) in directions.iter() {
            let row_check = row as i32 + *x;
            let col_check = col as i32 + *y;

            if row_check < 0
                || col_check < 0
                || row_check == m.len() as i32
                || col_check == m[0].len() as i32
            {
                continue;
            }

            let (row_get, col_get) = if question == 1 {
                (row_check, col_check)
            } else {
                m[row_check as usize][col_check as usize].find_neighbor_seat(&m, *x, *y)
            };

            m[row as usize][col as usize].add_neighbor(row_get, col_get);
            m[row_get as usize][col_get as usize].add_neighbor(row, col);
        }
    }

    // get the sum of occupied neighbor seats
    fn get_neighbor_count(&mut self, m: &[Vec<SEAT>]) -> usize {
        self.neighbors
            .iter()
            .filter(|(r, c)| m[*r as usize][*c as usize].get_state() == STATE::Occupied)
            .count()
    }

    // Update a seat depends on the neighbor states, return a boolean
    // to indicate whether the state of seat has been changed.
    fn update(&mut self, m: &[Vec<SEAT>], question: usize) -> bool {
        let tolerant = question + 3;
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
        self.state != m[self.row as usize][self.col as usize].get_state()
    }
}

fn flip(mx: &mut Vec<Vec<SEAT>>, question: usize) -> bool {
    let mut unstable = false;
    let new_matrix = mx.to_owned();
    for seat in mx.iter_mut().flatten() {
        unstable |= seat.update(&new_matrix, question);
    }
    unstable
}

//todo: could try to improve the initialization precedure
fn init_matrix(v: Vec<String>, q: usize) -> Vec<Vec<SEAT>> {
    let row = v.len() + 2;
    let col = v[0].len() + 2;
    let mut matrix: Vec<Vec<SEAT>> = vec![vec![SEAT::from_char('E', 0, 0); col]; row];

    //println!("matrix is {:#?}", matrix);

    // initialise matrix
    for r in 0..row {
        for c in 0..col {
            let ch = if r > 0 && c > 0 && r < row - 1 && c < col - 1 {
                v[r - 1].chars().nth(c - 1).unwrap()
            } else {
                'E'
            };

            matrix[r][c] = SEAT::from_char(ch, r as i32, c as i32);
            if ch == '.' {
                continue;
            }

            SEAT::set_neighbor(&mut matrix, q, r as i32, c as i32);
        }
    }
    //println!("matrix after init is {:#?}", matrix);
    matrix
}

// Get number of seats been occupies while the matrix get stable
fn question(v: Vec<String>, q: usize) -> Result<usize, &'static str> {
    let mut matrix = init_matrix(v, q);
    // println!("matrix is {:#?}", matrix);

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
    //println!("raw date in is: {:#?}", data);
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
