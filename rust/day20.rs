#[test]
fn test_run_20() {
    let inputs = vec![
        ("Tile 2311:\n..##.#..#.\n##..#.....\n#...##..#.\n####.#...#\n##.##.###.\n##...#.###\n.#.#.#..##\n..#....#..\n###...#.#.\n..###..###\n\nTile 1951:\n#.##...##.\n#.####...#\n.....#..##\n#...######\n.##.#....#\n.###.#####\n###.##.##.\n.###....#.\n..#.#..#.#\n#...##.#..\n\nTile 1171:\n####...##.\n#..##.#..#\n##.#..#.#.\n.###.####.\n..###.####\n.##....##.\n.#...####.\n#.##.####.\n####..#...\n.....##...\n\nTile 1427:\n###.##.#..\n.#..#.##..\n.#.##.#..#\n#.#.#.##.#\n....#...##\n...##..##.\n...#.#####\n.#.####.#.\n..#..###.#\n..##.#..#.\n\nTile 1489:\n##.#.#....\n..##...#..\n.##..##...\n..#...#...\n#####...#.\n#..#.#.#.#\n...#.#.#..\n##.#...##.\n..##.##.##\n###.##.#..\n\nTile 2473:\n#....####.\n#..#.##...\n#.##..#...\n######.#.#\n.#...#.#.#\n.#########\n.###.#..#.\n########.#\n##...##.#.\n..###.#.#.\n\nTile 2971:\n..#.#....#\n#...###...\n#.#.###...\n##.##..#..\n.#####..##\n.#..####.#\n#..#.#..#.\n..####.###\n..#.#.###.\n...#.#.#.#\n\nTile 2729:\n...#.#.#.#\n####.#....\n..#.#.....\n....#..#.#\n.##..##.#.\n.#.####...\n####.#.#..\n##.####...\n##..#.##..\n#.##...##.\n\nTile 3079:\n#.#.#####.\n.#..######\n..#.......\n######....\n####.#..#.\n.#...#.##.\n#.#####.##\n..#.###...\n..#.......\n..#.###...", (20899048083289, 274)),       
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
    edges_all: Vec<String>,
    content: String,
    flipped: bool,
    rotated: Rotation
}

impl Tile {
    fn orient_top(&mut self, id: &String) {
        if self.edges[0][0] == *id {
            self.flipped = false;
            self.rotated = Rotation::Deg0;
        } else if self.edges[0][1] == *id {
            self.flipped = false;
            self.rotated = Rotation::Deg90;
        } else if self.edges[0][2] == *id {
            self.flipped = false;
            self.rotated = Rotation::Deg180;
        }else if self.edges[0][3] == *id {
            self.flipped = false;
            self.rotated = Rotation::Deg270;
        }else if self.edges[1][0] == *id {
            self.flipped = true;
            self.rotated = Rotation::Deg0;
        }else if self.edges[1][1] == *id {
            self.flipped = true;
            self.rotated = Rotation::Deg90;            
        }else if self.edges[1][2] == *id {
            self.flipped = true;
            self.rotated = Rotation::Deg180;            
        }else if self.edges[1][3] == *id {
            self.flipped = true;
            self.rotated = Rotation::Deg270;
        }
    } 

    fn orient_left(&mut self, id: &String) {
        if self.edges[0][0] == *id {
            self.flipped = true;
            self.rotated = Rotation::Deg90;
        } else if self.edges[0][1] == *id {
            self.flipped = true;
            self.rotated = Rotation::Deg0;
        } else if self.edges[0][2] == *id {
            self.flipped = true;
            self.rotated = Rotation::Deg270;
        }else if self.edges[0][3] == *id {
            self.flipped = true;
            self.rotated = Rotation::Deg180;
        }else if self.edges[1][0] == *id {
            self.flipped = false;
            self.rotated = Rotation::Deg90;
        }else if self.edges[1][1] == *id {
            self.flipped = false;
            self.rotated = Rotation::Deg0;            
        }else if self.edges[1][2] == *id {
            self.flipped = false;
            self.rotated = Rotation::Deg270;            
        }else if self.edges[1][3] == *id {
            self.flipped = false;
            self.rotated = Rotation::Deg180;
        }
    }
    
    fn get_right(&self) -> String {
        let f = if self.flipped { 1 } else { 0 };
        match self.rotated {
            Rotation::Deg0 => {
                self.edges[f][1].to_owned()
            },
            Rotation::Deg90 => {
                self.edges[f][2].to_owned()
            },
            Rotation::Deg180 => {
                self.edges[f][3].to_owned()
            },
            Rotation::Deg270 => {
                self.edges[f][0].to_owned()
            },
        }
    }

    fn get_bottom(&self) -> String {
        let f = if self.flipped { 1 } else { 0 };
        let o = match self.rotated {
            Rotation::Deg0 => {
                self.edges[f][2].to_owned()
            },
            Rotation::Deg90 => {
                self.edges[f][3].to_owned()
            },
            Rotation::Deg180 => {
                self.edges[f][0].to_owned()
            },
            Rotation::Deg270 => {
                self.edges[f][1].to_owned()
            },
        };
        return o.chars().rev().collect::<String>().to_owned();
    }
    fn draw(&self, space: &mut Vec<Vec<char>>, y0: usize, x0: usize, trim: bool) {
        let split = self.content.split("\n").collect::<Vec<&_>>();
        let tile_string = if trim { &split[2..split.len()-1] } else { &split[1..split.len()] };
        let size = if trim { 7 } else { 9 };
        for (y, line) in tile_string.iter().enumerate() {
            let chars;
            let chars_vec = line.chars().collect::<Vec<char>>();
            if trim {
                chars = &chars_vec[1..9];
            } else {
                chars = &chars_vec[0..10];
            }
            for (x, c) in chars.to_vec().iter().enumerate() { 
                let z = if self.flipped { size - x } else { x };
                match self.rotated {
                    Rotation::Deg0 => {
                        space[y0 + y][x0 + z] = *c;
                    },
                    Rotation::Deg90 => {
                        space[y0 + size - z][x0 + y] = *c;
                    },
                    Rotation::Deg180 => {
                        space[y0 + size - y][x0 + size - z] = *c;
                    },
                    Rotation::Deg270 => {
                        space[y0 + z ][x0 + size - y] = *c;
                    },
                }
            }
        }
    }
}

fn get_tile(id: i64, tiles: &Vec<Tile>) -> usize {
    for i in 0..tiles.len() {
        if tiles[i].id == id {
            return i;
        }
    }
    return 0;
}

fn get_tile_with(edge: &String, not: i64, tiles: &Vec<Tile>) -> Option<usize> {
    for i in 0..tiles.len() {
        if tiles[i].id != not && tiles[i].edges_all.contains(&edge) {
            return Some(i);
        }
    }
    return None;
}

fn parse_tiles(input: String) -> Vec<Tile> {
    let tiles_strings = input.split("\n\n").collect::<Vec<&_>>();
    let mut tiles = vec![];
    for tile_s in tiles_strings {
        let tile_string = tile_s.split("\n").collect::<Vec<&_>>();
        let id = tile_string[0][5..tile_string[0].len()-1].to_owned().parse::<i64>().unwrap();

        
        let ev = vec![
            parse_edge(tile_string[1].to_owned()), // Top
            parse_edge(tile_string[1..tile_string.len()].to_vec().iter().map(|x| x.chars().nth(x.len() - 1).unwrap()).collect::<Vec<char>>().into_iter().collect::<String>()), // Right
            parse_edge(tile_string[tile_string.len() - 1].chars().rev().collect::<String>().to_owned()), // Bottom
            parse_edge(tile_string[1..tile_string.len()].to_vec().iter().map(|x| x.chars().nth(0).unwrap()).collect::<Vec<char>>().into_iter().rev().collect::<String>()), // Left
        ];
            
        let edges_all = vec![
            ev[0].0.to_owned(),
            ev[0].1.to_owned(),
            ev[1].0.to_owned(),
            ev[1].1.to_owned(),
            ev[2].0.to_owned(),
            ev[2].1.to_owned(),
            ev[3].0.to_owned(),
            ev[3].1.to_owned()
        ];
        let edges = vec![
            vec![
                ev[0].0.to_owned(),
                ev[1].0.to_owned(),
                ev[2].0.to_owned(),
                ev[3].0.to_owned()
            ], 
            vec![
                ev[0].1.to_owned(),
                ev[3].1.to_owned(),
                ev[2].1.to_owned(),
                ev[1].1.to_owned()
            ]
        ];
        tiles.push(Tile{edges, edges_all, id, content:  tile_s.to_owned(), flipped: false, rotated: Rotation::Deg0});
    }
    return tiles;
}

fn find_corners(tiles: &Vec<Tile>) -> Vec<i64> {
    let mut out = vec![];
    for t1 in tiles {
        let mut seen = 0;
        for e in &t1.edges[0] {
            for t2 in tiles {
                if t2.id == t1.id {
                    continue;
                }
                if t2.edges_all.contains(&e) {
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

fn map_tiles(tiles: &mut Vec<Tile>) -> Vec<i64> {   
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

    let dim = (tiles.len() as f64).sqrt() as usize;
    let mut last_start = corner;
    let mut out = vec![];
    
    for i in 0..dim {
        let mut prev = 0;
        for j in 0..dim {
            if i == 0 && j == 0 {
                out.push(corner);
                let top_id = get_tile(corner, &tiles);
                println!("matching: .........., {}, {} is {} (Flipped: {}, Rotated: {:?})", i,j, tiles[top_id].id, tiles[top_id].flipped, tiles[top_id].rotated);
                prev = corner;
            } else if j == 0 {
                let top_id = &tiles[get_tile(last_start, &tiles)].get_bottom();
                let tile_index = get_tile_with(top_id, last_start, &tiles).unwrap();
                tiles[tile_index].orient_top(top_id);   
                out.push(tiles[tile_index].id);
                println!("matching: {}, {}, {} is {} (Flipped: {}, Rotated: {:?}) ", top_id, i,j, tiles[tile_index].id, tiles[tile_index].flipped, tiles[tile_index].rotated);
                last_start = tiles[tile_index].id;
                prev = tiles[tile_index].id;
            } else {
                let right_id = &tiles[get_tile(prev, &tiles)].get_right();
                let tile_index = get_tile_with(right_id, prev, &tiles).unwrap();
                tiles[tile_index].orient_left(right_id);  
                out.push(tiles[tile_index].id);
                println!("matching: {}, {}, {} is {} (Flipped: {}, Rotated: {:?}) ", right_id, i,j, tiles[tile_index].id, tiles[tile_index].flipped, tiles[tile_index].rotated);
                prev = tiles[tile_index].id;
            }
        }
    }
    return out;
}

pub fn run(input: String) -> Result<(i64, i64), &'static str> {
    let mut part2 = 0;
    let mut tiles = parse_tiles(input);
    let part1 = find_corners(&tiles).iter().fold(1, |acc, x| x * acc);

    let dim = (tiles.len() as f64).sqrt() as usize;
    let ordered_tiles = map_tiles(&mut tiles);

    let mut space = vec![vec![' '; 11*dim];11*dim];
    'outer: for v in 0..dim {
        for h in 0..dim {
            let tile_id = get_tile(ordered_tiles[v*dim + h], &tiles);
            tiles[tile_id].draw(&mut space, v*11, h*11, false);
        }
    }

    let mut space = vec![vec![' '; 8*dim]; 8*dim];
    'outer: for v in 0..dim {
        for h in 0..dim {
            let tile_id = get_tile(ordered_tiles[v*dim + h], &tiles);
            tiles[tile_id].draw(&mut space, v*8, h*8, true);
        }
    }

    let mut sea = 0;    
    for line in &space {
        sea += line.iter().fold(0, |acc, x| acc + if *x == '#' { 1 } else {0});
    }

    println!("Starting with {} sea cells", sea);

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
    'outer: for _ in vec![true, false] {
        for _ in 0..4 {
            for y in 0..(dim*8-2) {
                'x: for x in 0..(dim*8-20) {
                    for p in &sea_monster {
                        if space[y + p.0][x + p.1] != '#' {
                            continue 'x;
                        }
                    }
                    for p in &sea_monster {
                        space[y + p.0][x + p.1] = '0';
                        sea_monster_coords.push((y + p.0, x + p.1));
                    }
                    sea_monsters += 1;
                }
            }   
            if sea_monsters > 0 {
                break 'outer;
            }  
            space = rotate_picture(space);
        }
        space = flip_picture(space);
    }

    for line in &space {
        println!("{}", line.iter().collect::<String>());
    }

    println!("Found {} sea monsters!", sea_monsters);
    println!("Found sea monsters @ {:?}", sea_monster_coords);

    return Ok((part1 as i64, (sea - (sea_monsters*15)) as i64));
}

fn parse_edge(line: String) -> (String, String) {
    return (line.to_owned(), line.chars().rev().collect::<String>().to_owned());
}

fn rotate_picture(pic: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_pic = vec![vec![' '; pic.len()]; pic.len()];
    for y in 0..pic.len() {
        for x in 0..pic.len() {
            new_pic[x][pic.len() - y - 1] = pic[y][x];
        }
    }
    return new_pic;
}

fn flip_picture(pic: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_pic = vec![vec![' '; pic.len()]; pic.len()];
    for y in 0..pic.len() {
        for x in 0..pic.len() {
            new_pic[y][pic.len() - x - 1] = pic[y][x];
        }
    }
    return new_pic;
}