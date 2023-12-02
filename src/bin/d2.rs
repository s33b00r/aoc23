use std::{str::FromStr, num::ParseIntError};
use std::time::Instant;
use y23::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

#[derive(Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
enum SetParseError {
    Int(ParseIntError),
    String(String),
}
impl SetParseError {
    fn to_string(&self) -> String {
        match self {
            SetParseError::Int(i) => i.to_string(),
            SetParseError::String(s) => s.clone()
        }
    }
}

impl From<ParseIntError> for SetParseError {
    fn from(value: ParseIntError) -> Self {
        SetParseError::Int(value)
    }
}

impl FromStr for Set {
    type Err = SetParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = Set { red: 0, green: 0, blue: 0 };
        for c in s.trim().split(',') {
            let (q_str, color) = c.trim()
                .split_once(' ')
                .ok_or(SetParseError::String("Could not split".to_string()))?;
            let quantity = q_str.parse::<u32>()?;
            match color {
                "red" => set.red = quantity,
                "green" => set.green = quantity,
                "blue" => set.blue = quantity,
                _ => return Err(SetParseError::String("Unknown value".to_string()))
            }
        }
        Ok(set)
    }
}

impl Default for Set {
    fn default() -> Self {
        Set { red: 0, green: 0, blue: 0 }
    }
}

impl Set {
    fn has_more(&self, other: &Set) -> bool {
        self.red < other.red || self.green < other.green || self.blue < other.blue
    }

    fn max_each(&self, other: &Set) -> Set {
        let red = self.red.max(other.red);
        let green = self.green.max(other.green);
        let blue = self.blue.max(other.blue);
        Set { red, green, blue }
    }
}

struct Game {
    id: u32,
    sets: Vec<Set>
}

#[derive(Debug)]
enum GameParseError {
    Int(ParseIntError),
    String(String),
}

impl From<ParseIntError> for GameParseError {
    fn from(value: ParseIntError) -> Self {
        GameParseError::Int(value)
    }
}

impl From<SetParseError> for GameParseError {
    fn from(s: SetParseError) -> Self {
        GameParseError::String(s.to_string())
    }
}

impl FromStr for Game {
    type Err = GameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split([':', ';']);
        let id = splitted.next()
            .ok_or(GameParseError::String("No id colon".to_string()))?
            .split_once(' ')
            .ok_or(GameParseError::String("No space between game and id".to_string()))?
            .1.parse::<u32>()?;
        let sets = splitted.map(|s| s.parse::<Set>())
            .collect::<Result<Vec<Set>, SetParseError>>()?;
        Ok(Self { id, sets })
    }
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let games: Vec<Game> = args.input.lines()
        .map(|l| l.parse::<Game>().unwrap())
        .collect();

    let solution: i32 = if !args.second {
        let allowed = Set { red: 12, green: 13, blue: 14 };
        games.into_iter()
            .filter(|g| g.sets.iter().all(|s| !allowed.has_more(s)))
            .map(|g| g.id as i32)
            .sum()
    } else {
        games.iter()
            .map(|g| g.sets.iter().fold(Set::default(), |s, acc| s.max_each(acc)))
            .map(|m_set| m_set.red * m_set.green * m_set.blue)
            .sum::<u32>() as i32
    };

    result(solution, now.elapsed(), &args);
}
