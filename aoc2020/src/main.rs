use std::error::Error;
use std::fs;
use std::path::Path;

use std::cmp::Ordering;

struct Window {
    head: usize,
    tail: usize,
    sum: usize,
}
impl Window {
    fn new(a: &[usize]) -> Self {
        Window {
            head: 1,
            tail: 0,
            sum: a[0] + a[1],
        }
    }

    fn move_head(&mut self, whole_array: &[usize]) -> usize {
        self.head += 1;
        self.sum += whole_array[self.head];
        self.sum
    }

    fn move_tail(&mut self, whole_array: &[usize]) -> usize {
        self.sum -= whole_array[self.tail];
        self.tail += 1;
        self.sum
    }
}
fn question2(target: usize, whole_array: &[usize]) -> Result<(usize, usize), &'static str> {
    println!("target is {}, whole array is {:#?}", target, whole_array);
    let mut s = 0;
    let mut w = Window::new(whole_array);
    while w.tail != whole_array[whole_array.len() - 1] {
        match s.cmp(&target) {
            Ordering::Equal => return Ok((w.head, w.tail)),
            Ordering::Less => s = w.move_head(whole_array),
            Ordering::Greater => s = w.move_tail(whole_array),
        };
    }
    Err("Cannot find the window to fit target number.")
}

fn question1(whole_array: &[usize]) -> Result<usize, &'static str> {
    const PREAMBLE: usize = 25;
    let check = &whole_array[PREAMBLE..];
    'next_num: for (i, &num) in check.iter().enumerate() {
        let preamble = &whole_array[i..i + PREAMBLE];
        //println!("Checking {} with preamble {:#?}", num, preamble);
        for (j, first) in preamble.iter().enumerate() {
            for second in &preamble[j + 1..] {
                if (first + second) == num {
                    //println!(
                    //    "The number been checked is {} at postion {}, it can be made up by {} + {}",
                    //    num, i, first, second
                    //);
                    continue 'next_num;
                }
            }
        }
        //println!("The first number is found: {}", num);
        return Ok(num);
    }
    Err("Cannot find first number.")
}

// [in]     Path of file to read details from
// [out     Arrary of String for each lines
fn load_file<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Box<dyn Error>> {
    let input = fs::read_to_string(path)?;
    //println!("read in content:\n{}", input);
    let mut out = Vec::new();
    for line in input.lines() {
        //println!("read in peron details:\n{}", person);
        out.push(line.to_string());
    }
    Ok(out)
}
fn add_max_min(a: &[usize]) -> usize {
    let mut max = a[0];
    let mut min = a[0];
    for &i in a {
        if i > max {
            max = i;
        }
        if i < min {
            min = i;
        }
    }
    max + min
}
// Question 1 uses find_end, and question 2 uses find_bug
fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file("input.txt")?;
    //println!("{:#?}", data);

    let whole_array: Vec<usize> = data.iter().map(|s| s.parse::<usize>().unwrap()).collect();
    //println!("The whole array is {:#?}", whole_array);
    let mut target = 0;
    match question1(&whole_array) {
        Ok(x) => {
            println!("The result is {}", x);
            target = x;
        }
        Err(x) => println!("Error processing the input data: {:?}", x),
    };

    match question2(target, &whole_array) {
        Ok((x, y)) => {
            println!("The sequency from position {} to {}", y, x);
            let range = &whole_array[y..x];
            println!("The result is {}", add_max_min(range));
        }
        Err(x) => println!("Error processing the input data: {:?}", x),
    };
    Ok(())
}
