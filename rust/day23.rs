#[test]
fn test_run_23() {
    let inputs = vec![
        ("389125467", (67384529, 149245887792)),       
    ];
    for i in inputs {
        let res = run(i.0.to_owned());
        match res {
            Ok(o) => assert_eq!(o, i.1),
            Err(e) => panic!(e),
        }
    }  
}

pub fn run(input: String) -> Result<(i64, i64), &'static str> {
    // Form a string from the results of the basic cup game to get the part 1 answer.
    let mut part1 = "0".to_owned();
    // Play the cup game for the basic 9 digits and 100 rotations
    let cups1 = calculate_cups(input.to_owned(), 9, 100);
    let mut prev = 1;
    // Starting (but skipping) 1, and rotating until we get back to 1, form a string of digets
    loop {
        let next = cups1[prev];        
        if next == 1 {
            break;
        }
        part1 = format!("{}{}", part1, next);
        prev = cups1[prev];
        
    }

    // Play the cup game with 1mil cups and 10mil rotations
    let cups2 = calculate_cups(input.to_owned(), 1_000_000, 10_000_000);


    return Ok((part1.parse::<i64>().unwrap(), (cups2[1] * cups2[cups2[1]]) as i64));
}

// Play the cup game with {size} cups, returns the final cup fomation after {count} rotations
// Populate the first digits with the input.
fn calculate_cups(input: String, size: usize, count: usize) -> Vec<usize> {
    // Form a list of cups, the index is the digit, and the value is the number that follows it in the list
    // Start with {size} blank cups, 0 index is unused, but leave it blank to avioid dealing with offsets.
    let mut cups = vec![0; size + 1];
    // Parse the input as the starting cup positions
    let digits = input.chars().collect::<Vec<char>>().iter().map(|x| x.to_digit(10).unwrap() as usize).collect::<Vec<usize>>();
    // The starting cursor is the first cup in the input
    let mut cursor = digits.first().unwrap().to_owned();

    // Using our starting list, update our list with the next digit
    let mut prev = digits.first().unwrap().to_owned();
    for d in 1..digits.len() {
        cups[prev] = digits[d];
        prev = digits[d];
    }
    // If we have more cups than inputs fill out the remainder as n -> n+1
    for d in (digits.len()+1)..size+1 {
        cups[prev] = d;
        prev = d;
    }
    // Loop back out last cup by referencing the first.
    cups[prev] = cursor;

    // Loop for the total count
    for _ in 0..count {   
        // Extract the next 3 cups so we can make sure our destination isn't one of them
        let pop1 = cups[cursor];
        let pop2 = cups[pop1];
        let pop3 = cups[pop2];
        let next = cups[pop3];

        // Work out where we're moving our popped list to
        // If the destination is in our popped list, then try the next lowest value
        // Loop back to the highest value if we reach the first
        let mut dest = cursor;
        loop {
            dest -= 1;
            if dest == 0 {
                dest = size;
            }
            if dest != pop1 && dest != pop2 && dest != pop3 {
                break;
            }
        }
        // Join our current point to the one after 3 popped llist
        cups[cursor] = next;
        // Join the end of the popped list to one after the destination
        cups[pop3] = cups[dest];
        // Join the destination onto the popped list
        cups[dest] = pop1;

        // Move the cursor on to the next value
        cursor = next
    }
    return cups;
}