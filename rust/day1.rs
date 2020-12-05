pub fn run(input: String) -> Result<(i32, i32), &'static str> {
    let lines = input.split("\n");
    let mut points = Vec::new();
    for line in lines {
        let i = line.trim().parse::<i32>();
        match i {
            Ok(v) => points.push(v),
            Err(e) => println!("Error parsing {}: {}", line, e),
        }
    }
    return Ok((part1(&points), part2(&points)));
}

fn part1(input: &Vec<i32>) -> i32 {
    for n in 0..input.len() {
        for m in n..input.len() {
            if input[m] + input[n] == 2020 {
                return input[m] * input[n];
            }
        }
    }
    return 0;
}

fn part2(input: &Vec<i32>) -> i32 {
    for n in 0..input.len() {
        for m in n..input.len() {
            for o in m..input.len() {
                if input[m] + input[n] + input[o] == 2020 {
                    return input[m] * input[n] * input[o];
                }
            }
        }
    }

    return 0;
}
