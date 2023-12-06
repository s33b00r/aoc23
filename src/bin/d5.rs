use std::{time::Instant, str::FromStr, num::ParseIntError};
use y23::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

struct Almanac {
    seeds: Vec<i64>,
    map: Vec<Vec<(i64, i64, i64)>>
}

impl FromStr for Almanac {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.lines();
        let seeds = iter.next().unwrap().split(" ")
            .filter_map(|s| s.parse::<i64>().ok())
            .collect::<Vec<i64>>();
        let mut map: Vec<Vec<(i64, i64, i64)>> = vec![];
        let mut sub_map: Vec<(i64, i64, i64)> = vec![];
        while let Some(l) = iter.next() {
            if l.is_empty() { continue; }
            if l.contains("map") {
                if !sub_map.is_empty() {
                    sub_map.sort_by(|a, b| a.1.cmp(&b.1));
                    map.push(sub_map);
                    sub_map = vec![];
                }
                continue;
            }
            let m = l.split(" ")
                .map(|s| s.parse::<i64>())
                .collect::<Result<Vec<i64>, ParseIntError>>()?;
            sub_map.push((m[0], m[1], m[2]));
        }
        sub_map.sort_by(|a, b| a.1.cmp(&b.1));
        map.push(sub_map);
        Ok(Almanac { seeds, map })
    }
}

impl Almanac {
    fn lowest_location(&self, position: i64, depth: usize) -> i64 {
        let mut new_dest = position;
        for (d, s, r) in &self.map[depth] {
            if new_dest >= *s && new_dest < s + r {
                new_dest = position - s + d;
                break;
            }
        }
        if depth + 1 == self.map.len() { return new_dest; }
        return self.lowest_location(new_dest, depth + 1);
    }

    fn lowest_location_range(&self, low: i64, high: i64, depth: usize) -> i64 {
        let mut ranges: Vec<(i64, i64)> = vec![];
        let mut cur_low = low;
        for i in 0..self.map[depth].len() {
            let (d, s, r) = &self.map[depth][i];
            let h = s + r - 1;
            if *s > cur_low {
                if *s > high {
                    ranges.push((cur_low, high));
                    break;
                }
                ranges.push((cur_low, s - 1));
                if high <= h {
                    ranges.push((*d, d + high - s));
                    break;
                }
                ranges.push((*d, d + r - 1));
                cur_low = h + 1;
            } else if cur_low <= h && cur_low >= *s {
                if high <= h {
                    ranges.push((d + cur_low - s, d + high - s));
                    break;
                }
                ranges.push((d + cur_low - s, d + r - 1));
                cur_low = h + 1;
            } else {
                if i + 1 == self.map[depth].len() {
                    ranges.push((cur_low, high));
                }
            }
        }
        if depth + 1 == self.map.len() {
            return ranges.iter().map(|r| r.0).min().unwrap();
        }
        return ranges.iter()
            .map(|(l, h)| self.lowest_location_range(*l, *h, depth + 1))
            .min().unwrap();
    }
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();
    let almanac = args.input.parse::<Almanac>().unwrap();

    let solution: i64 = if !args.second {
        almanac.seeds.iter().map(|s| almanac.lowest_location(*s, 0))
            .min().unwrap()
    } else {
        almanac.seeds.chunks(2)
            .map(|c| almanac.lowest_location_range(c[0], c[0] + c[1] - 1, 0))
            .min().unwrap()
    };

    result(solution, now.elapsed(), &args);
}
