#[test]
fn test_run_20() {
    let inputs = vec![
        ("Tile 2311:\n..##.#..#.\n##..#.....\n#...##..#.\n####.#...#\n##.##.###.\n##...#.###\n.#.#.#..##\n..#....#..\n###...#.#.\n..###..###\n\nTile 1951:\n#.##...##.\n#.####...#\n.....#..##\n#...######\n.##.#....#\n.###.#####\n###.##.##.\n.###....#.\n..#.#..#.#\n#...##.#..\n\nTile 1171:\n####...##.\n#..##.#..#\n##.#..#.#.\n.###.####.\n..###.####\n.##....##.\n.#...####.\n#.##.####.\n####..#...\n.....##...\n\nTile 1427:\n###.##.#..\n.#..#.##..\n.#.##.#..#\n#.#.#.##.#\n....#...##\n...##..##.\n...#.#####\n.#.####.#.\n..#..###.#\n..##.#..#.\n\nTile 1489:\n##.#.#....\n..##...#..\n.##..##...\n..#...#...\n#####...#.\n#..#.#.#.#\n...#.#.#..\n##.#...##.\n..##.##.##\n###.##.#..\n\nTile 2473:\n#....####.\n#..#.##...\n#.##..#...\n######.#.#\n.#...#.#.#\n.#########\n.###.#..#.\n########.#\n##...##.#.\n..###.#.#.\n\nTile 2971:\n..#.#....#\n#...###...\n#.#.###...\n##.##..#..\n.#####..##\n.#..####.#\n#..#.#..#.\n..####.###\n..#.#.###.\n...#.#.#.#\n\nTile 2729:\n...#.#.#.#\n####.#....\n..#.#.....\n....#..#.#\n.##..##.#.\n.#.####...\n####.#.#..\n##.####...\n##..#.##..\n#.##...##.\n\nTile 3079:\n#.#.#####.\n.#..######\n..#.......\n######....\n####.#..#.\n.#...#.##.\n#.#####.##\n..#.###...\n..#.......\n..#.###...", (20899048083289, 273)),       
    ];
    for i in inputs {
        let res = run(i.0.to_owned());
        match res {
            Ok(o) => assert_eq!(o, i.1),
            Err(e) => panic!(e),
        }
    }  
}

#[derive(Debug, PartialEq)]
enum Rotation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

#[derive(Debug)]
struct Tile {
    id: i64,
    edges: Vec<Vec<String>>,
    content: String,
    flipped: bool,
    rotated: Rotation
}

impl Tile {
    // Rotate/flip the tile the the passed line is at the top
    // Flip if the line is found in the flipped edges
    fn orient_top(&mut self, id: &String) {
        self.flipped = self.edges[1].contains(id);
        let t = if self.flipped { 1 } else { 0 };
        if self.edges[t][0] == *id {
            self.rotated = Rotation::Deg0;
        } else if self.edges[t][1] == *id {
            self.rotated = Rotation::Deg90;
        } else if self.edges[t][2] == *id {
            self.rotated = Rotation::Deg180;
        } else if self.edges[t][3] == *id {
            self.rotated = Rotation::Deg270;
        }
    } 

    // Rotate/flip the tile the the passed line is at the left
    // Start by orienting to the top, then flip and rotate 90 deg.
    fn orient_left(&mut self, id: &String) {
        self.orient_top(id);
        self.flipped = !self.flipped;
        self.rotated = match self.rotated {
            Rotation::Deg0 => Rotation::Deg90,
            Rotation::Deg90 => Rotation::Deg0,
            Rotation::Deg180 => Rotation::Deg270,
            Rotation::Deg270 => Rotation::Deg180
        }
    }
    
    // Get the pattern for the right hand side.
    fn get_right(&self) -> String {
        let f = if self.flipped { 1 } else { 0 };
        match self.rotated {
            Rotation::Deg0 => self.edges[f][1].to_owned(),
            Rotation::Deg90 => self.edges[f][2].to_owned(),
            Rotation::Deg180 => self.edges[f][3].to_owned(),
            Rotation::Deg270 => self.edges[f][0].to_owned()
        }
    }

    // Get the pattern for the bottom
    // Reverse this because we're comparing it to a top.
    fn get_bottom(&self) -> String {
        let f = if self.flipped { 1 } else { 0 };
        let o = match self.rotated {
            Rotation::Deg0 =>  self.edges[f][2].to_owned(),
            Rotation::Deg90 => self.edges[f][3].to_owned(),
            Rotation::Deg180 => self.edges[f][0].to_owned(),
            Rotation::Deg270 => self.edges[f][1].to_owned()
        };
        return o.chars().rev().collect::<String>().to_owned();
    }

    // Draw the tile with the relevent orientation
    // Write the characters to {space}
    // Offset by tile position {y0},{x0}
    // If {trim} is true, remove the borders
    fn draw(&self, space: &mut Vec<Vec<char>>, y0: usize, x0: usize, trim: bool) {
        let split = self.content.split("\n").collect::<Vec<&_>>();
        let tile_string = if trim { &split[2..split.len()-1] } else { &split[1..split.len()] };
        let size = if trim { 7 } else { 9 };
        for (y, line) in tile_string.iter().enumerate() {
            let chars_vec = line.chars().collect::<Vec<char>>();
            let chars = if trim { &chars_vec[1..9] } else { &chars_vec[0..10] };
            for (x, c) in chars.to_vec().iter().enumerate() { 
                // If we're in flipped space invert the x coordinate first
                let z = if self.flipped { size - x } else { x };
                match self.rotated {
                    Rotation::Deg0 => space[y0 + y][x0 + z] = *c,
                    Rotation::Deg90 => space[y0 + size - z][x0 + y] = *c,
                    Rotation::Deg180 => space[y0 + size - y][x0 + size - z] = *c,
                    Rotation::Deg270 => space[y0 + z][x0 + size - y] = *c
                }
            }
        }
    }
}

// Returns the index of the tile matching the ID
fn get_tile(id: i64, tiles: &Vec<Tile>) -> usize {
    for i in 0..tiles.len() {
        if tiles[i].id == id {
            return i;
        }
    }
    return 0;
}

// Returns the index (or None) of the other tile that has the given edge
fn get_tile_with(edge: &String, not: i64, tiles: &Vec<Tile>) -> Option<usize> {
    for i in 0..tiles.len() {
        if tiles[i].id != not && (tiles[i].edges[0].contains(&edge) || tiles[i].edges[1].contains(&edge)) {
            return Some(i);
        }
    }
    return None;
}

// Take the input and create or Tiles
fn parse_tiles(input: String) -> Vec<Tile> {
    // Tiles are sperated by a blank line.
    let tiles_strings = input.split("\n\n").collect::<Vec<&_>>();
    let mut tiles = vec![];

    for tile_s in tiles_strings {
        let tile_string = tile_s.split("\n").collect::<Vec<&_>>();
        // Extract the ID from the first line
        let id = tile_string[0][5..tile_string[0].len()-1].to_owned().parse::<i64>().unwrap();

        // Extract the first and last line, and the first and last columns from the tile
        // Reverse the bottom and left side as if we were reading it around a circle.
        let ev = vec![
            tile_string[1].to_owned(), // Top
            tile_string[1..tile_string.len()].to_vec().iter().map(|x| x.chars().nth(x.len() - 1).unwrap()).collect::<Vec<char>>().into_iter().collect::<String>(), // Right
            tile_string[tile_string.len() - 1].chars().rev().collect::<String>().to_owned(), // Bottom
            tile_string[1..tile_string.len()].to_vec().iter().map(|x| x.chars().nth(0).unwrap()).collect::<Vec<char>>().into_iter().rev().collect::<String>(), // Left
        ];
        // Form our edges from the read patterns, 
        // egdes[0] are the edges read clockwise from top left
        // edges[1] are the edges read counterclockwise from top right
        // I had tried to convert the patterns to ints as if binary, however this was harder to debug
        let edges = vec![ev.to_owned(), vec![
                ev[0].chars().rev().collect::<String>().to_owned().to_owned(),
                ev[3].chars().rev().collect::<String>().to_owned().to_owned(),
                ev[2].chars().rev().collect::<String>().to_owned().to_owned(),
                ev[1].chars().rev().collect::<String>().to_owned().to_owned()
            ]
        ];
        tiles.push(Tile{edges, id, content:  tile_s.to_owned(), flipped: false, rotated: Rotation::Deg0});
    }
    return tiles;
}

// Corners are the tiles that have just two sides that match other tiles patterns
// Thankfully the input didn't have any ambiguity here
fn find_corners(tiles: &Vec<Tile>) -> Vec<i64> {
    let mut out = vec![];
    for t1 in tiles {
        let mut seen = 0;
        for e in &t1.edges[0] {
            for t2 in tiles {
                if t2.id == t1.id {
                    continue;
                }
                if t2.edges[0].contains(&e) || t2.edges[1].contains(&e) {
                    seen += 1;
                }
            }
        }     
        if seen == 2 {
            out.push(t1.id);
        }
    }
    return out;
}

// Take our tiles and lay them out in a grid, orienting them so they join correctly
fn map_tiles(tiles: &mut Vec<Tile>) -> Vec<i64> {
    // Start by finding our first corner   
    let corner = find_corners(tiles)[0];
    let corner_tile_id = get_tile(corner, tiles);
    // Flip / Rotate the first corner until it is the top left one.
    'outer: for f in vec![true, false] {
        tiles[corner_tile_id].flipped = f; 
        for _ in 0..4 {
            let bottom_match = tiles[corner_tile_id].get_bottom();
            let bottom = get_tile_with(&bottom_match, corner, tiles);
            let right_match = tiles[corner_tile_id].get_right();
            let right = get_tile_with(&right_match, corner, tiles);
            if bottom.is_none() || right.is_none() {
                tiles[corner_tile_id].rotated = match tiles[corner_tile_id].rotated {
                    Rotation::Deg0 => Rotation::Deg90,
                    Rotation::Deg90 => Rotation::Deg180,
                    Rotation::Deg180 => Rotation::Deg270,
                    Rotation::Deg270 => Rotation::Deg0
                }
            } else {
                break 'outer;
            }
        }
    }

    // Work out the number of tiles in each row, the image is square to take the square root
    let dim = (tiles.len() as f64).sqrt() as usize;

    // We'll be moving left -> right, for the first item in each line we're matching it to the one above, so keep track of which tile that was
    // Is it cleaner to read from out[(i-1)*dim]?
    let mut last_start = corner;
    let mut out = vec![];
    
    // Loop our rows
    for i in 0..dim {
        // Keep track of our previous entry
        // Is it cleaner to read from out[i*dim + (j-1)]?
        let mut prev = 0;
        // Loop our columns
        for j in 0..dim {
            if i == 0 && j == 0 {
                // Stick our already rotated corner in the first position
                out.push(corner);
                // println!("matching: .........., {}, {} is {} (Flipped: {}, Rotated: {:?})", i,j, tiles[top_id].id, tiles[top_id].flipped, tiles[top_id].rotated);
                prev = corner;
            } else if j == 0 {
                // First column in a row, look at the tile above and find the tile that matches it's bottom row
                // Bottom is flipped as we're comparing it left -> right when it is originally right -> left
                // Stricktly speaking I should have inverted it when passing it to orient_top;
                let top_id = &tiles[get_tile(last_start, &tiles)].get_bottom();
                let tile_index = get_tile_with(top_id, last_start, &tiles).unwrap();
                // Orient the found tile so that it's top line matches the bottom of the one above
                tiles[tile_index].orient_top(top_id);   
                out.push(tiles[tile_index].id);
                // println!("matching: {}, {}, {} is {} (Flipped: {}, Rotated: {:?}) ", top_id, i,j, tiles[tile_index].id, tiles[tile_index].flipped, tiles[tile_index].rotated);
                last_start = tiles[tile_index].id;
                prev = tiles[tile_index].id;
            } else {
                // Look at the tile to the left and find the tile that matches it's right hand side
                let right_id = &tiles[get_tile(prev, &tiles)].get_right();
                let tile_index = get_tile_with(right_id, prev, &tiles).unwrap();
                // Orient the found tile so that it's left matches the right
                tiles[tile_index].orient_left(right_id);  
                out.push(tiles[tile_index].id);
                // println!("matching: {}, {}, {} is {} (Flipped: {}, Rotated: {:?}) ", right_id, i,j, tiles[tile_index].id, tiles[tile_index].flipped, tiles[tile_index].rotated);
                prev = tiles[tile_index].id;
            }
        }
    }
    return out;
}

pub fn run(input: String) -> Result<(i64, i64), &'static str> {
    // Parse our input into Tile objects
    let mut tiles = parse_tiles(input);
    // Locate the corners for part 1 and multiply their IDs
    let part1 = find_corners(&tiles).iter().fold(1, |acc, x| x * acc);

    // The tiles fit in a square so calculate the size
    let dim = (tiles.len() as f64).sqrt() as usize;
    // Get the tiles as they would be read ltr
    let ordered_tiles = map_tiles(&mut tiles);

    // For debugging the drawn map, useful to see them lined up side by side to make sure the borders match
    // let mut space = vec![vec![' '; 11*dim];11*dim];
    // for v in 0..dim {
    //     for h in 0..dim {
    //         let tile_id = get_tile(ordered_tiles[v*dim + h], &tiles);
    //         tiles[tile_id].draw(&mut space, v*11, h*11, false);
    //     }
    // }

    // Combine the times into a single array of characters
    let mut space = vec![vec![' '; 8*dim]; 8*dim];
    for v in 0..dim {
        for h in 0..dim {
            let tile_id = get_tile(ordered_tiles[v*dim + h], &tiles);
            tiles[tile_id].draw(&mut space, v*8, h*8, true);
        }
    }

    // This is the pattern for the sea monster!
    let sea_monster = vec![
        (1,0),
        (2,1),
        (2,4),
        (1,5),
        (1,6),
        (2,7),
        (2,10),
        (1,11),
        (1,12),
        (2,13),
        (2,16),
        (0,18),
        (1,17),
        (1,18),
        (1,19),
    ];

    let mut sea_monsters = 0;
    let mut sea_monster_coords = vec![];
    // Rotate 90deg at a time, the flip and rotate again until we find an orientation that contains sea monsters.
    'outer: for _ in vec![true, false] {
        for _ in 0..4 {
            // Scan the picture, setting the bounds so a sea monster won't be cut off the edge of the space
            for y in 0..(dim*8-2) {
                'x: for x in 0..(dim*8-20) {
                    // Check each point of the sea monster to see if it's a # character
                    // Stop checking as soon as a blank space is found can discard this start
                    for p in &sea_monster {
                        if space[y + p.0][x + p.1] != '#' {
                            continue 'x;
                        }
                    }
                    // Valid sea monster found!
                    // Mark it's coordinates with a 0 for outputting a pretty picture
                    // This assums that sea monsters can't overlap
                    for p in &sea_monster {
                        space[y + p.0][x + p.1] = '0';
                        sea_monster_coords.push((y + p.0, x + p.1));
                    }
                    sea_monsters += 1;
                }
            }   
            // If we found sea monsters in this picture we can stop
            // Otherwise we need to flip/rotate again.
            if sea_monsters > 0 {
                break 'outer;
            }  
            space = rotate_picture(space);
        }
        space = flip_picture(space);
    }

    // for line in &space {
    //     // println!("{}", line.iter().collect::<String>());
    // }
    // println!("Found {} sea monsters!", sea_monsters);
    // println!("Found sea monsters @ {:?}", sea_monster_coords);

    // For part 2, find out how many sea cells we have left
    let mut sea = 0;    
    for line in &space {
        sea += line.iter().fold(0, |acc, x| acc + if *x == '#' { 1 } else {0});
    }

    return Ok((part1 as i64, sea as i64));
}

// Create a new space rotated 90deg counter clockwise
fn rotate_picture(pic: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_pic = vec![vec![' '; pic.len()]; pic.len()];
    for y in 0..pic.len() {
        for x in 0..pic.len() {
            new_pic[x][pic.len() - y - 1] = pic[y][x];
        }
    }
    return new_pic;
}

// Create a new space flipped in the vertical axis
fn flip_picture(pic: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_pic = vec![vec![' '; pic.len()]; pic.len()];
    for y in 0..pic.len() {
        for x in 0..pic.len() {
            new_pic[y][pic.len() - x - 1] = pic[y][x];
        }
    }
    return new_pic;
}