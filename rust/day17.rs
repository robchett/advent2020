#[test]
fn test_run_17() {
    let res = run(".#.\n..#\n###".to_owned());
    match res {
        Ok(i) => assert_eq!(i, (112, 848)),
        Err(e) => panic!(e),
    }  

}

pub fn run(input: String) -> Result<(i64, i64), &'static str> {
    let lines = input.split("\n").collect::<Vec<&str>>();
    // update our counters as we go, saves looping the grid at the end.
    let mut part_1: i64 = 0;
    let mut part_2: i64 = 0;
    // Set up our grids, increase them in size to accomidate the boundries, 
    let grid_size = lines.len() + 20;
    let mut grid1 = vec![vec![vec![vec![false; grid_size]; grid_size]; grid_size]; 1];
    let mut grid2 = vec![vec![vec![vec![false; grid_size]; grid_size]; grid_size]; grid_size];
    // Populate the starting grid and counters
    // Start in the middle of the grid so we don't have to worry about offsets for -ve indexes
    let offset = grid_size / 2;
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                grid1[0][offset][y + offset][x + offset] = true;
                grid2[offset][offset][y + offset][x + offset] = true;
                part_1 += 1;
                part_2 += 1;
            }
        }
    }

    // Pre map the different neighbor offsets in 3 and 4 dimensions
    let mut dirs1 = vec![];
    let mut dirs2 = vec![];
    let range: Vec<i8> = vec![-1, 0, 1];
    for x0 in &range {
        for y0 in &range {
            for z0 in &range {
                if *x0 != 0 || *y0 != 0 || *z0 != 0 {
                    dirs1.push((0, *z0,*y0,*x0));
                }
                for w0 in &range {
                    if *x0 != 0 || *y0 != 0 || *z0 != 0 || *w0 != 0 {
                        dirs2.push((*w0,*z0,*y0,*x0));
                    }
                }
            }
        }
    }

    // Loop the first 6 iterations of CGL in 3&4 dimensions
    for _ in 1..7 {
        // We're copying the grids here. Would probably be better to scan & update seperatly to avoid this.
        let mut new_grid1 = vec![vec![vec![vec![false; grid_size]; grid_size]; grid_size]; 1];
        let mut new_grid2 = vec![vec![vec![vec![false; grid_size]; grid_size]; grid_size]; grid_size];
        // Move through the entire grid one by one.
        for w in 1..grid_size-1 {
            for z in 1..grid_size-1 {
                for y in 1..grid_size-1 {
                    for x in 1..grid_size-1 {
                        // If we're at the center of the w space then we can loop at the 3d version.
                        if w == 1 {   
                            let o = compute_new(&grid1, &dirs1, x,y,z,0);           
                            new_grid1[0][z][y][x] = o.0;
                            part_1 += o.1 as i64; 
                        }
                        // Map the changes to the 4d version
                        let o = compute_new(&grid2, &dirs2, x,y,z,w);           
                        new_grid2[w][z][y][x] = o.0;
                        part_2 += o.1 as i64; 
                    }
                }
            }
        }
        // Swap the old and new grids
        grid1 = new_grid1;
        grid2 = new_grid2;
    }
   
    return Ok((part_1, part_2));
}

// Check how many neigboring points in 4d space are active, return the new active/inactive state + the change to our total active points.
fn compute_new(grid: &Vec<Vec<Vec<Vec<bool>>>>, dirs: &Vec<(i8,i8,i8,i8)>, x: usize, y: usize, z: usize, w: usize) -> (bool, i8) {
    let mut count = 0;
    for dir in dirs {
        if grid[(w as i8 + dir.0) as usize][(z as i8 + dir.1) as usize][(y as i8 + dir.2) as usize][(x as i8 + dir.3) as usize] {
            count += 1;
        }
    }
    if grid[w][z][y][x] && (count < 2 || count > 3) {
        return (false, -1);
    } else if !grid[w][z][y][x] && count == 3 {
        return (true, 1);
    } else {
        return (grid[w][z][y][x], 0);
    }
}