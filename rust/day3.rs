#[test]
fn test_run() {
    match run("..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#".to_owned()) {
        Ok(i) => assert_eq!(i, (7, 336)),
        Err(e) => panic!(e)
    }
}

pub fn run(input: String) -> Result<(i64, i64), &'static str> {
    let mut lines = input.split("\n");
    // Setup the storage for the tree counters.
    let mut counts = [0, 0, 0, 0, 0];
    // Setup the row incrementer
    let mut row = 0;
    // Read the first line and work out it's lenght
    // Improvement: Assumes all rows are the same length.
    let mut line = lines.nth(0).unwrap();
    let length = line.trim().len();
    // Vector of the different slopes used for parts 1 & 2.
    let indexes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    // Loop each row
    loop {
        // Loop each slope, index is used to store the tree count in {counts}
        for (i, coords) in indexes.iter().enumerate() {
            let (x, y) = coords;
            // Skip of the slope would bypass the current row
            if row % y == 0 {
                // Read the character from the row for the slope, use modulo to accomidate wrapping
                // Increment the counter if matches a tree '#'
                if line.chars().nth(row * x / y % length).unwrap() == '#' {
                    counts[i] += 1;
                }
            }
        }
        // Move to the next line.
        let line_option = lines.next();
        row += 1;
        // Break if the next line doesn't exist (end of the hill)
        match line_option {
            Some(x) => line = x,
            None => break,
        }
    }

    return Ok((
        // Return the slope (3,1) for part 1
        counts[1],
        // Return the product of all the slopes for part 2
        counts[0] * counts[1] * counts[2] * counts[3] * counts[4],
    ));
}
