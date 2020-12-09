use std::error::Error;

use aoc2020::*;

#[derive(Copy, Clone, PartialEq)]
enum State {
    Unset,    // unset state for a seat
    Occupied, // occupied by other person
    Possible, // possible seat for myself
}

fn get_num(s: &str) -> usize {
    let mut out = 0;
    match s.len() {
        7 => {
            for c in s.chars() {
                //println!("c is {}, out is {}", c, out);
                out <<= 1;
                match c {
                    'B' => out += 1,
                    'F' => out += 0,
                    _ => panic!("Invalid input row"),
                }
            }
        }
        3 => {
            for c in s.chars() {
                out <<= 1;
                match c {
                    'R' => out += 1,
                    'L' => out += 0,
                    _ => panic!("Invalid input col"),
                }
            }
        }
        _ => panic!("Invalid input length"),
    }
    //println!("number got here is {}", out);
    out
}

fn get_seat(seat: &str) -> (usize, usize) {
    let (row_str, col_str) = seat.split_at(7);
    //println!("Row string is {}, col string is {}", row_str, col_str);
    (get_num(row_str), get_num(col_str))
}

// When a new seat ID is found from list, this position is
// occupied by somebody else. It also makes the 2 seats next
// to it possible to be mine
fn update_list(whole: &mut Vec<State>, id: usize) {
    whole[id] = State::Occupied;
    if whole[id - 1] == State::Unset {
        whole[id - 1] = State::Possible;
    }
    if whole[id + 1] == State::Unset {
        whole[id + 1] = State::Possible;
    }
}

fn finalize_list(whole: Vec<State>) {
    let mut valid: Vec<usize> = Vec::new();

    for (i, s) in whole.iter().enumerate() {
        if *s == State::Possible {
            //println!("Find a possible postion {}", i);
            valid.push(i);
        }
    }
    let l = valid.len();
    println!("Found {} possible values.", l);
    if l > 0 {
        println!("The middle value is {}", valid[l / 2]);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file()?;
    //println!("{:#?}", data);

    const MAX: usize = 0x7F * 8 + 8;
    let mut whole = vec![State::Unset; MAX];
    for pass in data.iter() {
        let (row, col) = get_seat(pass);
        let seat_id = row * 8 + col;
        //println!("seat id is {}", seat_id);
        update_list(&mut whole, seat_id);
    }

    finalize_list(whole);
    Ok(())
}
