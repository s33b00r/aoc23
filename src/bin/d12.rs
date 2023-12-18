use std::time::Instant;
use y23::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

fn get_possible_count(s: &str, folds: usize) -> usize {
    let (s, q) = s.split_once(" ").unwrap();
    let quantities = q.split(",").map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>().repeat(folds);
    let springs = vec![s].repeat(folds).join("?") + ".";

    let mut broken = vec![];
    broken.push(0);
    let mut sum = 0;
    for c in springs.chars() {
        if c != '.' {
            sum += 1;
        }
        broken.push(sum);
    }
    let w = springs.len() - quantities.iter().sum::<usize>() - quantities.len() + 1;
    let mut partial_possible: Vec<usize> = vec![0; springs.len()];
    sum = 0;
    let mut table = vec![];
    let quantity = quantities[0];
    let mut valid = true;
    for i in 0..w {
        if springs.chars().nth(i + quantity) == Some('#') {
            sum = 0;
        } else if valid && broken[i + quantity] - broken[i] == quantity {
            sum += 1;
        }
        partial_possible[i + quantity] = sum;
        valid &= springs.chars().nth(i) != Some('#');
    }
    table.push(partial_possible);
    let mut start = quantity + 1;

    for &quantity in quantities.iter().skip(1) {
        sum = 0;
        partial_possible = vec![0; springs.len()];
        for i in start..start + w {
            if springs.chars().nth(i + quantity) == Some('#') {
                sum = 0;
            } else if *table.last().unwrap().get(i - 1).unwrap_or_else(|| &0) > 0 &&
                springs.chars().nth(i - 1) != Some('#') &&
                broken[i + quantity] - broken[i] == quantity {
                    sum += table.last().unwrap()[i - 1];
            }
            partial_possible[i+quantity] = sum;
        }
        start += quantity + 1;
        table.push(partial_possible);
    }
    *table.last().unwrap().last().unwrap()
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();
    let solution: usize = if !args.second {
        args.input.lines().map(|s| get_possible_count(s, 1)).sum()
    } else {
        args.input.lines().map(|s| get_possible_count(s, 5)).sum()
    };

    result(solution, now.elapsed(), &args);
}
