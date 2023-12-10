use std::cmp::Ordering;

fn char_to_idx(ch: char) -> usize {
    match ch {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("bad"),
    }
}

fn char_to_idx_p2(ch: char) -> usize {
    match ch {
        'J' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("bad"),
    }
}

fn get_type(hand: &[u8; 13]) -> HandType {
    match hand.iter().max().unwrap() {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => {
            // three of a kind or full house
            if hand.iter().any(|num| *num == 2) {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        }
        2 => {
            // one or two pair
            if hand.iter().filter(|num| **num == 2).count() == 2 {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        }
        1 | 0 => HandType::HighCard,
        _ => panic!("{hand:#?}"),
    }
}

fn score_hand(hand: &[u8; 13], orig_hand: &[usize; 5]) -> u64 {
    if hand.iter().sum::<u8>() != 5 {
        return 0;
    }

    let mut score = (7 - get_type(hand) as u64) * 1_00_00_00_00_00;

    let mut multiplier = 1u64;
    for card in orig_hand.iter().rev() {
        score += *card as u64 * multiplier;
        multiplier *= 100;
    }

    score
}

fn recurse_fill_hand(
    orig_hand: &[usize; 5],
    starting_hand: &[u8; 13],
    (max_score, max_hand): (&mut u64, &mut [u8; 13]),
) {
    // dbg!(starting_hand);
    let non_wilds_in_hand: u8 = starting_hand.iter().sum();
    // base case
    if non_wilds_in_hand >= 5 {
        return;
    } else if non_wilds_in_hand == 0 {
        // special case: all wilds
        *max_hand = [0; 13];
        max_hand[12] = 5;
        *max_score = score_hand(&max_hand, &orig_hand);
        return;
    }

    // Recurse on all possible placements
    // All possible placements are slots that already have a card
    // except in the all-wilds case, which is handled above
    for possible_idx in 0..starting_hand.len() {
        if starting_hand[possible_idx] != 0 {
            let mut cloned_hand = starting_hand.clone();
            cloned_hand[possible_idx] += 1;

            let score = score_hand(&cloned_hand, orig_hand);
            if score > *max_score {
                *max_score = score;
                *max_hand = cloned_hand;
            }

            recurse_fill_hand(orig_hand, &cloned_hand, (max_score, max_hand));
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand1 {
    ranks: [u8; 13], // in order of strength, low strength to high
    orig_hand: [usize; 5],
}
impl Hand1 {
    fn new(hand_chars: impl Iterator<Item = char>) -> Self {
        let mut ranks = [0; 13];
        let mut orig_hand = [100; 5];

        for (i, card) in hand_chars.enumerate() {
            let idx = char_to_idx(card);
            orig_hand[i] = idx;
            ranks[idx] += 1;
        }

        assert_eq!(ranks.iter().cloned().sum::<u8>(), 5);
        Self { ranks, orig_hand }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand2 {
    orig_hand_p2: [usize; 5],
    max_hand: [u8; 13],
}
impl Hand2 {
    fn new(hand_chars: impl Iterator<Item = char>) -> Self {
        let mut ranks_p2 = [0; 13];
        let mut orig_hand_p2 = [101; 5];

        for (i, card) in hand_chars.enumerate() {
            let idx2 = char_to_idx_p2(card);
            orig_hand_p2[i] = idx2;
            ranks_p2[idx2] += 1;
        }

        ranks_p2[0] = 0;
        let mut max_score = score_hand(&ranks_p2, &orig_hand_p2);
        let mut max_hand = ranks_p2.clone();
        recurse_fill_hand(&orig_hand_p2, &ranks_p2, (&mut max_score, &mut max_hand));

        assert!(ranks_p2.iter().cloned().sum::<u8>() <= 5);
        Self {
            orig_hand_p2,
            max_hand,
        }
    }
}

impl PartialOrd for Hand1 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match get_type(&self.ranks).cmp(&get_type(&other.ranks)) {
            Ordering::Equal => Some(self.orig_hand.cmp(&other.orig_hand).reverse()),
            ord => Some(ord),
        }
    }
}
impl Ord for Hand1 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match get_type(&self.max_hand).cmp(&get_type(&other.max_hand)) {
            Ordering::Equal => Some(self.orig_hand_p2.cmp(&other.orig_hand_p2).reverse()),
            ord => Some(ord),
        }
    }
}
impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,  // 7
    FourOfAKind,  // 6
    FullHouse,    // 5
    ThreeOfAKind, // 4
    TwoPair,      // 3
    OnePair,      // 2
    HighCard,     // 1
}

fn main() {
    let mut hands_and_bids: Vec<(Hand1, Hand2, u32)> = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_at(line.find(' ').unwrap());
            let hand1 = Hand1::new(hand.chars());
            let hand2 = Hand2::new(hand.chars());
            (hand1, hand2, bid.trim().parse().unwrap())
        })
        .collect();

    hands_and_bids.sort_by(|(h11, _, _), (h12, _, _)| h11.cmp(h12));

    let result1: u32 = hands_and_bids
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, (_, _, bid))| (idx as u32 + 1) * bid)
        .sum();

    println!("{result1}");

    hands_and_bids.sort_by(|(_, h21, _), (_, h22, _)| h21.cmp(h22));

    let result2: u32 = hands_and_bids
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, (_, _, bid))| (idx as u32 + 1) * bid)
        .sum();

    println!("{result2}");
}
