use aoc2020::*;
use std::collections::HashMap;
use std::error::Error;

struct MaskLookUp {
    count: usize,
    mask_table: HashMap<&'static str, usize>,
}

impl MaskLookUp {
    fn new() -> Self {
        MaskLookUp {
            count: 0,
            mask_table: HashMap::new(),
        }
    }
}

fn question2(data: Vec<String>) -> Result<usize, &'static str> {
    Err("Cannot find second number.")
}

//todo: Parse the input line, and fill in the tables we need
fn parse(
    s: String,
    mask_table: &mut MaskLookUp,
    allergens: &mut HashMap<&str, u128>,
    string_masks: &mut Vec<u128>,
) {
}

fn question1(data: Vec<String>) -> Result<usize, &'static str> {
    let mut mask_table = MaskLookUp::new();
    let mut allergens = HashMap::new();
    let mut string_masks = Vec::new();

    for line in data {
        parse(line, &mut mask_table, &mut allergens, &mut string_masks);
    }

    // got the whole mask for all allergens
    let mut allergen_mask = 0;
    for v in allergens.values() {
        allergen_mask |= v;
    }

    // count the non-possible allergen ones
    let mut count = 0;
    for m in string_masks {
        count += (m & !allergen_mask).count_ones();
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
