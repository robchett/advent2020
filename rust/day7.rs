use std::collections::HashMap;

#[test]
fn test_run() {
    let res = run("light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.".to_owned());
    match res {
        Ok(i) => assert_eq!(i, (4, 32)),
        Err(e) => panic!(e)
    }
    let res = run("shiny gold bags contain 2 dark red bags.\ndark red bags contain 2 dark orange bags.\ndark orange bags contain 2 dark yellow bags.\ndark yellow bags contain 2 dark green bags.\ndark green bags contain 2 dark blue bags.\ndark blue bags contain 2 dark violet bags.\ndark violet bags contain no other bags.".to_owned());
    match res {
        Ok(i) => assert_eq!(i.1, 126),
        Err(e) => panic!(e)
    }
}

pub fn run(input: String) -> Result<(i64, i64), &'static str> {
    // Remove full stops from the end of lines so each split ends with bag or bags
    let clean_input = input.replace(".", "");
    // Each rule is one line
    let lines = clean_input.split("\n");
    // Set up a hashmap for quick lookups: Colour => (components, contained bags)
    let mut bags = HashMap::new();
    for line in lines {
        // Extract the first and last parts
        let mut split = line.split("s contain ").collect::<Vec<&str>>();

        // Discard anything that doens't split cleanly (shouldn't happen with this puzzle input)
        if split.len() == 2 {
            // Explode the component parts by ,
            let subs = split.pop().unwrap().split(", ").collect::<Vec<&str>>();
            let mut sub_tuples = vec![];
            // sub_count is the number of bags that make up the current rule
            // we can set to 1 if the bag can hold no others
            // otherwise set to -1 as it needs calculating later on
            let mut sub_count = 1;
            // Loop the components and extract the number required and the colour
            for sub in subs {
                // Extract and parse the first character as an it
                // If it's not an it then assume the rule is 'no other bags' so we have a root bag
                let s = &sub[0..1];
                let si = s.parse::<i32>();
                if si.is_ok() {
                    // Parsed as an int so we have a component, safe to unwrap.
                    let sj = si.unwrap();
                    let offset;
                    // Remove bag/bags, check the int to see if it needs pluralising.
                    if sj > 1 {
                        offset = sub.len() - 1;
                    } else {
                        offset = sub.len();
                    }
                    let e = &sub[2..offset];
                    sub_tuples.push((e, sj));
                    // This bag has components so we can't know the count yet
                    sub_count = -1;
                } else {
                    sub_tuples.push(("", 0));
                }
            }
            let key = split.pop().unwrap();
            bags.insert(key, (sub_tuples, sub_count));
        }
    }

    // Set up a stack of colours and start with our bag
    let mut stack = vec!["shiny gold bag"];
    // Set up a list of seen bags so we don't repeat ourselves
    let mut seen = vec![];
    // Pop the last item in the stack until it's empty
    while stack.len() > 0 {
        let curr = stack.pop().unwrap();
        // Check each rule to see if it requires the current colour
        for (&key, val) in bags.iter() {
            if val.0.iter().find(|x| x.0 == curr).is_some() {
                // The rule uses the colour so we can add it to the stack for traversal
                // don't bother if we've seen it before or it's already in the stack
                if !seen.contains(&key) && !stack.contains(&key) {
                    stack.push(&key)
                }
            }
        }
        // Add the item to the seen stack
        seen.push(&curr);
    }

    // Copy the bags as keys() makes the list immutable
    let bag_copy = bags.clone();
    // Extract a list of the colour names, we'll pop items from the list as we're able to determine their size
    let keys = bag_copy.keys();
    // Work out which key still need their size computing
    let mut to_calc = keys.filter(|x| bags.get(*x).unwrap().1 < 0).collect::<Vec<&&str>>();
    // While there are keys to compute loop them all.
    // Improvement: I'm computing scores for bags that won't get used, use the list of bags from part 1 to seed the list
    while to_calc.len() > 0 {
        for k in to_calc {
            // Get the corrisponding bag and see if all it's components have been computed yet.
            let b = bags.get(k).unwrap();
            let computed_children = b.0.iter().filter(|x| bags.get(x.0).unwrap().1 >= 0).collect::<Vec<&(&str, i32)>>();
            if computed_children.len() == b.0.len() {
                // All components are computed so we can calculate the total for this rule
                // Sum for each component: Multiply the size by the number of it we need
                let mut count = 1 as i64;
                for c in computed_children {
                    let d = bags.get(c.0).unwrap();
                    count += d.1 * c.1 as i64;
                }
                // Update the rule with the count in the original hashmap
                let update = bags.get_mut(k).unwrap();
                update.1 = count;
            }
        }
        // Recalculate our misisng keys for the loop.
        let keys = bag_copy.keys();
        to_calc = keys.filter(|x| bags.get(*x).unwrap().1 < 0).collect::<Vec<&&str>>();
    }
    
    // Return our answers, subtract 1 from each to account for our bag
    return Ok((seen.len() as i64 - 1, (bags.get("shiny gold bag").unwrap().1 - 1) as i64));
}