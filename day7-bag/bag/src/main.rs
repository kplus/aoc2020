use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
struct BAG {
    name: String,
    father: Vec<String>,
    child: Vec<(usize, String)>,
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
    fn add_child(&mut self, v: (usize, String), tree: &mut HashMap<String, BAG>) {
        self.child.push(v.to_owned());
        let child = tree
            .entry(v.1.to_owned())
            .or_insert_with(|| BAG::new(v.1.as_str()));
        if !child.father.contains(&self.name) {
            child.father.push(self.name.to_owned());
        }
    }
}

// Parse the input string
// [in]     String for single bag info
// [out]    Name of current bag, and array of children bags

fn parse_bag(s: &str) -> (String, Vec<(usize, String)>) {
    let bags: Vec<&str> = s
        .split(' ')
        .filter(|&x| !x.contains("bag") && !x.contains("contain"))
        .collect();
    //println!("bags are {:#?}", bags);

    let bag = bags[0..2].join(" ");

    let mut child_bags = Vec::new();
    for (pos, num) in bags.iter().enumerate().skip(2).step_by(3) {
        match num.parse::<usize>() {
            Ok(x) => child_bags.push((x, bags[(pos + 1)..(pos + 3)].join(" "))),
            _ => child_bags.push((0, bags[pos..(pos + 2)].join(" "))),
        }
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
        //println!("The bag line is {}", bags);
        let (bag, children_list) = parse_bag(bags);

        //println!("The children list for bag {} is {:#?}", bag, children_list);

        // to_owned is esstential, otherwise tree will be
        // mutbale borrowed twice, which is not allowed
        let mut new_bags = tree
            .entry(bag.to_owned())
            .or_insert_with(|| BAG::new(bag.as_str()))
            .to_owned();

        for child in children_list {
            if child.1 != "no other" {
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

// Recursion function to get father and upper layers
fn count_father_iter(s: String, tree: &HashMap<String, BAG>, v: &mut Vec<String>) {
    for father in tree.get(&s).unwrap().father.to_owned() {
        v.push(father.to_owned());
        count_father_iter(father, &tree, v);
    }
}
//todo: Get the number of bags which can contain the specified bag
// [in]     The target bag name
// [in]     The bag tree
// [out]    Number of possible bags
fn get_containers(s: &str, tree: HashMap<String, BAG>) -> usize {
    let mut v: Vec<String> = Vec::new();
    let s = s.to_string();
    count_father_iter(s, &tree, &mut v);
    v.sort();
    v.dedup();
    v.len()
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file("../input.txt")?;
    //println!("{:#?}", data);
    let bag_name = "shiny gold";
    let bag_tree = build_tree(data);
    //println!("bag tree built is {:#?}", bag_tree);

    let count = get_containers(bag_name, bag_tree);
    match count {
        0 => eprintln!(
            "No bag name {} found, please check your input file.",
            bag_name
        ),
        x => println!(
            "There are {} bags that can contain at least 1 {}",
            x, bag_name
        ),
    }
    Ok(())
}
