use crate::solutions::helpers::read_lines;

const SCALE: usize = 13;
const MULTIPLIER: u32 = 13u32.pow(5);

fn map_char_to_rank(value: char) -> u8 {
    match value {
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
        _ => panic!("Invalid character read: {value}"),
    }
}

fn map_char_to_rank_with_joker(value: char) -> u8 {
    match value {
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
        _ => panic!("Invalid character read: {value}"),
    }
}

#[derive(Debug)]
struct Game {
    value: u32,
    bid: u32,
}
impl Game {
    fn new(hand: &str, bid: u32, with_joker: bool) -> Self {
        let mut card_count = [0u8; SCALE];
        let mut value: u32 = 0;
        let mut joker_count: u8 = 0;

        let mapper = if with_joker { map_char_to_rank_with_joker } else { map_char_to_rank };
        // Use bytes to skip mapping numbers
        for rank in hand.chars().map(mapper) {
            // Using a 13 scale
            value *= SCALE as u32;
            if with_joker && rank == 0 {
                joker_count += 1;
            } else {
            value += rank as u32;
            card_count[rank as usize] += 1;
            }
        }

        // Calc max counts
        let (mut first, second) = card_count
            .iter()
            .fold((0u8, 0u8), |(first, second), &count| {
                if count > first {
                    (count, first)
                } else if count > second {
                    (first, count)
                } else {
                    (first, second)
                }
            });

        first += if with_joker { joker_count } else { 0 };
        value += MULTIPLIER * (3 * first + second) as u32;
        Self { value, bid }
    }
}

fn solve(with_joker: bool) {
    let mut games = read_lines("day7.txt")
        .iter()
        .map(|s| {
            let (game, bid) = s.split_once(' ').unwrap();
            Game::new(game, bid.parse().unwrap(), with_joker)
        })
        .collect::<Vec<Game>>();
    games.sort_by_key(|game| game.value);

    println!(
        "{}",
        games
            .iter()
            .enumerate()
            .map(|(i, game)| (i as u32 + 1) * game.bid)
            .sum::<u32>()
    );
}

pub fn solve_a() {
    solve(false);
}

pub fn solve_b() {
    solve(true);
}
