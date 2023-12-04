use std::{time::Instant, str::FromStr, num::ParseIntError};
use y23::{args, result};
use std::collections::HashSet;

const BIN: &str = env!("CARGO_BIN_NAME");

struct Card {
    id: usize,
    winning_numbers: HashSet<u32>,
    numbers: HashSet<u32>
}

impl FromStr for Card {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split(" ").filter(|splitted| !splitted.is_empty());
        let id = splitted.nth(1).unwrap().chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<usize>()?;
        let mut winning_numbers = HashSet::new();
        while let Some(nr) = splitted.next() {
            if nr == "|" { break; }
            winning_numbers.insert(nr.parse::<u32>()?);
        }
        let numbers = splitted.map(|nr| nr.parse::<u32>()).collect::<Result<HashSet<u32>, ParseIntError>>()?;
        Ok(Card{ id, winning_numbers, numbers })
    }
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();
    let cards = args.input.lines()
        .map(|l| l.parse::<Card>().unwrap())
        .collect::<Vec<Card>>();

    let solution: i32 = if !args.second {
        cards.iter()
            .map(|c| c.winning_numbers.intersection(&c.numbers).collect::<Vec<&u32>>().len())
            .filter(|x| *x > 0)
            .fold(0, |acc, x| acc + (1 << (x-1))) as i32
    } else {
        let mut copies = vec![0u32; cards.len()];
        for i in 0..cards.len() {
            let quantity = copies[i] + 1;
            let matching = cards[i].winning_numbers.intersection(&cards[i].numbers).collect::<Vec<&u32>>().len();
            for j in 1..=matching {
                copies[i + j] += quantity;
            }
        }
        (copies.iter().sum::<u32>() + cards.len() as u32) as i32
    };

    result(solution, now.elapsed(), &args);
}
