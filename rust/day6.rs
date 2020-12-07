#[test]
fn test_run() {
    let res = run("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb".to_owned());
    match res {
        Ok(i) => assert_eq!(i, (11, 6)),
        Err(e) => panic!(e)
    }
}

pub fn run(input: String) -> Result<(i32, i32), &'static str> {
    // Set up outputs
    let mut part2: i32 = 0;
    let mut part1: i32 = 0;
    // Groups are seperated by a blank line
    let groups = input.split("\n\n");
    for group in groups {
        let mut group_all = vec![];
        let mut group_chars = vec![];
        let mut group_first = true;
        // Split the groups into char vectors
        for person in group.split("\n").into_iter().map(|x| x.chars().collect::<Vec<char>>()) {
            let mut group_all_new = vec![];
            for c in person {
                // Part 1: Add unique questions to the group array
                if !group_chars.contains(&c) {
                    group_chars.push(c)
                }
                // Part 2: all questions anwered by all people must be in the first person so start with all their answers
                // for each other person in the group we can keep the question if they answered it and drop it if they didn't
                // do a lookup and add to a new list if they pass
                // after the last person anything left was answered by everyone 
                if group_first {
                    group_all_new.push(c);
                } else {
                    if group_all.contains(&c) {
                        group_all_new.push(c);
                    }
                }
            }
            group_all = group_all_new;
            group_first = false;
        }
        // Increment part 1 by the unique answers
        part1 += group_chars.len() as i32;
        // Increament part 2 by the answers all had
        part2 += group_all.len() as i32;
    }
    return Ok((part1, part2));
}