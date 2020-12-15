#[test]
fn test_run() {
    let res = run("0,3,6".to_owned());
    match res {
        Ok(i) => assert_eq!(i, (436, 175594)),
        Err(e) => panic!(e),
    }  
}

pub fn run(input: String) -> Result<(i64, i64), &'static str> {
    // Parse the input as numbers
    let numbers = input.split(",").collect::<Vec<&str>>().iter().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    // Compute the 2020th entry
    let part1 = compute(&numbers, 2020);
    // Compute the 30000000th entry,
    // Yes I could have returned both from one function call, but that would be a less clean approach.
    let part2 = compute(&numbers, 30000000);

    return Ok((part1, part2));
}

fn compute(start: &Vec<usize>, iters: usize) -> i64 {
    // Set up the history of all available positions, 
    // Testing showed that the high indexes do get used
    // A hashmap woudl be more space efficient, the list is sparse.
    // But that would also come with performance drawbacks. 
    // Taking the speed route instead of the RAM route.
    let mut history = vec![0 as usize; iters];
    let mut i = 1;
    // Storage for the previous values (last seen, number called)
    let mut prev = (0, 0);
    // Add the starting values to the history, store the prev.
    for j in start {
        prev = (history[*j], *j);
        history[*j] = i;
        i += 1;
    }

    // This is such a small amount of code but was a nightmare to get right
    // I don't know what was wrong with me but I just couldn't get my head around it.
    // Loop until the last turn (+1 because I've started from 1 not 0)
    for i in i..iters+1 {
        // j = the last seen time of the previous value.
        // If j is 0 the number has never been seen before, so our number is 0,
        // If j is > 0 then we will be using the difference between j and the current turn
        let mut j = prev.0;
        if prev.0 > 0 {
            j = i - prev.0 - 1;
        }
        // Store the last seen time and number called for the current number
        // Add it to the history afterwards.
        prev = (history[j], j);
        history[prev.1] = i;
    }

    return prev.1 as i64;

}
