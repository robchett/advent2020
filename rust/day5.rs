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

pub fn run(input: String) -> Result<(i32, i32), &'static str> {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut max = 0;
    let mut min = 999;
    let mut our_seat = 0;
    let mut seats = vec![];
    for line in lines {
        let boarding_pass = BoardingPass::new(line.to_owned());
        let seat = boarding_pass.seat();
        seats.push(seat);
        if seat < min {
            min = seat;
        }
        if seat > max {
            max = seat;
        }
    }
    for i in min..max {
        if !seats.contains(&i) {
            our_seat = i;
            break;
        }
    }
    return Ok((max, our_seat));
}

struct BoardingPass {
    row: i32,
    column: i32,
}

impl BoardingPass {
    fn new(line: String) -> BoardingPass {
        let mut row = (0, 127);
        let mut col = (0, 7);
        for c in line.chars() {
            match c {
                'L' => col.1 = (col.0 + col.1 + 1) / 2,
                'R' => col.0 = (col.0 + col.1 + 1) / 2,
                'F' => row.1 = (row.0 + row.1 + 1) / 2,
                'B' => row.0 = (row.0 + row.1 + 1) / 2,
                _ => {}
            }
        }
        return BoardingPass {
            row: row.0,
            column: col.0,
        };
    }
    fn seat(&self) -> i32 {
        return (self.row * 8) + self.column;
    }
}
