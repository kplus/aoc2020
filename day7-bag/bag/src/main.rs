use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct BAG {
    //    name: String,
    father: Vec<String>,
    child: Vec<String>,
}

impl BAG {
    const fn new(s: &str) -> Self {
        BAG {
            //name: s.to_string(),
            father: Vec::new(),
            child: Vec::new(),
        }
    }

    fn add_child(&mut self, s: String) {
        self.child.push(s);
        //todo: add function to link father
    }
}

fn get_relation(s: &str, bt: &HashMap<String, BAG>) -> (String, Vec<String>) {
    let bags: Vec<&str> = s.split("bag").collect();
    //println!("bags are {:#?}", bags);

    let bag = "me".to_string();
    let mut v = Vec::new();
    v.push("1".to_string());
    (bag, v)
}

fn build_tree(s: Vec<String>) -> HashMap<String, BAG> {
    let mut tree = HashMap::new();

    for bags in s.iter() {
        println!("The bag line is {}", bags);
        let (bag, children) = get_relation(bags, &tree);
        let new_bags = BAG::new(bag.as_str());
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
fn get_containers(s: &str, tree: HashMap<String, BAG>) -> Option<usize> {
    Some(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file("../input.txt")?;
    //println!("{:#?}", data);
    let bag_name = "shiny gold";
    let bag_tree = build_tree(data);
    println!("bag tree built is {:#?}", bag_tree);

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
