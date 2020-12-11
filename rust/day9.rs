#[test]
fn test_run() {
    let res = find_fault(
        &"35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576"
            .to_owned(),
        5,
    );

    match res {
        Ok(i) => assert_eq!(i, 127),
        Err(e) => panic!(e),
    }

    let res2 = find_contiguous(
        &"35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576"
            .to_owned(),
        127,
    );
    match res2 {
        Ok(i) => assert_eq!(i, 62),
        Err(e) => panic!(e),
    }
}

pub fn run(input: String) -> Result<(i64, i64), &'static str> {
    let part1: i64;
    let part2: i64;
    // Find the fault code in the input string
    let res1 = find_fault(&input, 25);
    match res1 {
        Ok(v) => part1 = v,
        Err(e) => return Err(e),
    }
    // Find the set that adds up to the fault code
    let res2 = find_contiguous(&input, part1 as i64);
    match res2 {
        Ok(v) => part2 = v,
        Err(e) => return Err(e),
    }
    return Ok((part1, part2));
}

fn find_fault(input: &String, preamble: usize) -> Result<i64, &'static str> {
    // Split the lines and parse them as large ints (input exceeds 32bit size)
    let lines = input.split("\n");
    let mut ints = vec![];
    for s in lines {
        match s.parse::<i64>() {
            Ok(i) => {
                // Once we have enough ints to exceed the preamble we can start looking for faults
                // Take a slice of the ints from the end and check for a matching sum
                if ints.len() > preamble {
                    let slice = &ints[(ints.len() - preamble)..];
                    // If not matching sum, return our fault code
                    if !find_match(slice, i) {
                        return Ok(i);
                    }
                }
                ints.push(i)
            }
            Err(_) => return Err("Failed to parse int"),
        }
    }
    return Err("No fault code found");
}

// Checks for a matching pair to val
// Do the arithmatic out of the inner loop for speed, lookup is cheaper than multiple sums.
fn find_match(slice: &[i64], val: i64) -> bool {
    for j in slice {
        if slice.contains(&(val - j)) {
            return true;
        }
    }
    return false;
}

fn find_contiguous(input: &String, lookup: i64) -> Result<i64, &'static str> {
    // Parse all our ints again
    let lines = input.split("\n");
    let mut ints = vec![];
    for s in lines {
        match s.parse::<i64>() {
            Ok(i) => ints.push(i),
            Err(_) => return Err("Failed to parse int"),
        }
    }
    // Loop all the starting points
    let s = ints.len();
    // Name the outer loop so we can skip out of the inner one
    'outer: for i in 0..s {
        // Build the current sum starting with the start value
        let mut sum = *ints.get(i).unwrap();
        for j in i + 1..s {
            // Contunue to loop the next values until we either match or exceed our target
            sum += ints.get(j).unwrap();
            if sum == lookup {
                // If we match our target, extract the min max and return their sum
                let mut min = lookup;
                let mut max = 0;
                for s in &ints[i..j + 1] {
                    if min > *s {
                        min = *s;
                    }
                    if max < *s {
                        max = *s;
                    }
                }
                return Ok(min + max);
            } else if sum > lookup {
                // If we exceed the target move on to the next start value
                continue 'outer;
            }
        }
    }

    return Ok(0);
}
