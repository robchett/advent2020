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
    let mut decks:(VecDeque<_>, VecDeque<_>) = (VecDeque::new(), VecDeque::new());
    let mut player = 0;

    for line in lines {
        if line.len() == 0 {
            continue;
        } else if line.chars().nth(0).unwrap() == 'P' {
            player += 1;
        } else if player == 1 {
            decks.0.push_back(line.parse::<u32>().unwrap());
        } else {
            decks.1.push_back(line.parse::<u32>().unwrap());
        }
    }

    let mut part1_deck = decks.to_owned();
    let part1 = score_deck(if play_deck(&mut part1_deck) { &part1_deck.0 } else { &part1_deck.1 });


    let mut part2_deck = decks.to_owned();
    let part2 = score_deck(if play_deck_recursive(&mut part2_deck) { &part2_deck.0 } else { &part2_deck.1 });

    return Ok((part1 as i64, part2 as i64));
}

fn score_deck(deck: &VecDeque<u32>) -> u32 {
    let mut score = 0;
    for (i,c) in deck.iter().rev().enumerate() {
        score += (i+1) as u32 * c
    }
    return score;
}

fn play_deck(decks: &mut (VecDeque<u32>, VecDeque<u32>)) -> bool {
    while decks.0.len() > 0 && decks.1.len() > 0 {
        let cards = (decks.0.pop_front().unwrap(), decks.1.pop_front().unwrap());
        if cards.0 > cards.1 {
            decks.0.push_back(cards.0);
            decks.0.push_back(cards.1);
        } else {
            decks.1.push_back(cards.1);
            decks.1.push_back(cards.0);  
        }
    }

    return if decks.0.len() > 0 { true } else { false };
}

fn extract_slice(queue: &mut VecDeque<u32>, size: u32) -> VecDeque<u32> {
    let mut new = VecDeque::new();
    for _ in 0..size {
        new.push_back(queue.pop_front().unwrap());
    }
    return new;
}



fn play_deck_recursive(decks: &mut (VecDeque<u32>, VecDeque<u32>)) -> bool {
    let mut seen_rounds = vec![];
    while decks.0.len() > 0 && decks.1.len() > 0 {
        let scores = (score_deck(&decks.0), score_deck(&decks.1));
        if seen_rounds.contains(&scores) {
            return true;
        }
        seen_rounds.push(scores);
        let cards = (decks.0.pop_front().unwrap(), decks.1.pop_front().unwrap());
        if decks.0.len() >= cards.0 as usize && decks.1.len() >= cards.1 as usize {
            if play_deck_recursive(&mut (extract_slice(&mut decks.0.to_owned(), cards.0), extract_slice(&mut  decks.1.to_owned(), cards.1))) {
                decks.0.push_back(cards.0);
                decks.0.push_back(cards.1);
            } else {
                decks.1.push_back(cards.1);
                decks.1.push_back(cards.0);  
            }
        } else if cards.0 > cards.1 {
            decks.0.push_back(cards.0);
            decks.0.push_back(cards.1);
        } else {
            decks.1.push_back(cards.1);
            decks.1.push_back(cards.0);  
        }
    }

    return if decks.0.len() > 0 { true } else { false };
}