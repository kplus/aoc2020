use std::collections::HashMap;
use std::error::Error;

// f(n)=f(n-1) * e(n-1) + f(n-2) * e(n-2) + f(n-3) * e(n-3)
// where e(n) means the existance of n in the array.
fn find_route(s: &[usize], i: usize, cache: &mut HashMap<usize, usize>) -> usize {
    //println!("Checking route for {}", i);
    if !s.contains(&i) {
        //println!("Skipping due to {} not in array", i);
        return 0;
    }
    if cache.contains_key(&i) {
        return cache.get(&i).unwrap().to_owned();
    }
    let route = match i {
        0 => 0,
        1 => 1,
        2 => find_route(&s, 1, cache) + 1,
        3 => find_route(&s, 1, cache) + find_route(&s, 2, cache) + 1,
        _ => {
            find_route(&s, i - 1, cache)
                + find_route(&s, i - 2, cache)
                + find_route(&s, i - 3, cache)
        }
    };
    cache.insert(i, route);
    //println!("route for {} is {}", i, route);
    route
}

fn question2(s: Vec<usize>) -> Result<usize, &'static str> {
    //println!("array to check is {:#?}", s);
    let mut cache: HashMap<usize, usize> = HashMap::new();
    Ok(find_route(&s, s[s.len() - 1], &mut cache))
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

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"28
        33
        18
        42
        31
        14
        46
        20
        48
        47
        24
        23
        49
        45
        19
        38
        39
        11
        1
        32
        25
        35
        8
        17
        7
        9
        4
        2
        34
        10
        3";

    #[test]
    fn test_question1() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        let mut data: Vec<usize> = data.iter().map(|s| s.parse::<usize>().unwrap()).collect();
        data.sort();
        assert_eq!(Ok([22, 0, 10]), question1(data));
    }
    #[test]
    fn test_question2() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        let mut data: Vec<usize> = data.iter().map(|s| s.parse::<usize>().unwrap()).collect();
        data.sort();
        assert_eq!(Ok(19208), question2(data));
    }
}
