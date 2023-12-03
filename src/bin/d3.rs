use std::{time::Instant, str::FromStr, convert::Infallible, string::ParseError};
use y23::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

struct Grid {
    width: i32,
    height: i32,
    grid: Vec<Vec<char>>
}


impl Grid {
    fn get(&self, x: i32, y: i32) -> Option<char> {
        if self.width <= x || self.height <= y || x < 0 || y < 0 { return None }
        Some(self.grid[y as usize][x as usize])
    }

    fn check_grid(&self, x: i32, y: i32, filter: fn(char) -> bool) -> bool {
        for y_ in y-1..=y+1 {
            for x_ in x-1..=x+1 {
                if let Some(c) = self.get(x_, y_) {
                    if filter(c) { 
                        return true; 
                    }
                }
            }
        }
        return false;
    }

    fn find_all_chars(&self, f: fn(char) -> bool) -> Vec<(i32, i32)> {
        let mut found = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                if f(self.get(x, y).unwrap()) {
                    found.push((x, y));
                }
            }
        }
        found
    }
}

impl FromStr for Grid {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();
        let width = grid[0].len() as i32;
        let height = grid.len() as i32;
        Ok(Grid{ grid, width, height })
    }
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();
    let grid = args.input.parse::<Grid>().unwrap();

    let solution: i32 = if !args.second {
        let mut total = 0;
        for y in 0..grid.height {
            let mut partial_number = "".to_string();
            let mut symbolic_neighbor = false;
            for x in 0..grid.width {
                let c = grid.get(x, y).unwrap();
                if !c.is_digit(10) {
                    if symbolic_neighbor { 
                        total += partial_number.parse::<i32>().unwrap(); 
                    }
                    symbolic_neighbor = false;
                    partial_number.clear();
                    continue;
                }
                symbolic_neighbor |= grid.check_grid(x, y, |c| !c.is_digit(10) && c != '.');
                partial_number.push(c);
            }
            if symbolic_neighbor { total += partial_number.parse::<i32>().unwrap(); }
        }
        total
    } else {
        let mut numbers: Vec<Vec<(i32, i32)>> = vec![];
        let all_digits = grid.find_all_chars(|c| c.is_digit(10));
        let mut iter = all_digits.iter();
        let mut sub_list = vec![];
        let mut current = iter.next();

        while let Some(c) = current {
            if sub_list.is_empty() { 
                sub_list.push(*c);
                current = iter.next();
                continue;
            }

            let last = sub_list.last().unwrap();
            if last.0 + 1 == c.0 && last.1 == c.1 {
                sub_list.push(*c);
            } else {
                numbers.push(sub_list);
                sub_list = vec![*c];
            }
            current = iter.next();
        }
        numbers.push(sub_list);
        grid.find_all_chars(|c| c == '*').iter()
            .map(|(x, y)| numbers.iter().filter(|n| n.iter().any(|(x2, y2)| (x - x2).abs() <= 1 && (y - y2).abs() <= 1) ).collect::<Vec<&Vec<(i32, i32)>>>())
            .filter(|n| n.len() == 2)
            .map(|a| a.iter().map(|n| n.iter().map(|(x, y)| grid.get(*x, *y).unwrap()).collect::<String>()).map(|s| s.parse::<i32>().unwrap()).fold(1, |x, acc| acc * x))
            .sum()
    };

    result(solution, now.elapsed(), &args);
}
