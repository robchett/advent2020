#[test]
fn test_run() {
    assert_eq!(extended_euclidean_algorithm(7,5).unwrap(), 3);
    assert_eq!(extended_euclidean_algorithm(15,26).unwrap(), 7);

    assert_eq!(chinese_remainder((0, 1789), (36, 37)), (30413, 66193));
    assert_eq!(chinese_remainder((30413, 66193), (45, 47)), (1288080, 3111071));
    assert_eq!(chinese_remainder((1288080, 3111071), (1886, 1889)), (1202161486, 5876813119));

    let res = run("939\n7,13,x,x,59,x,31,19".to_owned());
    match res {
        Ok(i) => assert_eq!(i, (295, 1068781)),
        Err(e) => panic!(e),
    }
    let res = run("939\n17,x,13,19".to_owned());
    match res {
        Ok(i) => assert_eq!(i.1, 3417),
        Err(e) => panic!(e),
    }
    let res = run("939\n67,7,59,61".to_owned());
    match res {
        Ok(i) => assert_eq!(i.1, 754018),
        Err(e) => panic!(e),
    }
    let res = run("939\n67,x,7,59,61".to_owned());
    match res {
        Ok(i) => assert_eq!(i.1, 779210),
        Err(e) => panic!(e),
    }
    let res = run("939\n67,7,x,59,61".to_owned());
    match res {
        Ok(i) => assert_eq!(i.1, 1261476),
        Err(e) => panic!(e),
    }
    let res = run("939\n1789,37,47,1889".to_owned());
    match res {
        Ok(i) => assert_eq!(i.1, 1202161486),
        Err(e) => panic!(e),
    }
}

pub fn run(input: String) -> Result<(i64, i64), &'static str> {
    let mut lines = input.split("\n");
    // Extract the departure time from the input
    let depart = lines.nth(0).unwrap().parse::<u64>().unwrap();
    // Setup storage for the first bus after depart (minutes from depart, bus#)
    let mut first = (999999, 999999);
    // Storage for part 2 [(bus#, departure offset)].
    let mut busses = vec![];
    // Loop each input
    for (i,b) in lines.nth(0).unwrap().split(",").collect::<Vec<&_>>().iter().enumerate() {
        // Parse as ints, ignore x
        match b.parse::<u64>() {
            Ok(v) => {
                // Check if the remainder of depart time % bus# < current max, store it.
                if v - depart % v < first.0 {
                    first = (v - depart % v, v);
                } 
                busses.push((v, i as u64));
            }
            Err(_) => {}
        }
    }

    // This was quick, but doens't scale well.
    // let mut time = (max.0 - max.1) as u64;
    // 'outer: loop {
    //     for b in &busses {
    //         let t = time % b.0 as u64;
    //         let m = (b.0 as u64 - t) % b.0 as u64;
    //         if m != b.1 as u64 {
    //             time += max.1 as u64;
    //             continue 'outer;
    //         }
    //     }
    //     break;
    // }

    // For part 2, we're solving
    // x = (bus 0 offset) mod (bus 0 number) 
    // x = (bus 1 offset) mod (bus 1 number) 
    // ...
    // x = (bus n offset) mod (bus n number) 
    // 
    // This is solved by recursing the chinese_remainder_theorem
    // Luckily all our bus#s are primes so this is simple.
    let mut crt = (0, 1);
    for b in &busses {
        crt = chinese_remainder(crt, (b.0 - (b.1 % b.0), b.0));
    };
   
    // Return first_bus# * first_bus_delay for part 1,
    // Return the CRT result for part 2.
    return Ok(((first.1 * first.0) as i64, crt.0  as i64));
}

// CRT solves
// x = a mod b 
// x = c mod d
// => x = (inv(b, d)(c - a) mod d) mod (b*d)
// Where inv(b, d) is the multiplicative inverse of b in modulo space d. We use extended_euclidean_algorithm for this.
fn chinese_remainder(a: (u64, u64), b: (u64, u64)) -> (u64, u64) {
    let invert = extended_euclidean_algorithm(a.1, b.1).unwrap() as i64;
    let diff = b.0 as i64 - a.0 as i64;
    let mut k = (invert * diff) % b.1 as i64;
    if k < 0 {
        k += b.1 as i64;
    }
    return (k as u64 * a.1 + a.0, a.1 * b.1);
}

// Extended Euclidean Algorithm
// Can't be arsed to work out how this works
// Converted from the psudocode on https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Computing_multiplicative_inverses_in_modular_structures
fn extended_euclidean_algorithm(a: u64,n: u64) -> Result<u64, String> {
    let mut t = (0 as i64, 1 as i64);
    let mut r = (n as i64, a as i64);

    while r.1 != 0 {
        let quotient = r.0 / r.1;
        t = (t.1, t.0 - quotient * t.1);
        r = (r.1, r.0 - quotient * r.1);
    }
    if r.0 > 1 {
        return Err(format!("{} is not invertible", a));
    }
    if t.0 < 0 {
        t.0 = t.0 + n as i64
    }

    return Ok(t.0 as u64);
}