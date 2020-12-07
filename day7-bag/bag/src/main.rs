use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct BAG {
    name: String,
    father: Vec<String>,
    child: Vec<String>,
}

impl BAG {
    // Create a new BAG object with empty father/chlid list
    fn new(s: &str) -> Self {
        BAG {
            name: s.to_string(),
            father: Vec::new(),
            child: Vec::new(),
        }
    }

    // Add a child objec to the list, also add self into
    // child's father list.
    // If child object doesn't exist, create one with empty lists
    // [in]     Child bag string name to be added
    // [in]     The whole hashmap tree for bags
    // [out]    Updated BAG objects for both self and child
    fn add_child(&mut self, s: String, tree: &mut HashMap<String, BAG>) {
        self.child.push(s.to_owned());
        let child = tree
            .entry(s.to_owned())
            .or_insert_with(|| BAG::new(s.as_str()));
        if !child.father.contains(&self.name) {
            child.father.push(self.name.to_owned());
        }
    }
}

// Parse the input string
// [in]     String for single bag info
// [out]    Name of current bag, and array of children bags

fn parse_bag(s: &str) -> (String, Vec<String>) {
    let bags: Vec<&str> = s
        .split(' ')
        .filter(|&x| !x.contains("bag") && !x.contains("contain") && !x.contains(char::is_numeric))
        .collect();
    //println!("bags are {:#?}", bags);

    let bag = bags[0..2].join(" ");

    let mut child_bags = Vec::new();
    for (pos, _color) in bags.iter().enumerate().skip(2).step_by(2) {
        child_bags.push(bags[pos..(pos + 2)].join(" "));
    }
    (bag, child_bags)
}

// Build the bag tree
// [in]     String array contains each line of the bags
// [out]    Hashmap for all bags with bag name as key,
//          and BAG struct as value
fn build_tree(s: Vec<String>) -> HashMap<String, BAG> {
    let mut tree = HashMap::new();

    for bags in s.iter() {
        println!("The bag line is {}", bags);
        let (bag, children_list) = parse_bag(bags);
        //println!("The children list for bag {} is {:#?}", bag, children_list);
        let mut new_bags = BAG::new(bag.as_str());
        for child in children_list {
            if child != "no other" {
                new_bags.add_child(child, &mut tree);
            }
        }
        tree.insert(bag, new_bags);
    }
    tree
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

//todo: Get the number of bags which can contain the specified bag
// [in]     The bag tree
// [in]     The target bag name
// [out]    Number of possible bags, or None if target bag name is invalid
fn get_containers(_s: &str, _tree: HashMap<String, BAG>) -> Option<usize> {
    Some(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file("../input.txt")?;
    //println!("{:#?}", data);
    let bag_name = "shiny gold";
    let bag_tree = build_tree(data);
    //println!("bag tree built is {:#?}", bag_tree);

    let count = get_containers(bag_name, bag_tree);
    match count {
        Some(x) => println!(
            "There are {} bags that can contain at least 1 {}",
            x, bag_name
        ),
        None => eprintln!(
            "No bag name {} found, please check your input file.",
            bag_name
        ),
    }
    Ok(())
}
