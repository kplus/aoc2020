use aoc2020::*;
use std::collections::HashMap;
use std::error::Error;
use std::ops::*;

#[derive(Debug, Clone)]
struct Mask {
    masks: Vec<u128>,
}
impl Mask {
    fn new() -> Self {
        let mut masks = Vec::new();
        masks.push(0);
        Self { masks }
    }

    fn max() -> Self {
        const MAX_GROUP: usize = 2; // set maximum mask vector to be 2 for now

        Self {
            masks: vec![u128::MAX; MAX_GROUP],
        }
    }
    fn count_ones(&self) -> u32 {
        let mut ones = 0;
        for mask in &self.masks {
            ones += mask.count_ones();
        }
        ones
    }

    fn get_index(&self) -> u32 {
        let mut index = 0;
        for mask in &self.masks {
            if *mask == 0 {
                index += 128;
            } else {
                index += mask.trailing_zeros();
                break;
            }
        }
        index
    }

    fn is_not_empty(&self) -> bool {
        self.masks.len() != 1 || self.masks[0] != 0
    }
}
impl BitOrAssign for Mask {
    fn bitor_assign(&mut self, rhs: Self) {
        let count = self.masks.len();
        for i in 0..rhs.masks.len() {
            if i < count {
                self.masks[i] |= rhs.masks[i];
            } else {
                self.masks.push(rhs.masks[i]);
            }
        }
    }
}
impl Not for Mask {
    type Output = Self;

    fn not(self) -> Self::Output {
        let mut out = Vec::new();
        for mask in self.masks {
            out.push(!mask);
        }
        Self { masks: out }
    }
}
impl BitAnd for Mask {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a & b`
    fn bitand(self, rhs: Self) -> Self::Output {
        let mut out = Vec::new();
        let count = rhs.masks.len();
        for (i, mask) in self.masks.iter().enumerate() {
            if i < count {
                out.push(mask & rhs.masks[i]);
            }
        }
        Self { masks: out }
    }
}
impl BitAndAssign for Mask {
    // rhs is the "right-hand side" of the expression `a &= b`
    fn bitand_assign(&mut self, rhs: Self) {
        let count = rhs.masks.len();
        for i in 0..self.masks.len() {
            if i < count {
                self.masks[i] &= rhs.masks[i];
            } else {
                self.masks[i] = 0;
            }
        }
    }
}

#[derive(Debug)]
struct MaskLookUp {
    count: usize,
    mask_table: HashMap<String, usize>,
}

impl MaskLookUp {
    fn new() -> Self {
        MaskLookUp {
            count: 0,
            mask_table: HashMap::new(),
        }
    }

    fn has(&self, s: &str) -> bool {
        self.mask_table.contains_key(s)
    }

    fn insert(&mut self, s: &str) {
        self.mask_table.insert(s.to_owned(), self.count);
        self.count += 1;
        if self.count == 256 {
            panic!("Item count exceeds 256, consider to increase MAX const in Mask");
        };
    }

    fn get_mask(&self, s: &str) -> Mask {
        let index = self.mask_table.get(s).unwrap();
        let mut out = Vec::new();
        for _i in 0..index / 128 {
            out.push(0);
        }
        out.push(1 << (index % 128));
        Mask { masks: out }
    }

    fn get_name(&self, mask: &Mask) -> String {
        let i = mask.get_index();
        for (name, index) in self.mask_table.iter() {
            if *index == i as usize {
                return name.to_owned();
            }
        }
        String::from("None")
    }
}

fn map_allergens(
    mask_table: &MaskLookUp,
    allergens: &mut HashMap<String, Mask>,
    map: &mut HashMap<String, String>,
) {
    let mut mask_to_clear = Mask::new();
    let mut name_to_clear = String::from("");
    loop {
        if mask_to_clear.is_not_empty() {
            allergens.remove(&name_to_clear);
            for v in allergens.values_mut() {
                *v &= !mask_to_clear.to_owned();
            }
            mask_to_clear = Mask::new();
        }
        for (name, mask) in allergens.iter() {
            if mask.count_ones() == 1 {
                let allergen = name.to_owned();
                let ingredient = mask_table.get_name(mask);
                map.insert(allergen, ingredient);
                mask_to_clear = mask.to_owned();
                name_to_clear = name.to_owned();
                break;
            }
        }
        if allergens.is_empty() {
            break;
        }
    }
}

fn question2(data: Vec<String>) -> Result<Vec<String>, &'static str> {
    let mut mask_table = MaskLookUp::new();
    let mut allergens = HashMap::new();
    let mut string_masks = Vec::new();

    for line in data {
        parse(line, &mut mask_table, &mut allergens, &mut string_masks);
    }

    //println!(
    //    "mask lookup table is {:#?}, allergens tabls is {:#?}",
    //    mask_table, allergens
    //);

    // store allergen to ingredient mapping
    let mut map = HashMap::new();
    map_allergens(&mask_table, &mut allergens, &mut map);
    //println!("final map is {:#?}", map);

    let mut order: Vec<&String> = map.keys().collect();
    order.sort();
    //println!("sorted is {:#?}", order);

    let mut ordered_ingredient = Vec::new();
    for allergen in order.into_iter() {
        ordered_ingredient.push(map.get(allergen).unwrap().to_owned());
    }

    Ok(ordered_ingredient)
}

// Parse the input line, and fill in the tables we need
fn parse(
    s: String,
    mask_table: &mut MaskLookUp,
    allergens: &mut HashMap<String, Mask>,
    string_masks: &mut Vec<Mask>,
) {
    //println!("String to parse is {}", s);

    let tmp_s: Vec<&str> = s.split_terminator("(contains").map(|s| s.trim()).collect();
    let mut string_mask = Mask::new();

    // go through the ingredients for this food, update mask lookup table if needed
    // and cache the mask of current string
    for ingredient in tmp_s[0].split_whitespace() {
        if !mask_table.has(ingredient) {
            mask_table.insert(ingredient);
        }
        string_mask |= mask_table.get_mask(ingredient);
    }

    // go through the allergens in this food, update allergens table
    // for a single allergen, it can only exists in the common ingredients
    // of different foods, so logic add is needed here
    for allergen in tmp_s[1]
        .split_terminator(|c: char| c.is_ascii_punctuation())
        .map(|s| s.trim())
    {
        let mask = allergens
            .entry(allergen.to_owned())
            .or_insert_with(Mask::max);
        *mask &= string_mask.to_owned();
    }
    string_masks.push(string_mask);
    //println!("string mask is {:#?}", string_mask);
}

fn question1(data: Vec<String>) -> Result<usize, &'static str> {
    let mut mask_table = MaskLookUp::new();
    let mut allergens = HashMap::new();
    let mut string_masks = Vec::new();

    for line in data {
        parse(line, &mut mask_table, &mut allergens, &mut string_masks);
    }

    //println!(
    //    "mask lookup table is {:#?}, allergens tabls is {:#?}",
    //    mask_table, allergens
    //);

    // got the whole mask for all allergens
    let mut allergen_mask = Mask::new();
    for v in allergens.values() {
        allergen_mask |= v.to_owned();
    }

    // count the non-possible allergen ones
    let inverse = !allergen_mask;
    let mut count = 0;
    for m in string_masks {
        count += (m & inverse.to_owned()).count_ones();
    }
    Ok(count as usize)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file()?;
    match question1(data.to_owned()) {
        Ok(x) => println!("The result for question 1 is {}", x),
        Err(x) => eprintln!("Error processing the input data: {:?}", x),
    };
    //println!("{:#?}", data);
    match question2(data) {
        Ok(x) => println!("The result for question 2 is {:?}", x.join(",")),
        Err(x) => eprintln!("Error processing the input data: {:?}", x),
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
    trh fvjkl sbzzf mxmxvkd (contains dairy)
    sqjhc fvjkl (contains soy)
    sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn test_question1() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Ok(5), question1(data));
    }
    #[test]
    fn test_question2() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(
            Ok(vec![
                String::from("mxmxvkd"),
                String::from("sqjhc"),
                String::from("fvjkl")
            ]),
            question2(data)
        );
    }
}
