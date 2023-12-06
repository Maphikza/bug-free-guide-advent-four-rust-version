use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn count_matches(winning_numbers: &HashSet<&str>, your_numbers: &HashSet<&str>) -> usize {
    winning_numbers.intersection(your_numbers).count()
}

fn main() -> io::Result<()> {
    let start = Instant::now();

    let path = Path::new("lotto.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let lotto_cards: Vec<String> = reader.lines().filter_map(io::Result::ok).collect();
    let mut total_cards = 0;
    let mut card_copies = vec![1; lotto_cards.len()]; // Start with one copy of each card

    for (index, card) in lotto_cards.iter().enumerate() {
        let parts: Vec<&str> = card.split('|').collect();
        let winning_numbers: HashSet<&str> = parts[0].trim().split_whitespace().collect();
        let your_numbers: HashSet<&str> = parts[1].trim().split_whitespace().collect();

        let match_count = count_matches(&winning_numbers, &your_numbers);
        total_cards += card_copies[index];

        for next_index in index + 1..index + 1 + match_count {
            if next_index < lotto_cards.len() {
                card_copies[next_index] += card_copies[index];
            }
        }
    }

    let duration = start.elapsed();
    println!("Total cards: {}", total_cards);
    println!("Time elapsed: {:?}", duration);

    Ok(())
}






