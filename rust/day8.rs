#[test]
fn test_run() {
    let res =
        run("nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6".to_owned());
    match res {
        Ok(i) => assert_eq!(i, (5, 8)),
        Err(e) => panic!(e),
    }
}

pub fn run(input: String) -> Result<(i64, i64), &'static str> {
    let lines = input.split("\n");
    // Create a new state machine
    let mut machine = StateMachine {
        pos: 0,
        acc: 0,
        commands: vec![],
        seen: vec![],
    };
    // Loop and parse the commands in the file
    for line in lines {
        // Split {type: string} {value: usize}
        let c = line.split(" ").collect::<Vec<&_>>();
        if c.len() != 2 {
            panic!("Failed to parse input: {}", line);
        }
        let size;
        let size_res = c.get(1).unwrap().parse::<isize>();
        match size_res {
            Ok(v) => size = v,
            Err(_) => panic!("Failed to parse int: {}", c.get(1).unwrap()),
        }
        // Create a command object from each line
        let command = c.get(0).unwrap();
        match *command {
            "nop" => machine.commands.push(Command {
                inst: CommandType::Nop,
                size,
            }),
            "acc" => machine.commands.push(Command {
                inst: CommandType::Acc,
                size,
            }),
            "jmp" => machine.commands.push(Command {
                inst: CommandType::Jmp,
                size,
            }),
            _ => panic!("Unknown command: {}", command),
        }
    }

    // Run the machine until it loops
    // Assumes it loops without making any changes
    let part1 = machine.run().0;
    let mut part2 = 0;

    // Set up storage for commands that could be changed without entering an existing loop.
    // If we hit a command that was part of the existing loop we cannot reach the end
    // It will either enter the existing loop or rehit the changed value (which is a new loop)
    let mut changeable = vec![];

    for i in machine.seen.iter() {
        let c = machine.commands.get(*i).unwrap();
        let next;
        match c.inst {
            CommandType::Nop => next = (*i as isize).checked_add(c.size).unwrap() as usize,
            CommandType::Jmp => next = *i + 1,
            _ => continue,
        }
        if !machine.seen.contains(&next) {
            changeable.push(*i);
        }
    }

    // For each of the changeable values re run the program
    // Improvement: Store the acc at each command so we can resume from that command
    for i in changeable {
        // Reset the machine to it's base state
        machine.reset();
        // Change the command at index i to the complementary type
        machine.change_command(i);
        // Run the machine to completion/loop
        // If the machine ends naturally we have our result
        let res = machine.run();
        match res.1 {
            StateMachineEnd::End => {
                part2 = res.0;
                break;
            }
            _ => {}
        }
        // Revert the change to the command
        machine.change_command(i);
    }

    return Ok((part1 as i64, part2 as i64));
}

// Enum of the available command type
enum CommandType {
    Nop, // No operation
    Acc, // Accumilate
    Jmp, // Jump
}

struct Command {
    inst: CommandType,
    size: isize, // Size of jump or accumilate
}

struct StateMachine {
    pos: usize,             // Current command line index
    acc: isize,             // The accumilator
    seen: Vec<usize>,       // List of seen command lines for detecting a loop
    commands: Vec<Command>, // The program
}

enum StateMachineEnd {
    Loop, // The machine revisited a seen command
    End,  // The machine reached the last line of the program
}

impl StateMachine {
    // Run the machine to loop/completion
    fn run(&mut self) -> (i32, StateMachineEnd) {
        loop {
            // Add the current command to the seen list
            self.seen.push(self.pos);
            let command = self.commands.get(self.pos);
            match command {
                Some(c) => match c.inst {
                    // Move to the next position
                    CommandType::Nop => self.pos += 1,
                    // Add to accumilator and move to the next position
                    CommandType::Acc => {
                        self.acc += c.size;
                        self.pos += 1
                    }
                    // Move to the command indicated by size relative to the current position
                    CommandType::Jmp => {
                        self.pos = (self.pos as isize).checked_add(c.size).unwrap() as usize
                    }
                },
                None => panic!("Command out out range: {}", self.pos),
            }
            // If we're at the last command end with End
            if self.pos == self.commands.len() {
                return (self.acc as i32, StateMachineEnd::End);
            }
            // If we've been here before, end with Loop
            if self.seen.contains(&self.pos) {
                return (self.acc as i32, StateMachineEnd::Loop);
            }
        }
    }

    // Reset the default values
    fn reset(&mut self) {
        self.pos = 0;
        self.acc = 0;
        self.seen = vec![];
    }

    // Swap the NOP/JMP at index to the complement
    fn change_command(&mut self, index: usize) {
        let command = self.commands.get_mut(index).unwrap();
        match command.inst {
            CommandType::Nop => command.inst = CommandType::Jmp,
            CommandType::Jmp => command.inst = CommandType::Nop,
            _ => {}
        }
    }
}
