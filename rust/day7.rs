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

pub fn run(input: String) -> Result<(i32, i32), &'static str> {
    let clean_input = input.replace(".", "");
    let lines = clean_input.split("\n");
    let mut bags = HashMap::new();
    for line in lines {
        let mut split = line.split("s contain ").collect::<Vec<&str>>();

        if split.len() >= 2 {
            let subs = split.pop().unwrap().split(", ").collect::<Vec<&str>>();
            let mut sub_tuples = vec![];
            let mut sub_count = 1;
            for sub in subs {
                let s = &sub[0..1];
                let si = s.parse::<i32>();
                if si.is_ok() {
                    let sj = si.unwrap();
                    let offset;
                    if sj > 1 {
                        offset = sub.len() - 1;
                    } else {
                        offset = sub.len();
                    }
                    let e = &sub[2..offset];
                    sub_tuples.push((e, sj));
                    sub_count = -1;
                } else {
                    sub_tuples.push(("", 0));
                }
            }
            let key = split.pop().unwrap();
            bags.insert(key, (sub_tuples, sub_count));
        }
    }

    let mut stack = vec!["shiny gold bag"];
    let mut seen = vec![];
    while stack.len() > 0 {
        let curr = stack.pop().unwrap();
        for (&key, val) in bags.iter() {
            if val.0.iter().find(|x| x.0 == curr).is_some() {
                if !seen.contains(&key) && !stack.contains(&key) {
                    stack.push(&key)
                }
            }
        }
        seen.push(&curr);
    }

    let bag_copy = bags.clone();
    let keys = bag_copy.keys();
    let mut to_calc = keys.filter(|x| bags.get(*x).unwrap().1 < 0).collect::<Vec<&&str>>();
    while to_calc.len() > 0 {
        for k in to_calc {
            let b = bags.get(k).unwrap();
            let computed_children = b.0.iter().filter(|x| bags.get(x.0).unwrap().1 >= 0).collect::<Vec<&(&str, i32)>>();
            if computed_children.len() == b.0.len() {
                let mut count = 1 as i64;
                for c in computed_children {
                    let d = bags.get(c.0).unwrap();
                    count += d.1 * c.1 as i64;
                }
                let update = bags.get_mut(k).unwrap();
                update.1 = count;
            }
        }
        let keys = bag_copy.keys();
        to_calc = keys.filter(|x| bags.get(*x).unwrap().1 < 0).collect::<Vec<&&str>>();
    }
    
    
    println!("{}", bags.get("shiny gold bag").unwrap().1 - 1);
    return Ok((seen.len() as i32 - 1, (bags.get("shiny gold bag").unwrap().1 - 1) as i32));
}