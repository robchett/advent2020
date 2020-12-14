use std::collections::HashMap;

#[test]
fn test_run() {
    let res = run("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0".to_owned());
    match res {
        Ok(i) => assert_eq!(i.0, 165),
        Err(e) => panic!(e),
    }
    let res = run("mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1".to_owned());
    match res {
        Ok(i) => assert_eq!(i.1, 208),
        Err(e) => panic!(e),
    }
   
}

pub fn run(input: String) -> Result<(i64, i64), &'static str> {
    let lines = input.split("\n");
    // Use hashmaps to store the memory addresses
    let mut map_1:HashMap<u64, u64> = HashMap::new();
    let mut map_2:HashMap<u64, u64> = HashMap::new();
    // Masks for the positions of 0,1 and x
    let mut mask_0 = 0;
    let mut mask_1 = 0;
    let mut mask_x = 0;
    // Storage for the positions that can be 1 or 0 in part 2
    let mut switches = vec![];

    for line in lines {
        // Check the first 3 chars of the line to see if it's a mask or mem address
        match &line[0..3] {
            "mas" => {
                // Reset the masks and switches to 0
                mask_0 = 0;
                mask_1 = 0;
                mask_x = 0;
                switches = vec![];
                // Extract the mask string and flip it
                let slice = line[7..43].chars();
                for (i,c) in slice.rev().enumerate() {
                    // The 0 mask is used t 
                    // The 1 mask is the values that are 1, everything else is 0'd out so it doesn't effect the OR
                    // The X mask is used to 0 out anything that isn't a 1 or 0, we want to extract the contant value
                    match c {
                        'X' => {
                            mask_0 |= (2 as u64).pow(i as u32);
                            switches.push((2 as u64).pow(i as u32));
                        },
                        '1' => {
                            mask_1 |= (2 as u64).pow(i as u32);
                            mask_x |= (2 as u64).pow(i as u32);
                        },
                        '0' => {
                            mask_x |= (2 as u64).pow(i as u32);
                        },
                        _ => panic!("Unknown character {} in line {}: {}", c, line, i),
                    }
                }
            } 
            "mem" => {
                // Extract the address and value from the mem command
                let parts = line.split(" = ").collect::<Vec<&str>>();                
                let addre = parts.get(0).unwrap();
                let addr =  addre[4..addre.len()-1].parse::<u64>().unwrap();
                let value = parts.get(1).unwrap().parse::<u64>().unwrap();

                // Mask the value, forcing 1 and 0 values from the mask.
                let masked_value = (value & mask_0) | mask_1;
                map_1.insert(addr, masked_value);

                // The first text has a lot of x's which causes it to write 2^32 addresses, don't let it do that.
                if switches.len() < 10 {
                    // Extract the address that isn't masked by an X and force 1's
                    let masked_addr = (addr | mask_1) & mask_x;     
                    // Generate a vector of all the combinations of X values and add then to the masked_add
                    let summed_switches = add_switches(&switches);      
                    for i in summed_switches {
                        map_2.insert(masked_addr + i, value);
                    }  
                } else {
                    // To many switches to compute quickly.
                }
            }
            _ => panic!("Unknown line format {}", line)
        }
    }

    let values_1 = map_1.values().fold(0, |acc, x| acc + x);
    let values_2 = map_2.values().fold(0, |acc, x| acc + x);

    return Ok((values_1 as i64, values_2 as i64));
}

// Compute each combination of X's
// Add x or 0 to each entry of the shorter list
// If the list is 1 entry return [x, 0]
fn add_switches(switches: &Vec<u64>) -> Vec<u64> {
    if switches.len() == 1 {
        return vec![switches[0], 0];
    }
    let f = switches[0];
    let less = add_switches(&switches[1..(switches.len())].to_vec());
    let mut out = vec![];
    for i in less {
        out.push(i);
        out.push(i + f);
    }
    return out;
}
