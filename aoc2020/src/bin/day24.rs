use aoc2020::*;
use std::collections::HashMap;
use std::error::Error;

/*
struct TILE {
    coordinarate: (isize, isize, isize),
    flip: usize
}

impl TILE {
    //todo: Get a instruction of tile from
    fn from_str(s:String) -> Self {

    }
}
*/
fn question2(data: Vec<String>) -> Result<usize, &'static str> {
    Err("Cannot find second number.")
}

fn question1(data: Vec<String>) -> Result<usize, &'static str> {
    let mut floor: HashMap<(isize, isize), usize> = HashMap::new();
    for s in data {
        // One line per tile
        let mut x = 0; // x-axis represents E/W directionS
        let mut y = 0; // y-axis represents NE/SW directionS
        let mut z = 0; // z-axis represents NW/SE directionS
        let mut cache_c = ' ';
        for c in s.chars() {
            match c {
                'n' => cache_c = 'n',
                's' => cache_c = 's',
                'e' => {
                    match cache_c {
                        'n' => y += 1,
                        's' => z -= 1,
                        _ => x += 1,
                    };
                    cache_c = ' ';
                }
                'w' => {
                    match cache_c {
                        'n' => z += 1,
                        's' => y -= 1,
                        _ => x -= 1,
                    };
                    cache_c = ' ';
                }
                _ => {}
            }
        }

        // as in hexagonal, different routes can ends in same position
        // we need to unify it. x+z=y in our setup
        let flip = floor.entry((x + y, y + z)).or_insert(0);
        *flip += 1;
    }

    //println!("final floor is {:#?}", floor);
    Ok(floor.into_iter().filter(|(_, flip)| flip % 2 == 1).count())
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file()?;
    //println!("{:#?}", data);
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

    static TEST_INPUT: &str = r"sesenwnenenewseeswwswswwnenewsewsw
    neeenesenwnwwswnenewnwwsewnenwseswesw
    seswneswswsenwwnwse
    nwnwneseeswswnenewneswwnewseswneseene
    swweswneswnenwsewnwneneseenw
    eesenwseswswnenwswnwnwsewwnwsene
    sewnenenenesenwsewnenwwwse
    wenwwweseeeweswwwnwwe
    wsweesenenewnwwnwsenewsenwwsesesenwne
    neeswseenwwswnwswswnw
    nenwswwsewswnenenewsenwsenwnesesenew
    enewnwewneswsewnwswenweswnenwsenwsw
    sweneswneswneneenwnewenewwneswswnese
    swwesenesewenwneswnwwneseswwne
    enesenwswwswneneswsenwnewswseenwsese
    wnwnesenesenenwwnenwsewesewsesesew
    nenewswnwewswnenesenwnesewesw
    eneswnwswnwsenenwnwnwwseeswneewsenese
    neswnwewnwnwseenwseesewsenwsweewe
    wseweeenwnesenwwwswnew";

    #[test]
    fn test_question1() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Ok(10), question1(data));
    }
    #[test]
    fn test_question2() {
        let data: Vec<String> = TEST_INPUT.lines().map(|s| s.trim().to_owned()).collect();

        assert_eq!(Err("Cannot find second number."), question2(data));
    }
}
