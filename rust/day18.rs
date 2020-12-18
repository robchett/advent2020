#[test]
fn test_run_18() {
    // Just visualising the steps for part 2,
    // let v = ((((2 + 4) * 9) * (((6 + 9) * (8 + 6)) + 6)) + 2 + 4 ) * 2;
    // let v = (((   6    * 9) * ((  15    *    14  ) + 6)) + 2 + 4 ) * 2;
    // let v = ((      54      * (              210   + 6)) + 2 + 4 ) * 2;
    // let v = ((      54      *               216       )  + 2 + 4 ) * 2;
    // let v = (                  11664                     + 2 + 4 ) * 2;
    // let v =                    11670                               * 2;
    // let v =                    23340                                  ;
    let inputs = vec![
        ("1 + (2 * 3) + (4 * (5 + 6))", (51, 51)),
        ("2 * 3 + (4 * 5)", (26, 46)),
        ("5 + (8 * 3 + 9 + 3 * 4 * 3)", (437, 1445)),
        ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", (12240, 669060)),
        ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", (13632, 23340)),
    ];
    for i in inputs {
        let res = run(i.0.to_owned());
        match res {
            Ok(o) => assert_eq!(o, i.1),
            Err(e) => panic!(e),
        }
    }  
}

// Different operators
#[derive(PartialEq)]
#[derive(Debug)]
enum Operator {
    Multiplication,
    Addition
}

pub fn run(input: String) -> Result<(i64, i64), &'static str> {
    // Split the calculations by line and set up the sum outputs.
    let lines = input.split("\n");
    let mut part1 = 0;
    let mut part2 = 0;
    // Loop each line
    for line in lines {
        part1 += compute(line, false);
        part2 += compute(line, true);        
    }
    return Ok((part1 as i64, part2 as i64));
}

macro_rules! collapse_stack {
    ($stack:tt, $index:tt) => {
        match $stack[$index - 1].2 {
            Operator::Addition => {
                $stack[$index - 1].0 += $stack[$index].0
            },
            Operator::Multiplication => {
                $stack[$index - 1].0 *= $stack[$index].0
            },
        }                                 
        $index -= 1;
    };
}

fn compute(line: &str, virtual_scopes: bool) -> i64 {
    // Set up a stack of previous value + previous operator
    // We'll form a new scope every time we see a ( and close the scope when we see a )
    // Initalise 20 scope, that's more depth than we need.

    // Part 2 is more interesting. To do this we a creating virtual scopes when we see a *
    // These scopes are closed when we hit the next )

    let mut stack: Vec<(u64, bool, Operator)> = vec![];
    for _ in 0..20 {
        stack.push((0, false, Operator::Addition));
    }
    let mut index = 0;
    for c in line.chars() {
        match c {
            // Using the last operator in this scope, modify the scopes value
            '0'..='9' => { 
                let i = c.to_digit(10).unwrap() as u64;
                match stack[index].2 {
                    Operator::Addition => {
                        stack[index].0 += i
                    },
                    Operator::Multiplication => {
                        stack[index].0 *= i
                    },
                }
            },
            // Create a new standard scope.
            // Start with 0 + ...
            '(' => {
                index += 1;
                stack[index] = (0, false, Operator::Addition);
            },
            // Close the current scope.
            // If we have virtual scopes open, close them first.
            ')' => {   
                while stack[index].1 {
                    collapse_stack!(stack, index);                   
                }
                collapse_stack!(stack, index);        
            },
            // Change the last operator to addition
            '+' => {
                stack[index].2 = Operator::Addition;
            },
            // Change the last operatior to multiplication.
            // If we're using virtual stacks, then create a new one.
            '*' => {
                stack[index].2 = Operator::Multiplication;
                if virtual_scopes {
                index += 1;
                    stack[index] = (0, true, Operator::Addition);
                }
            },
            ' ' => {},
            _ => panic!("Unknown char {}", c)
        }
    }

    // Close any remaining virtual stacks
    while stack[index].1 {
        collapse_stack!(stack, index);        
    }


    return stack[0].0 as i64;
}