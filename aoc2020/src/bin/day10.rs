use std::error::Error;

// f(n)=f(n-1) * e(n-1) + f(n-2) * e(n-2) + f(n-3) * e(n-3)
// where e(n) means the existance of n in the array.
fn find_route(s: &[usize], i: usize) -> usize {
    if !s.contains(&i) {
        return 0;
    }
    let route = match i {
        0 => 1,
        1 => s.contains(&1) as usize,
        2 => find_route(&s, 1) + 1,
        _ => find_route(&s, i - 1) + find_route(&s, i - 2) + find_route(&s, i - 3),
    };
    println!("route for {} is {}", i, route);
    route
}

fn question2(s: Vec<usize>) -> Result<usize, &'static str> {
    Ok(find_route(&s, s[s.len() - 1]))
}

fn question1(s: Vec<usize>) -> Result<[usize; 3], &'static str> {
    let mut jolts: [usize; 3] = [0; 3];

    //println!("sorted queue is {:#?}", s);
    jolts[s[0] - 1] += 1;
    for i in 1..s.len() {
        jolts[s[i] - s[i - 1] - 1] += 1;
    }
    jolts[2] += 1;
    Ok(jolts)
}
use aoc2020::*;
fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file()?;
    //println!("{:#?}", data);
    let mut data: Vec<usize> = data.iter().map(|s| s.parse::<usize>().unwrap()).collect();
    data.sort();
    match question1(data.to_owned()) {
        Ok(jolts) => {
            println!("There are {} 1-jolt difference, {} 2-jolts differences and {} 3-jolts differences, and the result of question 1 is {}",jolts[0], jolts[1], jolts[2], jolts[0]*jolts[2]);
        }
        Err(x) => println!("Error processing the input data: {:?}", x),
    };
    match question2(data) {
        Ok(routes) => {
            println!("There are {} routes", routes);
        }
        Err(x) => println!("Error processing the input data: {:?}", x),
    };
    Ok(())
}
