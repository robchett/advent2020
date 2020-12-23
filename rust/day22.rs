use std::collections::VecDeque;

#[test]
fn test_run_22() {
    let inputs = vec![
        ("Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10", (306, 291)),       
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
    let lines = input.split("\n");
    // Set up the two decks as queues and parse the inputs as the two decks
    let mut decks:(VecDeque<_>, VecDeque<_>) = (VecDeque::new(), VecDeque::new());    
    let mut player = 0;
    for line in lines {
        if line.len() == 0 {
            // Skip empty lines
            continue;
        } else if line.chars().nth(0).unwrap() == 'P' {
            // If the line is a new player, increment the player count
            player += 1;
        } else if player == 1 {
            // Push the card onto player 1 deck
            decks.0.push_back(line.parse::<u32>().unwrap());
        } else {
            // Push the card only player 2 deck
            decks.1.push_back(line.parse::<u32>().unwrap());
        }
    }

    // Copy the decks and run the simulation without recursion
    // Return the score of the winning deck
    let mut part1_deck = decks.to_owned();
    let part1 = score_deck(if play_deck(&mut part1_deck, false) { &part1_deck.0 } else { &part1_deck.1 });
    
    // Copy the decks and run the simulation with recursion
    // Return the score of the winning deck
    let mut part2_deck = decks.to_owned();
    let part2 = score_deck(if play_deck(&mut part2_deck, true) { &part2_deck.0 } else { &part2_deck.1 });

    return Ok((part1 as i64, part2 as i64));
}

// Calcuate the score of the deck, Sum(index * card)
fn score_deck(deck: &VecDeque<u32>) -> u32 {
    let mut score = 0;
    for (i,c) in deck.iter().rev().enumerate() {
        score += (i+1) as u32 * c
    }
    return score;
}

// Extract a subset of cards length {size} as a new queue
fn extract_slice(queue: &mut VecDeque<u32>, size: u32) -> VecDeque<u32> {
    let mut new = VecDeque::new();
    for _ in 0..size {
        new.push_back(queue.pop_front().unwrap());
    }
    return new;
}

// Plays out the hands until winner is found
// Returns true for player 1 winning, false for player 2.
fn play_deck(decks: &mut (VecDeque<u32>, VecDeque<u32>), recursive: bool) -> bool {
    // Keep track of seen hands, player 1 wins if a hand repeats.
    // Keep the score in memory instead of the actuall hands
    let mut seen_rounds = vec![];
    // Keep looping until someone has no cards left
    while decks.0.len() > 0 && decks.1.len() > 0 {
        // Check if the hands have been seen before, or add them to the history.
        let scores = (score_deck(&decks.0), score_deck(&decks.1));
        if seen_rounds.contains(&scores) {
            return true;
        }
        seen_rounds.push(scores);

        // Reveal the top card from each deck
        let cards = (decks.0.pop_front().unwrap(), decks.1.pop_front().unwrap());

        // If we're in recursive mode and there are enough cards to recurse, do that, the winning hand is the winning recursive game
        // Otherwise the winning hand is the one with the higher card
        let player_1_wins = if  recursive && decks.0.len() >= cards.0 as usize && decks.1.len() >= cards.1 as usize {
            play_deck(&mut (extract_slice(&mut decks.0.to_owned(), cards.0), extract_slice(&mut  decks.1.to_owned(), cards.1)), true)
        } else {
            cards.0 > cards.1
        };
        // Push the two cards onto the winners deck, winning card first.
        if player_1_wins {
            decks.0.push_back(cards.0);
            decks.0.push_back(cards.1);
        } else {
            decks.1.push_back(cards.1);
            decks.1.push_back(cards.0);  
        }
    }

    // Return true for player 1, false for player 2
    return if decks.0.len() > 0 { true } else { false };
}