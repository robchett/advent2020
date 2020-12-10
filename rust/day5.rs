#[test]
fn test_run() {
    let boarding_pass = BoardingPass::new("BFFFBBFRRR".to_owned());
    assert_eq!(
        (
            boarding_pass.row,
            boarding_pass.column,
            boarding_pass.seat()
        ),
        (70, 7, 567)
    );
    let boarding_pass = BoardingPass::new("FFFBBBFRRR".to_owned());
    assert_eq!(
        (
            boarding_pass.row,
            boarding_pass.column,
            boarding_pass.seat()
        ),
        (14, 7, 119)
    );
    let boarding_pass = BoardingPass::new("BBFFBBFRLL".to_owned());
    assert_eq!(
        (
            boarding_pass.row,
            boarding_pass.column,
            boarding_pass.seat()
        ),
        (102, 4, 820)
    );
}

pub fn run(input: String) -> Result<(i64, i64), &'static str> {
    // Boarding passes are a single line each
    let lines = input.split("\n");
    // set up the bounds for part 2
    let mut max = 0;
    let mut min = 99999;
    // set up the output for part 2
    let mut our_seat = 0;
    // Vector of seat IDs
    let mut seats = vec![];
    for line in lines {
        // Parse the string to a Boarding pass
        let boarding_pass = BoardingPass::new(line.to_owned());
        let seat = boarding_pass.seat();
        // Add the seat ID to vector
        seats.push(seat);
        // See if it is less than the current min and store if it is
        if seat < min {
            min = seat;
        }
        // See if it is greater than the current max and store if it is
        if seat > max {
            max = seat;
        }
    }
    // Loop each index between the min & max to see if it's missing from the vector
    // Imporvement: See if it's quicker to sort first then look for a missing index?
    for i in min..max {
        if !seats.contains(&i) {
            our_seat = i;
            break;
        }
    }
    return Ok((max as i64, our_seat as i64));
}

struct BoardingPass {
    row: i32,
    column: i32,
}

impl BoardingPass {
    fn new(line: String) -> BoardingPass {
        // Set up out binary search ranges
        let mut row = (0, 127);
        let mut col = (0, 7);
        // Loop each character and modify the relevent upper/lower bound
        for c in line.chars() {
            match c {
                // Update upper half of col
                'L' => col.1 = (col.0 + col.1 + 1) / 2,
                // Update lower half or col
                'R' => col.0 = (col.0 + col.1 + 1) / 2,
                // Update upper half of row
                'F' => row.1 = (row.0 + row.1 + 1) / 2,
                // Update lower half or row
                'B' => row.0 = (row.0 + row.1 + 1) / 2,
                _ => {}
            }
        }
        return BoardingPass {
            row: row.0,
            column: col.0,
        };
    }
    // Returns the seat ID calculated from the row an column
    fn seat(&self) -> i32 {
        return (self.row * 8) + self.column;
    }
}
