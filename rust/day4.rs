use std::fs;

pub fn run() {
    let contents = fs::read_to_string("../inputs/day4.txt").expect("Input file not found");
    let lines = contents.split("\n\n");
    let mut passports = Vec::new();
    for line in lines {
        passports.push(Passport::new(line));
    }
    println!("Part 1 output: {}", part1(&passports));
    println!("Part 2 output: {}", part2(&passports));
}

struct Passport {
    byr: (bool, bool),
    eyr: (bool, bool),
    iyr: (bool, bool),
    hgt: (bool, bool),
    pid: (bool, bool),
    hcl: (bool, bool),
    ecl: (bool, bool),
}

impl Passport {
    fn new(lines: &str) -> Passport {
        let mut byr = (false, false);
        let mut iyr = (false, false);
        let mut eyr = (false, false);
        let mut hgt = (false, false);
        let mut hcl = (false, false);
        let mut ecl = (false, false);
        let mut pid = (false, false);
        let sections = lines.split(&[' ', '\n'][..]);
        let sections_vec = sections.collect::<Vec<&_>>();
   
        for section in sections_vec {
            let mut i = 0;
            let parts = section.split(':');
            let mut section = "";
            for part in parts {
                match i {
                    0 => {
                        section = part;
                        match part {
                            "byr" => { byr.0 = true; },
                            "iyr" => { iyr.0 = true; },
                            "eyr" => { eyr.0 = true; },
                            "hgt" => { hgt.0 = true; },
                            "hcl" => { hcl.0 = true; },
                            "ecl" => { ecl.0 = true; },
                            "pid" => { pid.0 = true; },
                            _ => {}
                        }
                    }
                    1 => {
                        match section {
                            "byr" => { 
                                let val = part.parse::<i32>().unwrap_or_default(); 
                                byr.1 = val >= 1920 && val <= 2002;
                            }
                            "iyr" => { 
                                let val = part.parse::<i32>().unwrap_or_default();
                                iyr.1 = val >= 2010 && val <= 2020;
                            },
                            "eyr" => { 
                                let val = part.parse::<i32>().unwrap_or_default();
                                eyr.1 = val >= 2020 && val <= 2030 ;
                            },
                            "hgt" => { 
                                let (first, last) = part.split_at(part.len() - 2);
                                let val = first.parse::<i32>().unwrap_or_default(); 
                                hgt.1 = match last {
                                    "cm" => val >= 150 && val <= 193,
                                    "in" => val >= 59 && val <= 76,
                                    _ => false
                                }
                            },
                            "hcl" => { 
                                if part.len() != 7 {
                                    continue;
                                }
                                let chars = part.chars();
                                for (i, c) in chars.enumerate() {
                                    match c {
                                        '#' => if i != 0 { continue; },
                                        'a'..='f' => if i == 0 { continue; },
                                        '0'..='9' => if i == 0 { continue; },
                                        _ => { continue }
                                    }
                                }
                                hcl.1 = true;
                            },
                            "ecl" => {
                                ecl.1 = part == "amb" || part == "blu" || part == "brn" || part == "gry" || part == "grn" || part == "hzl" || part == "oth" 
                            },
                            "pid" => { 
                                if part.len() != 9 {
                                    continue;
                                }
                                let parsed = part.parse::<i32>();
                                match parsed {
                                    Ok(_v) => pid.1 = true,
                                    Err(_e) => {},
                                }
                            },
                            _ => {}
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
        }
        return Passport {byr, eyr, iyr, hgt, pid, hcl, ecl};
    }
}


fn part1(passports: &Vec<Passport>) -> i32 {
    let mut count = 0;
    for passport in passports {
        if passport.byr.0 && passport.eyr.0 && passport.iyr.0 && passport.hgt.0 && passport.pid.0 && passport.hcl.0 && passport.ecl.0 {
            count += 1;
        }
    }
    return count;
}

fn part2(passports: &Vec<Passport>) -> i32 {
    let mut count = 0;
    for passport in passports {
        if passport.byr.1 && passport.eyr.1 && passport.iyr.1 && passport.hgt.1 && passport.pid.1 && passport.hcl.1 && passport.ecl.1 {
            count += 1;
        }
    }
    return count;
}