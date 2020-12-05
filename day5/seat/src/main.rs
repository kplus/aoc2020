use std::error::Error;
use std::fs;
use std::path::Path;

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

// [in]     Path of file to read boarding passes from
// [out     Arrary of String, each String is a boarding ID
fn load_file<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Box<dyn Error>> {
    let input = fs::read_to_string(path)?;
    //    println!("read in content:\n{}", input);

    let mut v: Vec<String> = Vec::new();

    for line in input.lines() {
        v.push(line.to_string());
    }

    Ok(v)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file("../input.txt")?;
    //println!("{:#?}", data);

    const MAX: usize = 0x7F * 8 + 8;
    println!("max list length is {}", MAX);
    let mut whole = vec![State::Unset; MAX];
    for pass in data.iter() {
        let (row, col) = get_seat(pass);
        let seat_id = row * 8 + col;
        println!("seat id is {}", seat_id);

        // update list function

        whole[seat_id] = State::Occupied;
        if whole[seat_id - 1] == State::Unset {
            whole[seat_id - 1] = State::Possible;
        }
        if whole[seat_id + 1] == State::Unset {
            whole[seat_id + 1] = State::Possible;
        }
    }

    for (i, s) in whole.iter().enumerate() {
        if *s == State::Possible {
            println!("Find a possible postion {}", i);
        }
    }
    Ok(())
}
