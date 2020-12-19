#[test]
fn test_run_19() {
    let inputs = vec![
        ("0: 4 1 5\n1: 2 3 | 3 2\n2: 4 4 | 5 5\n3: 4 5 | 5 4\n4: \"a\"\n5: \"b\"\n\nababbb\nbababa\nabbbab\naaabbb\naaaabbb", (2, 2)),       
        ("42: 9 14 | 10 1\n9: 14 27 | 1 26\n10: 23 14 | 28 1\n1: \"a\"\n11: 42 31\n5: 1 14 | 15 1\n19: 14 1 | 14 14\n12: 24 14 | 19 1\n16: 15 1 | 14 14\n31: 14 17 | 1 13\n6: 14 14 | 1 14\n2: 1 24 | 14 4\n0: 8 11\n13: 14 3 | 1 12\n15: 1 | 14\n17: 14 2 | 1 7\n23: 25 1 | 22 14\n28: 16 1\n4: 1 1\n20: 14 14 | 1 15\n3: 5 14 | 16 1\n27: 1 6 | 14 18\n14: \"b\"\n21: 14 1 | 1 14\n25: 1 1 | 1 14\n22: 14 14\n8: 42\n26: 14 22 | 1 20\n18: 15 15\n7: 14 5 | 1 21\n24: 14 1\n\nabbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa\nbbabbbbaabaabba\nbabbbbaabbbbbabbbbbbaabaaabaaa\naaabbbbbbaaaabaababaabababbabaaabbababababaaa\nbbbbbbbaaaabbbbaaabbabaaa\nbbbababbbbaaaaaaaabbababaaababaabab\nababaaaaaabaaab\nababaaaaabbbaba\nbaabbaaaabbaaaababbaababb\nabbbbabbbbaaaababbbbbbaaaababb\naaaaabbaabaaaaababaa\naaaabbaaaabbaaa\naaaabbaabbaaaaaaabbbabbbaaabbaabaaa\nbabaaabbbaaabaababbaabababaaab\naabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba", (3, 12)),

    ];
    for i in inputs {
        let res = run(i.0.to_owned());
        match res {
            Ok(o) => assert_eq!(o, i.1),
            Err(e) => panic!(e),
        }
    }  
}

struct Rule {
    // Split the rule into (tree[0]) | (tree[1]), where they are parsed as numbers
    tree: Vec<Vec<usize>>,
    // Check to see if we have a letter instead of a tree
    is_end: bool,
    // The letter to use (a|b)
    letter: char,
}

pub fn run(input: String) -> Result<(i64, i64), &'static str> {
    let lines = input.split("\n").collect::<Vec<&_>>();

    // Split the input into the rules lines v the test lines
    let mut rules: &[&str] = &[];
    let mut tests: &[&str] = &[];
    for i in 0..lines.len() {
        let c = lines[i].chars().nth(0).unwrap_or('_');
        if c.to_digit(10).is_none() {
            rules = &lines[0..i];
            tests = &lines[i+1..];
            break;
        }
    } 
    // Set up storage for the rules for index lookup.
    let mut rule_vec: Vec<Rule> = vec![];
    // One of the tests they provide has a rule 42, but not 23-41. Hack for this
    for _ in 0..rules.len().max(43) {   
        rule_vec.push(Rule{tree: vec![], is_end: false, letter: '\0'});
    }
    // Parse each rule line as a Rule, 
    for r in rules {
        // Extract the index.
        let parts = r.split(&[':', '|'][..]).collect::<Vec<&_>>();
        let index = parts[0].parse::<usize>().unwrap();
        
        // Check for a letter with the presence of a quote ("a")
        if parts[1].trim().chars().nth(0).unwrap() == '"' {
            rule_vec[index].is_end = true;
            rule_vec[index].letter = parts[1].trim().chars().nth(1).unwrap();
        } else {
            // Parse the bisections as int vectors, (tree[0]) | (tree[1])
            let mut tree = vec![];
            for p in &parts[1..] {
                let numbers = p.trim().split(" ").collect::<Vec<&_>>().iter().map(|x| x.trim().parse::<usize>().unwrap()).collect::<Vec<usize>>();
                tree.push(numbers);
            }
            rule_vec[index].tree = tree;
        }
    }

    // Loop loop the test strings with the orignal rules and increment the passing ones.
    let mut part1 = 0;
    for test in tests {
        let r = check(test, 0, 0, &rule_vec);
        if r.0 && r.1 == test.len() {
            part1 += 1;
        }
    }
   
    // Skip the test case that doens't have rules 8 and 11, use 43 here as we've previously hacked the min size of the rule list.
    if lines.len() < 43 {
        return Ok((part1 as i64, part1 as i64));
    }
       
    // By changing rule 8 to (42 8) and 11 to (42 31 | 42 11 31) we need to consider repeating patterns
    // As the program is only repeats at 8 or 11 and there is no links between 8 and 11 we can reduce to two patterns 
    // The first is ^<42>{1, } as rule 8 can check rule 42 as many times as possible
    // The second is <31><42>{1, } as 11 is inserted into the middle of the two
    // The rules can be reduced to ^<42>{2,}<31>{1,j}$ Where j < the repetion of <42>
    // Now we cheat and modify the program to check for ^<42>{i}<31>{j}$
    let mut v0 = vec![];   
    for i in 2..30 {
        for j in 1..i {
            let mut v42 = vec![42; i];
            let mut v31 = vec![31; j];
            v42.append(&mut v31);
            v0.push(v42);
        }
    }
    // Replace rule 0 with the modified rule
    rule_vec[0].tree = v0;

    // Check the tests against the new program
    let mut part2 = 0;
    for test in tests {
        let r = check(test, 0, 0, &rule_vec);
        if r.0 {
            part2 += 1;
        }
    }

    return Ok((part1 as i64, part2 as i64));
}

// Traverse the rule tree until we hit a literal in which case check the position against that value.
fn check(test: &str, point: usize, offset: usize, rules: &Vec<Rule>) -> (bool, usize) {
    // Get the Rule for the current rule index.
    let start = &rules[point];
    // If it's an end return success/failure based on letter at offset
    if start.is_end {
        return (test.chars().nth(offset).unwrap_or('\0') == start.letter, offset + 1);
    }

    // Check each of the available paths in the tree
    'p: for path in &start.tree {
        // Loop the sub rules of the path, if they're all successful in order we can increase our offset by the letters read and move on
        let mut new_offset = offset;
        for r in path {
            let t = check(test, *r as usize, new_offset, &rules);
            if !t.0 {
                // The substr check failed, so try the next path
                continue 'p;
            }
            new_offset = t.1;          
        }
        // For the 0 rule we need to make sure that the pass was actually the end of the string, otherwise try the next path
        // The other rules aren't nessisarily the end so just return a sucess and offset change
        if point == 0 {
            if new_offset == test.len() {
                return (true, new_offset);
            }
        } else {
            return (true, new_offset);
        }
    }
    // Return false if no path matches or the length matched for any path in rule 0.
    return (false, 0);
}