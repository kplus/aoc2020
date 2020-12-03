use std::fs::File;
use std::io::prelude::*;

static STEP: usize = 3;

// Get repeat pattern for a row
// The pattern is the whole row, I am overthinking the question
fn get_pattern(st: &str) -> usize {
    let mut len = 1;
    for (p, c) in st.chars().enumerate() {
        let check = st.chars().nth(p % len).unwrap();
        if check != c {
            len = p + 1;
        }
    }
    len
}

// Return and find postion in next line
fn jump(row: &str, pos: usize, len: usize) -> usize {
    //let pattern = get_pattern(&row);
    let pattern = len;
    pos - len + len % pattern
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("../input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut row_len = 0;
    let mut pos = 0;
    let mut trees = 0;
    for line in contents.lines() {
        if row_len == 0 {
            row_len = line.len();
            println!("The row is {} long.", row_len);
        };

        if pos > (row_len - 1) {
            pos = jump(&line, pos, row_len);
        }
        println!("The row is {}, and current postion is at {}", line, pos);
        println!("the charactor is {}", line.chars().nth(pos).unwrap());
        if line.chars().nth(pos).unwrap() == '#' {
            trees += 1;
        }
        pos += STEP;
    }

    println!("entountered {} trees", trees);
    Ok(())
}
