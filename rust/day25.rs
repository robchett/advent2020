use std::collections::hash_map::{HashMap};

#[test]
fn test_run_25() {
    let inputs = vec![
        (5764801, 8),       
        (17807724, 11),       
    ];
    for i in inputs {
        let res = decrypt(i.0);
        assert_eq!(res, i.1);
    }  
    let inputs = vec![
        ((17807724, 8), 14897079),       
        ((5764801, 11), 14897079),       
    ];
    for i in inputs {
        let res = encrypt(i.0.0, i.0.1);
        assert_eq!(res, i.1);
    }  
}

fn encrypt(input: u64, loop_size: u64) -> u64 {
    let mut key = input;
    for _ in 0..loop_size-1 {
        key = key * input % 20201227
    }
    return key;
}

fn decrypt(input: u64) -> u64 {
    let mut key = 1;
    let mut loops = 0;
    while key != input {
        key = key * 7 % 20201227;
        loops += 1;
    }
    return loops;
}

pub fn run(input: String) -> Result<(i64, i64), &'static str> {
    let keys = input.split("\n").collect::<Vec<&_>>().iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let loop_size = decrypt(keys[0]);
    let part1 = encrypt(keys[1], loop_size);
    return Ok((part1 as i64, 0 as i64));

}