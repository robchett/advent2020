#[test]
fn test_run() {
    let res = run("F10\nN3\nF7\nR90\nF11".to_owned());
    match res {
        Ok(i) => assert_eq!(i, (25, 286)),
        Err(e) => panic!(e),
    }
}

pub fn run(input: String) -> Result<(i64, i64), &'static str> {
    // Read each line and convert a 2d vector of Positions.
    let lines = input.split("\n");
    // In part 1 the starting direction is East, coordinates (0,0)
    let mut dir = 90;
    let mut part1 = (0, 0);
    // For part 2 the waypoint is at 1,10 (relative to the ship at (0,0))
    let mut waypoint_pos = (1, 10);
    let mut part2 = (0, 0);
    for line in lines {
        // Extract the Command t and Value n
        let t = line.chars().nth(0).unwrap();
        let n = &line[1..].parse::<i32>().unwrap();
        match t {
            // Move the ship N/E/S/W for part 1
            // Move the waypoint N/E/S/W for part 2
            'N' => { 
                part1.0 += n; 
                waypoint_pos.0 += n; 
            },            
            'S' => { 
                part1.0 -= n; 
                waypoint_pos.0 -= n; 
            },            
            'E' => { 
                part1.1 += n; 
                waypoint_pos.1 += n; 
            },            
            'W' => { 
                part1.1 -= n; 
                waypoint_pos.1 -= n; 
            },       
            // Rotate the ship/waypoint clockwise(R) or counter clockwise (L)     
            'L' => {
                dir -= n;
                // Keep dir in the range 0..360
                if dir < 0 {
                    dir += 360;
                }
                // Rotating the wayoint is just a case of flipping the x,y coordinates and changing the sign
                match n {
                    180 => {
                        waypoint_pos = (-waypoint_pos.0, -waypoint_pos.1);
                    },
                    90 =>{
                        waypoint_pos = (waypoint_pos.1, -waypoint_pos.0);
                    },
                    270 => {
                        waypoint_pos = (-waypoint_pos.1, waypoint_pos.0);
                    }
                    _ => {}
                }
            }           
            'R' => {
                dir += n;
                // Keep dir in the range 0..360
                if dir >= 360 {
                    dir -= 360;
                }      
                match n {
                    180 => {
                        waypoint_pos = (-waypoint_pos.0, -waypoint_pos.1);
                    },
                    90 =>{
                        waypoint_pos = (-waypoint_pos.1, waypoint_pos.0);
                    },
                    270 => {
                        waypoint_pos = (waypoint_pos.1, -waypoint_pos.0);
                    }
                    _ => {}
                }      
            },            
            // In part 1 move the ship forward n places, in the direction of dir
            // For part 2 move the ship towards the waypoint n times, the waypoint stays relative to the ship
            'F' => { 
                part2.0 += waypoint_pos.0 * n;
                part2.1 += waypoint_pos.1 * n;
                match dir {
                    0 => part1.0 += n,
                    180 => part1.0 -= n,
                    90 => part1.1 += n,
                    270 => part1.1 -= n,
                    _ => panic!("Direction is not cartisian: {}", dir),
                }
            },            
            _ => panic!("Unknown command: {}", t),
        }
    }
    // Return the Manhatten distance travelled by the ship (x+y)
    return Ok(((part1.0.abs() + part1.1.abs()) as i64, (part2.0.abs() + part2.1.abs()) as i64));
}