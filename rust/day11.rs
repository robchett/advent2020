#[test]
fn test_run_11() {
    let res = run("L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL".to_owned());
    match res {
        Ok(i) => assert_eq!(i, (37, 26)),
        Err(e) => panic!(e),
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Position {
    EmptySeat,
    FilledSeat,
    Floor,
}

pub fn run(input: String) -> Result<(i64, i64), &'static str> {
    // Read each line and convert a 2d vector of Positions.
    let lines = input.split("\n");
    let mut map = vec![];
    for line in lines {
        let mut line_vec = vec![];
        for c in line.chars() {
            match c {
                'L' => line_vec.push((Position::EmptySeat, Position::EmptySeat)),
                '.' => line_vec.push((Position::Floor, Position::Floor)),
                // Could check for a filled seat here, but the task said they all start empty so error if that's not true
                _ => println!("Unknown position {}: {}", line, c),
            }
        }
        map.push(line_vec);
    }

    // Reduce each task to the positions that no longer change
    let mut map1 = map.to_owned();
    let mut map2 = map.to_owned();
    find_repeat(&mut map1, get_adjacent_filled, 3);
    find_repeat(&mut map2, get_visible_filled, 4);

    // Return the number of filled seats in each map
    return Ok((count_seats(&map1), count_seats(&map2)));
}

// Take a map, function to work out how many seats are filled for a given position and how many seats need to be empty to keep filled.
fn find_repeat(
    map: &mut Vec<Vec<(Position, Position)>>,
    cmp: fn(usize, usize, &Vec<Vec<(Position, Position)>>) -> i32,
    need: i32,
) {    
    // Loop until no changes have been made on a pass
    // Use a scan and update approach to remove unessisary copies of the space
    loop {
        for x in 0..map.len() {
            for y in 0..map[0].len() {
                let val = map[x][y];
                match val.0 {
                    // Make note of an empty seat if the comparison function returns no matches
                    Position::EmptySeat => {
                        let adjacent = cmp(x, y, map);
                        map[x][y].1 = if adjacent == 0 { Position::FilledSeat } else { Position::EmptySeat };
                    }
                    // Make note of a filled seat if there are {need} or more 'adjacents'
                    Position::FilledSeat => {
                        let adjacent = cmp(x, y, map);
                        map[x][y].1 = if adjacent > need { Position::EmptySeat } else { Position::FilledSeat };
                    },
                    _ => {}
                }
            }
        }
        // Check the noted changes and apply them
        let mut changes = false;
        for x in 0..map.len() {
            for y in 0..map[0].len() {
                let val = map[x][y];
                if val.0 != val.1 {
                    changes = true;
                    map[x][y].0 = map[x][y].1;
                }
            }
        }
        // If no changes were applied, our map is now complete.
        if !changes {
            return;
        }
    }
}

// Look in each direction for a filled seat
fn get_visible_filled(x: usize, y: usize, map: &Vec<Vec<(Position, Position)>>) -> i32 {
    let dirs = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut seen = 0;
    let x_size = map.len() as i32;
    let y_size = map[0].len() as i32;
    for dir in dirs {
        let mut x0 = x as i32;
        let mut y0 = y as i32;
        // Move in the direction until we hit the boundry or a seat
        loop {
            x0 += dir.0;
            y0 += dir.1;
            if x0 < 0 || x0 > x_size - 1 || y0 < 0 || y0 > y_size - 1 {
                break;
            }
            match map[x0 as usize][y0 as usize].0 {
                // Skip over floor
                Position::Floor => {}
                // Empty seat, not an adjacent
                Position::EmptySeat => break,
                // Filled seat, add to adjacent count.
                Position::FilledSeat => {
                    seen += 1;
                    break;
                }
            }
        }
    }
    return seen;
}

fn get_adjacent_filled(x: usize, y: usize, map: &Vec<Vec<(Position, Position)>>) -> i32 {
    let dirs = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut seen = 0;
    let x_size = map.len() as i32;
    let y_size = map[0].len() as i32;
    for dir in dirs {
        let x0 = x as i32 + dir.0;
        let y0 = y as i32 + dir.1;
        // check the adjacent seat in the direction, skip if we hit the boundry
        if x0 < 0 || x0 > x_size - 1 || y0 < 0 || y0 > y_size - 1 {
            continue;
        }
        match map[x0 as usize][y0 as usize].0 {
            // Skip over floor
            Position::Floor => {}
            // Empty seat, not an adjacent
            Position::EmptySeat => continue,
            // Filled seat, add to adjacent count.
            Position::FilledSeat => {
                seen += 1;
                continue;
            }
        }
    }
    return seen;
}

fn count_seats(map: &Vec<Vec<(Position, Position)>>) -> i64 {
    return map.into_iter().fold(0, |acc, v| {
        acc + v
            .into_iter()
            .fold(0, |acc, c| acc + (c.0 == Position::FilledSeat) as i64)
    });
}
