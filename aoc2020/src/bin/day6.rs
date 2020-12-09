use std::error::Error;

use aoc2020::*;

fn to_mask(s: &str) -> u32 {
    let mut mask = 0;
    let bytes = s.as_bytes();

    for i in bytes.iter() {
        mask |= 1 << (i - 97);
    }
    mask
}

// count how many yes has been answered by at least one of the person
// within a group
fn count_yes(s: String) -> usize {
    let mut list: Vec<char> = s.split_whitespace().collect::<String>().chars().collect();
    list.sort();
    list.dedup();
    //println!("sorted list is {:#?}", list);

    list.len()
}

// count how many yes has been answered by all the person within a group
fn count_all(s: String) -> u32 {
    //println!("The input is {}", s);
    let mut mask: u32 = 0xFFFFFFFF;
    for line in s.lines() {
        mask &= to_mask(line);
    }
    //println!("final mask is {:b}", mask);
    mask.count_ones()
}
// For question 1, call count_yes in the loop
// For question 2, call count_all in the loop
fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file()?;
    let mut count = 0;

    //println!("{:#?}", data);

    for group in data {
        //println!("this group is {}", group);
        //let y = count_yes(group);
        //        println!("There are {} yes in this group", y);
        let y = count_all(group);
        count += y;
    }
    println!("The result is {}", count);
    Ok(())
}
