#[test]
fn test_run() {
    let res = run("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb".to_owned());
    match res {
        Ok(i) => assert_eq!(i, (11, 6)),
        Err(e) => panic!(e)
    }
}

pub fn run(input: String) -> Result<(i32, i32), &'static str> {
    let mut part2: i32 = 0;
    let mut part1: i32 = 0;
    let input_clean = input.replace("\r", "");
    let groups = input_clean.split("\n\n");
    for group in groups {
        let mut group_all = vec![];
        let mut group_chars = vec![];
        let mut group_first = true;
        for person in group.split("\n").into_iter().map(|x| x.chars().collect::<Vec<char>>()) {
            let mut group_all_new = vec![];
            for c in person {
                if !group_chars.contains(&c) {
                    group_chars.push(c)
                }
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
        part1 += group_chars.len() as i32;
        part2 += group_all.len() as i32;
    }
    return Ok((part1, part2));
}