use std::{time::Instant, str::FromStr, num::ParseIntError, collections::HashMap, cmp::Ordering};
use y23::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

#[derive(Debug)]
struct CamelHand {
    hand: String,
    score: u32
}

impl FromStr for CamelHand {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (h, s) = s.split_once(" ").unwrap();
        let hand = h.to_string().replace("A", "E").replace("K", "D")
            .replace("Q", "C").replace("J", "B").replace("T", "A");
        let score = s.parse::<u32>()?;
        Ok(CamelHand { hand, score })
    }
}

impl PartialEq for CamelHand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl Eq for CamelHand { }

fn char_quantity(s: &String) -> HashMap<char, u32> {
    let mut map = HashMap::new();
    for c in s.chars() {
        let c_count = map.entry(c).or_insert(0);
        *c_count += 1;
    }
    map
}

fn check_type_ordering(s: &HashMap<char, u32>, o: &HashMap<char, u32>) -> Option<Ordering> {
    let max_s = s.iter().map(|(_, v)| v).max().unwrap();
    let max_o = o.iter().map(|(_, v)| v).max().unwrap();
    if max_s != max_o {
        return max_s.partial_cmp(max_o);
    }
    if s.len() != o.len() {
        return o.len().partial_cmp(&s.len());
    }
    None
}

impl PartialOrd for CamelHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let s = char_quantity(&self.hand);
        let o = char_quantity(&other.hand);
        if let Some(ordering) = check_type_ordering(&s, &o) {
            return Some(ordering);
        }
        for i in 0..5 {
            let s_char = self.hand.chars().nth(i).unwrap();
            let o_char = other.hand.chars().nth(i).unwrap();
            if s_char != o_char {
                return s_char.partial_cmp(&o_char);
            }
        }
        None
    }
}

impl Ord for CamelHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn part_2_sort(s: &CamelHand, o: &CamelHand) -> Ordering {
    if !s.hand.contains("B") && !o.hand.contains("B") {
        return s.cmp(o);
    }

    let mut s_quantity = char_quantity(&s.hand);
    let s_j = s_quantity.remove(&'B').unwrap_or(0);
    let mut o_quantity = char_quantity(&o.hand);
    let o_j = o_quantity.remove(&'B').unwrap_or(0);
    let mut s_max = ('0', 0);
    for (key, value) in &s_quantity {
        if *value > s_max.1 {
            s_max = (*key, *value);
        }
    }
    s_quantity.insert(s_max.0, s_max.1 + s_j);
    let mut o_max = ('0', 0);
    for (key, value) in &o_quantity {
        if *value > o_max.1 {
            o_max = (*key, *value);
        }
    }
    o_quantity.insert(o_max.0, o_max.1 + o_j);
    if let Some(ordering) = check_type_ordering(&s_quantity, &o_quantity) {
        return ordering;
    }

    for (s_c, o_c) in s.hand.chars().zip(o.hand.chars()) {
        if s_c == o_c { continue; }
        if s_c == 'B' { return Ordering::Less; }
        if o_c == 'B' { return Ordering::Greater; }
        return s_c.cmp(&o_c);
    }
    
    panic!();
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();
    let mut hands = args.input.lines()
        .map(|l| l.parse::<CamelHand>().unwrap())
        .collect::<Vec<CamelHand>>();

    let solution: u32 = if !args.second {
        hands.sort();
        (0..hands.len()).map(|i| (i + 1) as u32 * hands[i].score).sum()
    } else {
        hands.sort_by(part_2_sort);
        (0..hands.len()).map(|i| (i + 1) as u32 * hands[i].score).sum()
    };

    result(solution, now.elapsed(), &args);
}
