#[test]
fn test_run() {
    let res = run("departure 1: 1-3 or 5-7\ndeparture 2: 6-11 or 33-44\ndeparture 3: 13-40 or 45-50\nyour ticket:\n7,1,14\nnearby tickets:\n7,3,47\n40,4,50\n55,2,20\n38,6,12".to_owned());
    match res {
        Ok(i) => assert_eq!(i, (71, 98)),
        Err(e) => panic!(e),
    }  
    let res = run("departure 1: 0-1 or 4-19\ndeparture 2: 0-5 or 8-19\ndeparture 3: 0-13 or 16-19\nyour ticket:\n11,12,13\nnearby tickets:\n3,9,18\n15,1,5\n5,14,9".to_owned());
    match res {
        Ok(i) => assert_eq!(i, (0, 1716)),
        Err(e) => panic!(e),
    }  
}

enum ParserSection {
    Rules,
    OurTicket,
    OtherTickets,
}

pub fn run(input: String) -> Result<(i64, i64), &'static str> {
    let lines = input.split("\n");
    // Out rules are ((start1..end1) (start2..end2) name column)
    let mut rules = vec![];
    // Storage for our ticket values
    let mut our_ticket = vec![];
    // Other tickets
    let mut other_tickets = vec![];
    // Start by parsing the rules
    let mut section = ParserSection::Rules;
    // Which of the rules are departures?
    let mut departures = vec![];

    for line in lines {
        match line {
            // Ignore blank lines
            "" => {}
            // Switch to parsing our ticket for the next line
            "your ticket:" => {
                section = ParserSection::OurTicket
            },
            // Following lines will be other tickets
            "nearby tickets:" => {
                section = ParserSection::OtherTickets
            },
            // Not a mode switcher or blank line, parse the inut by mode.
            _ => {
                match section {
                    // Extract the name: start1-end1 or start2..end2
                    // If name starts with departure, add this to our departures list
                    ParserSection::Rules => {
                        let r = line.split(":").collect::<Vec<&_>>();
                        if r[0].len() > 9 && r[0][0..9] == "departure".to_owned() {
                            departures.push(rules.len());
                        }
                        let parts = r[1].split(&[':', '-', ' '][..]).collect::<Vec<&_>>().iter().map(|x| x.parse::<u32>().unwrap_or_default()).collect::<Vec<u32>>();
                        rules.push(((parts[1], parts[2]), (parts[4], parts[5]), r[0], 0));
                    },
                    // Parse the , seperated list as ints, store as out_ticket
                    ParserSection::OurTicket => {
                        our_ticket = line.split(",").collect::<Vec<&_>>().iter().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
                    },
                    // Parse the , seperated list as ints, append to other_tickets
                    ParserSection::OtherTickets => {
                        other_tickets.push(line.split(",").collect::<Vec<&_>>().iter().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>());
                    },
                }
            }
        }
    }

    // Sum the values that fail all rule. 
    // If all the values in a ticket pass the rules then promote to the good_tickets list.
    let mut invalid = 0;
    let mut good_tickets = vec![];
    't: for (n,t) in other_tickets.iter().enumerate() {
        'i: for i in t {
            for rule in &rules {
                // If the value passes the rule, move to the next column
                if (i >= &rule.0.0 && i <= &rule.0.1) || (i >= &rule.1.0 && i <= &rule.1.1) {
                    continue 'i;
                }
            }
            // We've not continued, so this value fails all rules. Add it to the output and move to the next ticket. 
            invalid += i;
            continue 't;
        }
        // Passes all the rules, good ticket found
        good_tickets.push(n);
    }
    
    // Storage to the columns that pass a rule for each ticket
    let mut column_match = vec![(0, vec![]); rules.len()];

    // Loop each rule in turn, checking each column for each ticket to see which a valid
    for (r, rule) in rules.iter().enumerate() {
        'c: for c in 0..rules.len() {
            for n in &good_tickets {
                let t = &other_tickets[*n];
                let i = t[c];
                // If the value fails, move onto the next column
                if !((i >= rule.0.0 && i <= rule.0.1) || (i >= rule.1.0 && i <= rule.1.1)) {
                    continue 'c;
                }
            }
            // All tickets passed the rule for this column.
            column_match[r].0 = r;
            column_match[r].1.push(c);
        }
    }

    // Find the rules that is matched by only one column.
    // Remove that column from all the other rules
    // Repeat until we've found the 1 valid column for each rule.
    'start: while column_match.len() > 0 {
        for i in 0..column_match.len() {       
            // Skip rules currently covered by multiple columns
            if column_match[i].1.len() != 1 {
                continue;
            }
            // Store the correct column against the rule
            let v = column_match[i].1[0];
            rules[column_match[i].0].3 = v;
            // Loop the other rules are remove this as a possible column
            for j in 0..column_match.len() {
                // Ignore the current column
                if i == j {
                    continue;
                }
                for k in 0..column_match[j].1.len() {
                    if column_match[j].1[k] == v {
                        column_match[j].1.remove(k);
                        // Column should only be there once.
                        break;
                    }
                }
            }
            // Remove this rule for efficiency
            column_match.remove(i);
            // Restart the outer loop as the the lenth has changed
            continue 'start;
        }
    }

    // Multiply the number on our ticket for the given rules columns.
    let mut part2 = 1 as i64;
    for i in departures {
        part2 *= our_ticket[rules[i].3] as i64;
    }
    
   
    return Ok((invalid as i64, part2));
}
