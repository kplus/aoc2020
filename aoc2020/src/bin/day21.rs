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
        Self { masks: Vec::new() }
    }

    fn max() -> Self {
        const MAX_GROUP: usize = 10; // set maximum mask vector to be 10 for now

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
}
impl BitOrAssign for Mask {
    fn bitor_assign(&mut self, rhs: Self) {
        let count = self.masks.len();
        for i in 0..rhs.masks.len() {
            if i <= count {
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
        let count = self.masks.len();
        out.copy_from_slice(&self.masks);
        for (i, mask) in rhs.masks.iter().enumerate() {
            if i <= count {
                out[i] &= mask;
            }
        }
        Self { masks: out }
    }
}
impl BitAndAssign for Mask {
    // rhs is the "right-hand side" of the expression `a &= b`
    fn bitand_assign(&mut self, rhs: Self) {
        let count = self.masks.len();
        for i in 0..rhs.masks.len() {
            if i <= count {
                self.masks[i] &= rhs.masks[i];
            }
        }
    }
}
/*
impl BitOr for Mask {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a | b`
    fn bitor(self, rhs: Self) -> Self::Output {
        let mut out = Vec::new();
        let count = self.masks.len();
        out.copy_from_slice(&self.masks);
        for i in 0..rhs.masks.len() {
            if i <= count {
                out[i] |= rhs.masks[i];
            } else {
                out.push(rhs.masks[i]);
            }
        }
        Self { masks: out }
    }
}
*/
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
        if self.count == 1280 {
            panic!("Item count exceeds 1280, consider to increase MAX const in Mask");
        };
    }

    fn get_mask(&self, s: &str) -> Mask {
        let index = self.mask_table.get(s).unwrap();
        let group = index / 128;
        let offset = index % 128;
        let mut out = Vec::new();
        for _i in 0..group {
            out.push(0);
        }
        out.push(1 << offset);
        Mask { masks: out }
    }
}

fn question2(data: Vec<String>) -> Result<usize, &'static str> {
    Err("Cannot find second number.")
}

// Parse the input line, and fill in the tables we need
fn parse(
    s: String,
    mask_table: &mut MaskLookUp,
    allergens: &mut HashMap<String, Mask>,
    string_masks: &mut Vec<Mask>,
) {
    println!("String to parse is {}", s);

    let tmp_s: Vec<&str> = s.split_terminator("(contains").map(|s| s.trim()).collect();
    let mut string_mask = Mask::new();

    // go through the ingredients for this food, update mask lookup table if needed
    // and cache the mask of current string
    for ingredient in tmp_s[0].split_whitespace() {
        println!("ingredient {}", ingredient);
        if !mask_table.has(ingredient) {
            println!("doesn't exist, so insert it");
            mask_table.insert(ingredient);
        }
        string_mask |= mask_table.get_mask(ingredient);
    }
    println!("got string mask as {:#?}", string_mask);
    string_masks.push(string_mask.to_owned());

    // go through the allergens in this food, update allergens table
    for allergen in tmp_s[1]
        .split_terminator(|c: char| c.is_ascii_punctuation())
        .map(|s| s.trim())
    {
        let mask = allergens
            .entry(allergen.to_owned())
            .or_insert_with(Mask::max);
        *mask &= string_mask.to_owned();
    }
}

fn question1(data: Vec<String>) -> Result<usize, &'static str> {
    let mut mask_table = MaskLookUp::new();
    let mut allergens = HashMap::new();
    let mut string_masks = Vec::new();

    for line in data {
        parse(line, &mut mask_table, &mut allergens, &mut string_masks);
    }

    // got the whole mask for all allergens
    let mut allergen_mask = Mask::new();
    for v in allergens.values() {
        allergen_mask |= v.to_owned();
    }

    println!(
        "the mask table is {:#?}, allergen table is {:#?}, and string masks is {:#?}",
        mask_table, allergens, string_masks
    );

    // count the non-possible allergen ones
    let mut count = 0;
    for m in string_masks {
        count += (m & !allergen_mask.to_owned()).count_ones();
    }
    Ok(count as usize)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file()?;
    println!("{:#?}", data);
    match question1(data.to_owned()) {
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

        assert_eq!(Err("Cannot find second number."), question2(data));
    }
}
