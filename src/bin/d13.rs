use std::{time::Instant, str::FromStr, string::ParseError};
use y23::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

#[derive(Debug)]
struct Map {
    rows_as_number: Vec<usize>,
    cols_as_number: Vec<usize>,
}

impl FromStr for Map {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows_as_number: Vec<usize> = s.lines()
            .map(|l| l.chars().map(|c| if c == '#' { '1' } else { '0' }).collect::<String>())
            .map(|s| usize::from_str_radix(s.as_str(), 2).unwrap())
            .collect();
        let mut cols_as_number: Vec<usize> = vec![];
        let lines: Vec<&str> = s.lines().collect();
        for i in 0..lines[0].len() {
            let mut partial_num = String::new();
            for j in 0..lines.len() {
                partial_num += if lines[j].chars().nth(i).unwrap() == '#' {
                    "1"
                } else {
                    "0"
                }
            }
            cols_as_number.push(usize::from_str_radix(partial_num.as_str(), 2).unwrap());
        }

        Ok(Map { rows_as_number, cols_as_number })
    }
}

impl Map {
    fn find_mirror_point(&self) -> usize {
        fn check_middle(v: &Vec<usize>) -> Option<usize> {
            for i in 0..v.len()-1 {
                if v[i] != v[i+1] { continue; }
                let mut left = i as i32 - 1;
                let mut right = i + 2;
                let mut is_middle = true;
                while left >= 0 && right < v.len() {
                    if v[left as usize] != v[right] {
                        is_middle = false;
                        break;
                    }
                    left -= 1;
                    right += 1;
                }
                if is_middle {
                    return Some(i + 1);
                }
            }
            None
        }
        if let Some(v) = check_middle(&self.rows_as_number) {
            return v * 100;
        }
        check_middle(&self.cols_as_number).unwrap()
    }

    fn find_smudged_mirror_point(&self) -> usize {
        fn check_middle(v: &Vec<usize>) -> Option<usize> {
            fn smudged_equal(x1: usize, x2: usize) -> bool {
                if x1 == x2 { return false; }
                let val = x1 ^ x2;
                val & (val - 1) == 0
            }

            for i in 0..v.len()-1 {
                let mut equal = v[i] == v[i+1];
                let mut s_equal = smudged_equal(v[i], v[i+1]);
                if !(equal || s_equal) { continue; }
                let mut has_seen_smudge = s_equal;

                let mut left = i as i32 - 1;
                let mut right = i + 2;
                let mut is_middle = true;
                while left >= 0 && right < v.len() {
                    equal = v[left as usize] == v[right];
                    s_equal = smudged_equal(v[left as usize], v[right]);
                    if !(equal || s_equal) || (s_equal && has_seen_smudge) {
                        is_middle = false;
                        break;
                    }
                    has_seen_smudge |= s_equal;
                    left -= 1;
                    right += 1;
                }
                if is_middle && has_seen_smudge {
                    return Some(i + 1);
                }
            }
            None
        }
        if let Some(v) = check_middle(&self.rows_as_number) {
            return v * 100;
        }
        check_middle(&self.cols_as_number).unwrap()
    }
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();
    let input: Vec<Map> = args.input.split("\n\n")
        .map(|l| l.parse::<Map>().unwrap()).collect();

    let solution: usize = if !args.second {
        input.iter().map(|m| m.find_mirror_point()).sum()
    } else {
        input.iter().map(|m| m.find_smudged_mirror_point()).sum()
    };

    result(solution, now.elapsed(), &args);
}
