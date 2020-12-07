use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

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
    let bag = "me".to_string();
    let mut v = Vec::new();
    v.push("1".to_string());

    println!("bags are {:#?}", bags);

    (bag, v)
}

fn build_tree(s: Vec<String>) -> HashMap<String, BAG> {
    let mut bag_tree = HashMap::new();

    for bags in s.iter() {
        let (bag, children) = get_relation(bags, &bag_tree);
        let new_bags = BAG::new(bag.as_str());
        bag_tree.insert(bag, new_bags);
    }
    bag_tree
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

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file("../input.txt")?;
    //println!("{:#?}", data);

    build_tree(data);
    Ok(())
}
