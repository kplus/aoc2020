use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
struct LIST {
    big: Vec<usize>,             //cache the biggest 4 cup numbers
    list: HashMap<usize, usize>, //key:value -> cup_number:next_cup
}

impl LIST {
    fn from_str(s: &str) -> Self {
        const CACHE_BIG: usize = 4;
        let mut big = vec![0; CACHE_BIG];
        let mut list = HashMap::new();
        let mut next: usize = s.chars().next().unwrap().to_digit(10).unwrap() as usize;
        for i in s.chars().map(|c| c.to_digit(10).unwrap() as usize).rev() {
            for j in 0..CACHE_BIG {
                if i > big[j] {
                    big.insert(j, i);
                    big.truncate(CACHE_BIG);
                    break;
                }
            }

            list.insert(i, next);
            next = i;
        }

        Self { big, list }
    }

    //Find the cup for next step, it should be current cup advanced by 3
    fn step(&self, current: usize, steps: usize) -> usize {
        let mut next = current;
        for _i in 0..=steps {
            next = *self.list.get(&next).unwrap();
        }
        next
    }

    fn get_move(&mut self, current: usize, num: usize) -> Vec<usize> {
        let mut to_move = Vec::new();
        let mut ptr = current;

        for _i in 0..num {
            ptr = *self.list.get(&ptr).unwrap();
            to_move.push(ptr);
        }

        self.list.insert(current, *self.list.get(&ptr).unwrap());

        to_move
    }

    // Insert the cups into indicated position of linkedlist
    fn insert(&mut self, v: Vec<usize>, dest: usize) {
        //println!("dest is {}", dest);
        let joint = *self.list.get(&dest).unwrap();
        self.list.insert(dest, v[0]);
        self.list.insert(v[v.len() - 1], joint);
    }

    // Return the index of biggest cup in current list
    fn biggest(&self, cups_to_move: &[usize]) -> usize {
        for b in &self.big {
            if !cups_to_move.contains(&b) {
                return *b;
            }
        }
        0 // this will never happen, cups to move only have 3(big_four has 4)
    }

    //Get the list string from the final LIST
    fn get_list_string(&self) -> String {
        let mut final_list = Vec::new();
        let mut next = *self.list.get(&1).unwrap();
        while next != 1 {
            final_list.push(next);
            next = *self.list.get(&next).unwrap();
        }
        final_list
            .into_iter()
            .map(|d| d.to_string())
            .collect::<String>()
    }
}

fn question(data: &str) -> Result<String, &'static str> {
    const MOVES: usize = 100;
    let mut list = LIST::from_str(data);
    //println!("initial list is {:#?}", list);
    let mut next_cup = data.chars().next().unwrap().to_digit(10).unwrap() as usize; // start point
    for i in 0..MOVES {
        let current_cup = next_cup;
        next_cup = list.step(current_cup, 3);

        let mut dest = current_cup - 1;

        let cups_to_move = list.get_move(current_cup, 3);

        while cups_to_move.contains(&dest) {
            dest -= 1;
        }

        if dest == 0 {
            dest = list.biggest(&cups_to_move);
        };

        list.insert(cups_to_move, dest);
    }
    //println!("final list is {:?}", list);
    Ok(list.get_list_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = "538914762";
    //println!("{:#?}", data);
    match question(data) {
        Ok(x) => println!("The result for question 1 is {}", x),
        Err(x) => eprintln!("Error processing the input data: {:?}", x),
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"389125467";

    #[test]
    fn test_question() {
        assert_eq!(Ok(String::from("67384529")), question(TEST_INPUT));
    }
}
