#[test]
fn test_run() {
    let res = run("16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4".to_owned());
    match res {
        Ok(i) => assert_eq!(i, (35, 8)),
        Err(e) => panic!(e)
    }
    let res = run("28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3".to_owned());
    match res {
        Ok(i) => assert_eq!(i, (220, 19208)),
        Err(e) => panic!(e)
    }
}

pub fn run(input: String) -> Result<(i64, i64), &'static str> {
    // Read each line and convert to a vector of integers.
    let lines = input.split("\n");
    let mut input = vec![];

    // We always start at the 0 voltage.
    // Doesn't effect part 1 as we start by comparing 0->0 which doens't change the output.
    input.push(0);
    for line in lines {
        let i = line.trim().parse::<i32>();
        match i {
            Ok(v) => input.push(v),
            Err(e) => println!("Error parsing {}: {}", line, e),
        }
    }

    // Sort the input as we're always looking at them in order
    input.sort();

    // Set up the couters for the number of 1 and 3 skips.
    // There will always be a 3er at the end.
    let mut out = (0, 1);
    let mut prev = 0;
    // Also build up sections seperated by a 3er for part 2
    let mut prev_sec = 0;
    let mut sections = vec![];
    let mut out2 = 1 as i64;

    // Loop and caclulate differences
    // 1s and 3s go to the part one output
    // 3s also seperarte sections
    for (i, v) in input.iter().enumerate() {
        match *v - prev {
            1 => out.0 += 1,
            3 => {
                out.1 += 1;
                sections.push(&input[prev_sec..i]);
                prev_sec = i;
            },
            _ => {},
        }
        prev = *v;
    }
    sections.push(&input[prev_sec..]);

    // For each section find out how many times we branch and multiply all the sections for our result
    for s in sections {
        let sec_branches = 1 + reduce(&s[..]) as i64;
        out2 *= sec_branches;
    }

    return Ok((out.0 * out.1, out2));
}

// Recurse into smaller and smaller slices until we have no possible branches (can't branch with 2 points)
// This is quite dumb, it will track the same branch multiple times
// However as we've split the full list first it's still very quick
fn reduce(points: &[i32]) -> i32 {
    let v = points[0];
    let mut out = 0;
    if points.len() < 3 {
        return out;
    }
    let c1 = points.get(1).unwrap_or(&9999) - v <= 3;
    let c2 = points.get(2).unwrap_or(&9999) - v <= 3;
    let c3 = points.get(3).unwrap_or(&9999) - v <= 3;
    // If there are 3 possible branches add 2 to the new branch counter
    // If there are two possible branches add 1,
    // Otherwise we can't branch so leave as is.
    if c1 && c2 && c3 {
        out += 2;
    } else if (c1 && c2) || (c1 && c3) || (c2 && c3) {
        out += 1;
    }
    // Then recurse from the start of each new branch and add those to the result
    if c1 {
        out += reduce(&points[1..])
    }
    if c2 {
        out += reduce(&points[2..])
    }
    if c3 {
        out += reduce(&points[3..])
    }
    return out;
}