#[test]
fn test_run() {
    let res = run("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\nhcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\nhcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in".to_owned());
    match res {
        Ok(i) => assert_eq!(i, (2, 2)),
        Err(e) => panic!(e),
    }
}

pub fn run(input: String) -> Result<(i64, i64), &'static str> {
    // Each passport is delimited by a blank line.
    let lines = input.split("\n\n");
    let mut passports = Vec::new();
    // Parse each block of lines into a Passport struct
    for line in lines {
        passports.push(Passport::new(line));
    }
    // Setup return counters
    let mut part1 = 0;
    let mut part2 = 0;

    for passport in passports {
        // Increment part 1 counter if the passport has all the required fields
        // Improvement: no need to validate part 2 if this doens't pass, the Passport guarenteed to be invalid
        if passport.byr.0
            && passport.eyr.0
            && passport.iyr.0
            && passport.hgt.0
            && passport.pid.0
            && passport.hcl.0
            && passport.ecl.0
        {
            part1 += 1;
        }
        // Increment part 2 counter if the passport has all the required fields and they are valid
        if passport.valid_byr()
            && passport.valid_eyr()
            && passport.valid_iyr()
            && passport.valid_hgt()
            && passport.valid_pid()
            && passport.valid_hcl()
            && passport.valid_ecl()
        {
            part2 += 1;
        }
    }
    return Ok((part1, part2));
}

// Structure to hold the components, each tuple is (is_present: bool, contents: String)
// Intentionally kept unvalidated as we can use short-circiting to validate the bare minimum fields
struct Passport {
    byr: (bool, String),
    eyr: (bool, String),
    iyr: (bool, String),
    hgt: (bool, String),
    pid: (bool, String),
    hcl: (bool, String),
    ecl: (bool, String),
}

impl Passport {
    // Parses the entry as an int and compares to the
    fn _valid_year(entry: &String, start: i32, end: i32) -> bool {
        let val = entry.parse::<i32>().unwrap_or_default();
        return val >= start && val <= end;
    }
    // Checks the presence of the byr and if it falls in the range
    fn valid_byr(&self) -> bool {
        if !self.byr.0 {
            return false;
        }
        return Passport::_valid_year(&self.byr.1, 1920, 2002);
    }
    // Checks the presence of the iyr and if it falls in the range
    fn valid_iyr(&self) -> bool {
        if !self.iyr.0 {
            return false;
        }
        return Passport::_valid_year(&self.iyr.1, 2010, 2020);
    }
    // Checks the presence of the eyr and if it falls in the range
    fn valid_eyr(&self) -> bool {
        if !self.eyr.0 {
            return false;
        }
        return Passport::_valid_year(&self.eyr.1, 2020, 2030);
    }
    // Checks the presence of the hgt and if it falls in the range
    fn valid_hgt(&self) -> bool {
        if !self.hgt.0 {
            return false;
        }
        // Extract the last 2 characters to decide if comparing as "cm" or "in"
        let (first, last) = self.hgt.1.split_at(self.hgt.1.len() - 2);
        let val = first.parse::<i32>().unwrap_or_default();
        return match last {
            // Compare the heights in the correct units
            "cm" => val >= 150 && val <= 193,
            "in" => val >= 59 && val <= 76,
            // Fail validation if the units were not accounted for
            _ => false,
        };
    }
    // Checks the presence of the hcl and if it falls in the range
    fn valid_hcl(&self) -> bool {
        if !self.hcl.0 {
            return false;
        }
        // HCL must be 7 characters #[a-z0-9]{6}
        let part = &self.hcl.1;
        if part.len() != 7 {
            return false;
        }
        let chars = part.chars();
        for (i, c) in chars.enumerate() {
            match c {
                // Check that # is the first character.
                // Improvement: Cleaner to do this with a slice?
                '#' => {
                    if i != 0 {
                        return false;
                    }
                }
                // Check that index 1-6 are numbers or letters
                'a'..='f' | '0'..='9' => {
                    if i == 0 {
                        return false;
                    }
                }
                // Fail if any other character present
                _ => return false,
            }
        }
        return true;
    }
    // Checks the presence of the ecl and if it is one of the accepted values
    fn valid_ecl(&self) -> bool {
        if !self.ecl.0 {
            return false;
        }
        let part = &self.ecl.1;
        return part == "amb"
            || part == "blu"
            || part == "brn"
            || part == "gry"
            || part == "grn"
            || part == "hzl"
            || part == "oth";
    }
    // Checks the presence of the pid, and if it is a 9 digit number (allowing leading 0s)
    fn valid_pid(&self) -> bool {
        if !self.pid.0 {
            return false;
        }
        let part = &self.pid.1;
        // Fail if not 9 digits
        if part.len() != 9 {
            return false;
        }
        // Fail if can't be parsed as an int
        return part.parse::<i32>().is_ok();
    }

    // Parse lines into a new Passport
    fn new(lines: &str) -> Passport {
        // Setup holders for the values with placeholders
        let mut byr = (false, "".to_owned());
        let mut iyr = (false, "".to_owned());
        let mut eyr = (false, "".to_owned());
        let mut hgt = (false, "".to_owned());
        let mut hcl = (false, "".to_owned());
        let mut ecl = (false, "".to_owned());
        let mut pid = (false, "".to_owned());

        // Split out the parts with any whitespace and collect into a vector
        let sections_vec = lines.split(&[' ', '\n'][..]).collect::<Vec<&_>>();

        // Loop the sections and extract the data
        for section in sections_vec {
            let mut i = 0;
            // data is in the format 'key:value'
            // Impovement: Should check that there were exactly two parts
            let parts = section.split(':');
            let mut section = "";
            // Loop the parts
            // Improvement: if parts length is checked, extract as indexes instead of looping
            for part in parts {
                match i {
                    // The first part is the key
                    0 => {
                        section = part;
                    }
                    // Mark each present section as such and store it's value
                    1 => match section {
                        "byr" => {
                            byr.0 = true;
                            byr.1 = part.to_owned();
                        }
                        "iyr" => {
                            iyr.0 = true;
                            iyr.1 = part.to_owned();
                        }
                        "eyr" => {
                            eyr.0 = true;
                            eyr.1 = part.to_owned();
                        }
                        "hgt" => {
                            hgt.0 = true;
                            hgt.1 = part.to_owned();
                        }
                        "hcl" => {
                            hcl.0 = true;
                            hcl.1 = part.to_owned();
                        }
                        "ecl" => {
                            ecl.0 = true;
                            ecl.1 = part.to_owned();
                        }
                        "pid" => {
                            pid.0 = true;
                            pid.1 = part.to_owned();
                        }
                        _ => {}
                    },
                    _ => {}
                }
                // Increment the index so we know were on the result part
                // Improvement: this isn't very obvious.
                i += 1;
            }
        }

        // Return a populated Passport
        return Passport {
            byr,
            eyr,
            iyr,
            hgt,
            pid,
            hcl,
            ecl,
        };
    }
}
