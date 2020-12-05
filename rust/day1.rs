#[test]
fn test_run() {
    let res = run("1721\n979\n366\n299\n675\n1456".to_owned());
    match res {
        Ok(i) => assert_eq!(i, (514579, 241861950)),
        Err(e) => panic!(e),
    }
}

pub fn run(input: String) -> Result<(i32, i32), &'static str> {
    let lines = input.split("\n");
    let mut input = Vec::new();
    for line in lines {
        let i = line.trim().parse::<i32>();
        match i {
            Ok(v) => input.push(v),
            Err(e) => println!("Error parsing {}: {}", line, e),
        }
    }
    let mut res1 = 0;
    let mut res2 = 0;
    for n in 0..input.len() {
        for m in n..input.len() {
            if input[m] + input[n] == 2020 {
                res1 = input[m] * input[n];
            }
            for o in m..input.len() {
                if input[m] + input[n] + input[o] == 2020 {
                    res2 = input[m] * input[n] * input[o];
                }
            }
        }
    }
    return Ok((res1, res2));
}
