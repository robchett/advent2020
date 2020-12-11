#[test]
fn test_run() {
    let res = run("L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL".to_owned());
    match res {
        Ok(i) => assert_eq!(i, (37, 26)),
        Err(e) => panic!(e),
    }
}

#[derive(PartialEq)]
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
                'L' => line_vec.push(Position::EmptySeat),
                '.' => line_vec.push(Position::Floor),
                // Could check for a filled seat here, but the task said they all start empty so error if that's not true
                _ => println!("Unknown position {}: {}", line, c),
            }
        }
        map.push(line_vec);
    }

    // Reduce each task to the positions that no longer change
    let part1 = find_repeat(&map, get_adjacent_filled, 3);
    let part2 = find_repeat(&map, get_visible_filled, 4);

    // Return the number of filled seats in each map
    return Ok((count_seats(&part1), count_seats(&part2)));
}

// Take a map, function to work out how many seats are filled for a given position and how many seats need to be empty to keep filled.
fn find_repeat(
    map: &Vec<Vec<Position>>,
    cmp: fn(usize, usize, &Vec<Vec<Position>>) -> i32,
    need: i32,
) -> Vec<Vec<Position>> {
    // Copy the orinal map
    let mut prev_map = vec![];
    for line in map {
        let mut new_line = vec![];
        for val in line {
            new_line.push(match val {
                Position::Floor => Position::Floor,
                Position::FilledSeat => Position::FilledSeat,
                Position::EmptySeat => Position::EmptySeat,
            });
        }
        prev_map.push(new_line);
    }
    // Loop until no changes have been made on a pass
    loop {
        let mut new_map = vec![];
        let mut changes = false;
        for (x, line) in prev_map.iter().enumerate() {
            let mut new_line = vec![];
            for (y, val) in line.iter().enumerate() {
                match val {
                    // Floor maintinas a floor
                    Position::Floor => {
                        new_line.push(Position::Floor);
                    }
                    // Swap an empty seat if the comparison function returns no matches
                    Position::EmptySeat => {
                        let adjacent = cmp(x, y, &prev_map);
                        if adjacent == 0 {
                            new_line.push(Position::FilledSeat);
                            changes = true;
                        } else {
                            new_line.push(Position::EmptySeat);
                        }
                    }
                    // Swap a filled seat if there are 'need' or more 'adjacents'
                    Position::FilledSeat => {
                        let adjacent = cmp(x, y, &prev_map);
                        if adjacent > need {
                            new_line.push(Position::EmptySeat);
                            changes = true;
                        } else {
                            new_line.push(Position::FilledSeat);
                        }
                    }
                }
            }
            new_map.push(new_line);
        }
        if !changes {
            break;
        }
        prev_map = new_map;
    }
    return prev_map;
}

// Look in each direction for a filled seat
fn get_visible_filled(x: usize, y: usize, map: &Vec<Vec<Position>>) -> i32 {
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
    let y_size = map.get(0).unwrap().len() as i32;
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
            match *map.get(x0 as usize).unwrap().get(y0 as usize).unwrap() {
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

fn get_adjacent_filled(x: usize, y: usize, map: &Vec<Vec<Position>>) -> i32 {
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
    let y_size = map.get(0).unwrap().len() as i32;
    for dir in dirs {
        let x0 = x as i32 + dir.0;
        let y0 = y as i32 + dir.1;
        // check the adjacent seat in the direction, skip if we hit the boundry
        if x0 < 0 || x0 > x_size - 1 || y0 < 0 || y0 > y_size - 1 {
            continue;
        }
        match *map.get(x0 as usize).unwrap().get(y0 as usize).unwrap() {
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

fn count_seats(map: &Vec<Vec<Position>>) -> i64 {
    return map.into_iter().fold(0, |acc, v| {
        acc + v
            .into_iter()
            .fold(0, |acc, c| acc + (*c == Position::FilledSeat) as i64)
    });
}
