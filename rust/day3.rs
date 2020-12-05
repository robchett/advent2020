#[test]
fn test_run() {
    match run("..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#".to_owned()) {
        Ok(i) => assert_eq!(i, (7, 336)),
        Err(e) => panic!(e)
    }
}

pub fn run(input: String) -> Result<(i32, i32), &'static str> {
    let mut lines = input.split("\n");
    let mut counts = [0, 0, 0, 0, 0];
    let mut row = 0;
    let mut line = lines.nth(0).unwrap();
    let length = line.trim().len();
    let indexes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    loop {
        for (i, coords) in indexes.iter().enumerate() {
            let (x, y) = coords;
            if row % y == 0 {
                if line.chars().nth(row * x / y % length).unwrap() == '#' {
                    counts[i] += 1;
                }
            }
        }
        let line_option = lines.next();
        row += 1;
        match line_option {
            Some(x) => line = x,
            None => break,
        }
    }
    return Ok((
        counts[1],
        counts[0] * counts[1] * counts[2] * counts[3] * counts[4],
    ));
}
