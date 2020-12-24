use aoc2020::*;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
struct TILE {
    coordinate_x: isize,
    coordinate_z: isize,
    pre_black: bool,
    neigbhor_count: usize,
}

impl TILE {
    fn new(
        coordinate_x: isize,
        coordinate_z: isize,
        pre_black: bool,
        neigbhor_count: usize,
    ) -> Self {
        TILE {
            coordinate_x,
            coordinate_z,
            pre_black,
            neigbhor_count,
        }
    }

    fn flip_black(&self) -> bool {
        self.neigbhor_count == 2 || (self.neigbhor_count == 1 && self.pre_black)
    }
    fn previous_black(&mut self) {
        self.pre_black = true;
    }

    fn add_neighbor_count(&mut self) {
        self.neigbhor_count += 1;
    }

    fn propogate(&self, floor: &mut HashMap<(isize, isize), TILE>) {
        for x in -1..2 {
            for z in -1..2 {
                if x * z >= 0 {
                    let coordinate_x = self.coordinate_x + x;
                    let coordinate_z = self.coordinate_z + z;
                    let key = (coordinate_x, coordinate_z);
                    //6 positions of neighbor
                    if (x + z) != 0 {
                        match floor.get_mut(&key) {
                            Some(tile) => {
                                tile.add_neighbor_count();
                            }
                            None => {
                                floor.insert(key, TILE::new(coordinate_x, coordinate_z, false, 1));
                            }
                        }
                    } else {
                        // self
                        match floor.get_mut(&key) {
                            Some(cube) => {
                                cube.previous_black();
                            }
                            None => {
                                floor.insert(key, TILE::new(coordinate_x, coordinate_z, true, 0));
                            }
                        }
                    }
                }
            }
        }
    }
}

// After one day and do a flipping
// [in]     the existing lobby floor
// [out]    new lobby after flipping
fn cycle(old_lobby: HashMap<(isize, isize), TILE>) -> HashMap<(isize, isize), TILE> {
    let mut lobby: HashMap<(isize, isize), TILE> = HashMap::new();

    for tile in old_lobby.values() {
        tile.propogate(&mut lobby);
    }

    lobby.retain(|_, v| v.flip_black());

    lobby
}

fn question2(data: Vec<String>) -> Result<usize, &'static str> {
    const ROUND: usize = 100;
    //to be compatible with question 1, we generate this intermediate hashmap first
    let floor: HashMap<(isize, isize), usize> = init_floor(data);
    //println!("Init grid is {:#?}, length is {}", grid, grid.len());
    let mut lobby: HashMap<(isize, isize), TILE> = floor
        .iter()
        .map(|((x, z), flip)| {
            (
                (*x, *z),
                TILE {
                    coordinate_x: *x,
                    coordinate_z: *z,
                    pre_black: flip % 2 == 1,
                    neigbhor_count: 0,
                },
            )
        })
        .collect();
    for i in 0..ROUND {
        lobby = cycle(lobby);
    }

    Ok(lobby.len())
}

fn init_floor(data: Vec<String>) -> HashMap<(isize, isize), usize> {
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

    floor.retain(|_, flip| *flip % 2 == 1);
    floor
}

fn question1(data: Vec<String>) -> Result<usize, &'static str> {
    let floor: HashMap<(isize, isize), usize> = init_floor(data);
    Ok(floor.len())
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

        assert_eq!(Ok(2208), question2(data));
    }
}
