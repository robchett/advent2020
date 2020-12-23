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

    let mut part1 = "0".to_owned();
    let cups1 = calculate_cups(input.to_owned(), 9, 100);
    let mut prev = 1;
    loop {
        let next = cups1[prev];        
        if next == 1 {
            break;
        }
        part1 = format!("{}{}", part1, next);
        prev = cups1[prev];
        
    }

    let cups2 = calculate_cups(input.to_owned(), 1_000_000, 10_000_000);


    return Ok((part1.parse::<i64>().unwrap(), (cups2[1] * cups2[cups2[1]]) as i64));
}

fn calculate_cups(input: String, size: usize, count: usize) -> Vec<usize> {
    let mut cups = vec![0; size + 1];
    let digits = input.chars().collect::<Vec<char>>().iter().map(|x| x.to_digit(10).unwrap() as usize).collect::<Vec<usize>>();
    let mut cursor = digits.first().unwrap().to_owned();
    let mut prev = digits.first().unwrap().to_owned();
    for d in 1..digits.len() {
        cups[prev] = digits[d];
        prev = digits[d];
    }
    for d in (digits.len()+1)..size+1 {
        cups[prev] = d;
        prev = d;
    }
    cups[prev] = cursor;

    for i in 0..count {   
        let mut dest = cursor;
        let pop = cups[cursor];
        let popped = vec![cups[cursor], cups[cups[cursor]], cups[cups[cups[cursor]]]];

        loop {
            dest -= 1;
            if dest == 0 {
                dest = size;
            }
            if !popped.contains(&dest) {
                break;
            }
        }
        let cursor5 = cups[cups[cups[cups[cursor]]]];
        let cursor4 = cups[cups[cups[cursor]]];
        let cursor2 = cups[cursor];
        let dest1 = cups[dest];

        cups[cursor]  = cursor5;
        cups[cursor4] = dest1;
        cups[dest]    = cursor2;
        cursor = cursor5
    }
    return cups;
}