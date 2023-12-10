use std::collections::{HashMap, HashSet};

fn main() {
    let cards: Vec<usize> = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|card| {
            let (_, card) = card.split_at(card.find(":").unwrap());
            let card = &card[1..];
            let (winners, numbers) = card.split_at(card.find("|").unwrap());
            let winners = winners
                .trim()
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<HashSet<usize>>();

            let numbers = &numbers[1..]
                .trim()
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<HashSet<usize>>();

            winners.intersection(&numbers).count()
        })
        .collect();

    println!(
        "{}",
        cards
            .iter()
            .map(|num_matches| if *num_matches == 0 {
                0
            } else {
                2usize.pow((num_matches - 1) as u32)
            })
            .sum::<usize>()
    );

    let mut multipliers = HashMap::new();
    multipliers.insert(1, 1u32);

    let mut result2 = 0u32;
    for (idx, num_matches) in cards.into_iter().enumerate() {
        let idx = idx + 1;
        let current_multiplier = *multipliers.entry(idx).or_insert(1);

        result2 += current_multiplier;
        for i in 1..num_matches + 1 {
            *multipliers.entry(idx + i).or_insert(1) += current_multiplier;
        }
    }

    println!("{result2}");
}
