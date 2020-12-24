use std::collections::hash_map::{HashMap};

#[test]
fn test_run_24() {
    let inputs = vec![
        ("sesenwnenenewseeswwswswwnenewsewsw\nneeenesenwnwwswnenewnwwsewnenwseswesw\nseswneswswsenwwnwse\nnwnwneseeswswnenewneswwnewseswneseene\nswweswneswnenwsewnwneneseenw\neesenwseswswnenwswnwnwsewwnwsene\nsewnenenenesenwsewnenwwwse\nwenwwweseeeweswwwnwwe\nwsweesenenewnwwnwsenewsenwwsesesenwne\nneeswseenwwswnwswswnw\nnenwswwsewswnenenewsenwsenwnesesenew\nenewnwewneswsewnwswenweswnenwsenwsw\nsweneswneswneneenwnewenewwneswswnese\nswwesenesewenwneswnwwneseswwne\nenesenwswwswneneswsenwnewswseenwsese\nwnwnesenesenenwwnenwsewesewsesesew\nnenewswnwewswnenesenwnesewesw\neneswnwswnwsenenwnwnwwseeswneewsenese\nneswnwewnwnwseenwseesewsenwsweewe\nwseweeenwnesenwwwswnew", (10, 2208)),       
    ];
    for i in inputs {
        let res = run(i.0.to_owned());
        match res {
            Ok(o) => assert_eq!(o, i.1),
            Err(e) => panic!(e),
        }
    }  
}

pub fn run(input: String) -> Result<(i64, i64), &'static str> {
    let lines = input.split("\n");
    let mut tiles:HashMap<(i32, i32), (bool, usize)> = HashMap::new();

    // Parse the input, extract pairs if the character is a N or S, otherwise it's a plain E/W,
    // Convert to cartisian coordinates to remove duplicates. 
    // To avoid floats, double all distances. Treat each diagonal movement as 1 in each dir and E/W as 2 in the relevent axis
    // Store the (n, e) value in the Hashmap
    for line in lines {
        let mut n: i32 = 0;
        let mut e: i32 = 0;
        let chars = line.chars().collect::<Vec<char>>();
        let mut i: usize = 0;
        while i < chars.len() {
            match chars[i] {
                'n' => {
                    n += 1;
                    e += if chars[i+1] == 'e' { 1 } else { -1 };
                    i+=1
                },
                's' => {
                    n -= 1;
                    e += if chars[i+1] == 'e' { 1 } else { -1 };
                    i+=1
                }
                'e' => {
                    e += 2;
                },
                'w' => {
                    e -= 2;
                },
                _ => {}
            }
            i += 1;
        }
        let tuple = (n,e);
        // If we've seen this entry before, flip it again
        // Otherwise initiate it as black.
        // The second value in the tuple is for part 2, the number of adjacet tiles.
        tiles.entry(tuple).and_modify(|x| x.0 = !x.0).or_insert((true, 0));
    }

    // Filter out white tiles
    tiles.retain(|&_, v| v.0 );

    // Part 1 is the remaining (black) tiles
    let part1 = tiles.len();

    // Set up the adjacent vectors.
    let adjacents = vec![
        (1 as i32,1 as i32),
        (1 as i32,-1 as i32),
        (-1 as i32,1 as i32),
        (-1 as i32,-1 as i32),
        (0 as i32,2 as i32),
        (0 as i32,-2 as i32),
    ];
    
    // Loop 100 times
    // In each loop, check each current black tile and mark off it and it's adjacents with incrementors
    // Then filter out the tiles that would not pass the is black rules
    for _ in 0..100 {
        // Empty the current list into a vector to iterate
        let tile_vec = tiles.drain().collect::<Vec<((i32, i32), (bool, usize))>>();
        for (key, _) in &tile_vec {
            // Add the current black tile to the hash, if it's there already (it's adjacent has already been processed) make sure it's stored as black.
            tiles.entry((key.0, key.1)).and_modify(|x| {x.0 = true}).or_insert((true, 0));
            // Check and add each adjecent tile to the list, incrementing their seen counter as we go.
            for o in &adjacents {
                tiles.entry((key.0 + o.0, key.1 + o.1)).and_modify(|x| x.1 += 1).or_insert((false, 1));
            }
        }  

        // Filter out the tiles that don't pass the rules
        tiles.retain(|&_, v| if v.0 { v.1 == 1 || v.1 == 2 } else { v.1 == 2 });   
    }

    // Return the number of tiles after 0 and 100 iterations.
    return Ok((part1 as i64, tiles.len() as i64));
}