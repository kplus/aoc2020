use std::error::Error;

struct LIST {
    list: Vec<usize>,
}

impl LIST {
    fn from_str(s: &str) -> Self {
        Self {
            list: s
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect(),
        }
    }

    // try to find a cup in the current LIST
    // return index if found, or None if not
    fn find(&self, value: usize) -> Option<usize> {
        for (i, v) in self.list.iter().enumerate() {
            if *v == value {
                return Some(i);
            }
        }
        None
    }

    //get the nth cup in the current LIST
    fn get(&self, index: usize) -> usize {
        self.list[index]
    }

    // Shift list from index
    fn shift(&mut self, index: usize) {
        let mut tmp = self.list.split_off(index);
        tmp.append(&mut self.list);
        self.list = tmp;
    }

    fn get_len(&self) -> usize {
        self.list.len()
    }

    // Remove n element from right of current cup and return the slice
    fn remove(&mut self, index: usize, num: usize) -> Vec<usize> {
        let mut remove = Vec::new();
        for _i in 0..num {
            remove.push(self.list.remove(index + 1));
        }
        remove
    }

    // Insert the Vec into indicated position of LIST
    fn insert(&mut self, v: Vec<usize>, index: usize) {
        let mut i = index;
        for cup in v {
            i += 1;
            self.list.insert(i, cup);
        }
    }

    //Return the index of biggest cup in current list
    fn biggest(&self) -> usize {
        let mut biggest = 0;
        let mut index = 0;
        for (i, v) in self.list.iter().enumerate() {
            if *v > biggest {
                biggest = *v;
                index = i;
            }
        }
        index
    }

    fn get_list_string(&self) -> String {
        self.list.iter().map(|i| i.to_string()).collect::<String>()
    }
}

fn question2(data: &str) -> Result<usize, &'static str> {
    Err("Cannot find second number.")
}

fn question1(data: &str) -> Result<String, &'static str> {
    const MOVES: usize = 100;
    let mut list = LIST::from_str(data);
    let len = list.get_len();
    let mut next_cup = list.get(0);
    for _i in 0..MOVES {
        //doing: Do a move on the list
        let mut current_index = list.find(next_cup).unwrap();
        if len - current_index < 4 {
            //not enough elements to move at the tail of LIST
            list.shift(current_index);
            current_index = 0
        }

        let mut dest = next_cup - 1;
        next_cup = {
            if (len - current_index) == 4 {
                list.get(0)
            } else {
                list.get(current_index + 4)
            }
        }; //update and save next cup to deal with
        let cups_moved = list.remove(current_index, 3);

        while cups_moved.contains(&dest) {
            dest -= 1;
        }

        let dest_index = if dest == 0 {
            list.biggest()
        } else {
            list.find(dest).unwrap()
        };
        list.insert(cups_moved, dest_index);
    }
    list.shift(list.find(1).unwrap());
    //println!("final list is {}", list);
    Ok(list.get_list_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = "538914762";
    //println!("{:#?}", data);
    match question1(data) {
        Ok(x) => println!("The result for question 1 is {}", x),
        Err(x) => eprintln!("Error processing the input data: {:?}", x),
    };
    match question2(data) {
        Ok(x) => println!("The result for question 2 is {}", x),
        Err(x) => eprintln!("Error processing the input data: {:?}", x),
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"389125467";

    #[test]
    fn test_question1() {
        assert_eq!(Ok(String::from("167384529")), question1(TEST_INPUT));
    }
    #[test]
    fn test_question2() {
        assert_eq!(Err("Cannot find second number."), question2(TEST_INPUT));
    }
}
