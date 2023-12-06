use std::{time::Instant, str::FromStr, num::ParseIntError};
use y23::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

struct Records {
    time: Vec<u32>,
    distance: Vec<u32>,
}

impl FromStr for Records {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s.lines().map(|l| l.split(" ")
            .filter(|sp| !sp.is_empty())
            .skip(1)
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>())
            .collect::<Vec<Vec<u32>>>();
        Ok(Records { time: items[0].clone(), distance: items[1].clone() })
    }
}

fn distances_higher(time: u64, distance: u64) -> u64 {
    let mut r = 0;
    loop {
        if r * (time - r) > distance {
            break;
        }
        r += 1;
    }
    time - r * 2 + 1
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();
    let records = args.input.parse::<Records>().unwrap();

    let solution: u64 = if !args.second {
        records.time.iter().zip(records.distance.iter())
            .map(|(t, d)| distances_higher(*t as u64, *d as u64))
            .fold(1, |acc, x| acc * x)
    } else {
        let time = records.time.iter()
            .map(|x| x.to_string()).collect::<String>()
            .parse::<u64>().unwrap();
        let distance = records.distance.iter()
            .map(|x| x.to_string()).collect::<String>()
            .parse::<u64>().unwrap();
        distances_higher(time, distance)
    };

    result(solution, now.elapsed(), &args);
}
