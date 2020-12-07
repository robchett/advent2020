#[test]
fn test_run() {
    match run("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc".to_owned()) {
        Ok(i) => assert_eq!(i, (2, 1)),
        Err(e) => panic!(e),
    }
}

pub fn run(input: String) -> Result<(i32, i32), &'static str> {
    let lines = input.split("\n");
    let mut res1 = 0;
    let mut res2 = 0;
    // Split the lines and parse as passwords
    // Count the valid passwords by number of repleating characters
    // Count the valid passwords by characters in the given indexes
    for line in lines {
        let password = PasswordLine::new(line);
        if password.valid_length().is_ok() {
            res1 += 1;
        }
        if password.valid_positions().is_ok() {
            res2 += 1;
        }
    }
    return Ok((res1, res2));
}

struct PasswordLine {
    letter: char,
    min: u32,
    max: u32,
    password: String,
}

impl PasswordLine {
    fn new(line: &str) -> PasswordLine {
        // Setup storage for the members
        let mut letter: char = '_';
        let mut min: u32 = 0;
        let mut max: u32 = 0;
        let mut password: String = "".to_owned();
        // Flags for deciding if we're looking at the min/max, character/password
        let mut is_max = false;
        let mut is_password = false;
        // loop the characters in the line one at a time.
        for c in line.trim().chars() {
            match c {
                // Read the first number as min, the second as max. 
                // Use a boolean (is_max) to decide if we have the first or second.
                // Use base10 multiplication to deal with numbers > 9
                // Improvement: Use a tuple for this?
                '0'..='9' => {
                    if is_max {
                        if max > 0 {
                            max *= 10
                        }
                        max += c.to_digit(10).unwrap_or_default()
                    } else {                        
                        if min > 0 {
                            min *= 10
                        }
                        min += c.to_digit(10).unwrap_or_default()
                    }
                }
                // The first character is the lookup, anything after the first ':' is the password.
                'a'..='z' => {
                    if is_password {
                        password.push(c)
                    } else {
                        letter = c
                    }
                }
                // Switch from min -> max
                '-' => is_max = true,
                // Switch from character -> password
                ':' => is_password = true,
                // Ignore spaces
                ' ' => (),
                // Shouldn't be any other values in the password strings.
                _ => println!("Unexpect character found in line- {}", c),
            }
        }
        // Create a passord struct with the stored values
        return PasswordLine {
            letter,
            min: u32::from(min),
            max: u32::from(max),
            password: password.to_string(),
        };
    }

    // Checks if the number of references to self.letter is between self.min & self.max
    fn valid_length(&self) -> Result<bool, String> {
        let mut matches = 0;
        // See how many times self.letter appears in the string.
        for c in self.password.chars() {
            if c == self.letter {
                matches += 1;
            }
        }

        if matches >= self.min && matches <= self.max {
            return Ok(true);
        } else {
            return Err(format!(
                "{} is not between {} and {}",
                matches, self.min, self.max
            ));
        }
    }

    // Checks if the {min} & {max} characters in self.password are self.letter
    fn valid_positions(&self) -> Result<bool, &str> {
        // Extract the minth and maxth charactrs from the password string
        // Assumes the indexes exist in the string
        // Improvement: Check the string length first.
        let match1 = self.password.chars().nth(self.min as usize - 1).unwrap() == self.letter;
        let match2 = self.password.chars().nth(self.max as usize - 1).unwrap() == self.letter;
        if (match1 || match2) && !(match1 && match2) {
            return Ok(true);
        } else if !match1 && !match2 {
            return Err("neither characters match");
        } else {
            return Err("both characters match");
        }
    }
}

#[test]
fn test_part1_1() {
    let password = PasswordLine::new("1-3 a: abcde");
    assert!(password.valid_length().is_ok());
}
#[test]
fn test_part1_2() {
    let password = PasswordLine::new("1-3 b: cdefg");
    assert!(password.valid_length().is_err());
}

#[test]
fn test_part1_3() {
    let password = PasswordLine::new("2-9 c: ccccccccc");
    assert!(password.valid_length().is_ok());
}

#[test]
fn test_part2_1() {
    let password = PasswordLine::new("1-3 a: abcde");
    assert!(password.valid_positions().is_ok());
}
#[test]
fn test_part3_2() {
    let password = PasswordLine::new("1-3 b: cdefg");
    assert!(password.valid_positions().is_err());
}

#[test]
fn test_part4_3() {
    let password = PasswordLine::new("2-9 c: ccccccccc");
    assert!(password.valid_positions().is_err());
}
