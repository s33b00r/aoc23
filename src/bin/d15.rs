use std::time::Instant;
use y23::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

fn hash(s: &str) -> usize {
    s.chars().map(|c| c as usize).fold(0, |acc, x| ((acc + x) * 17) % 256)
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();
    let input = args.input.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

    let solution: usize = if !args.second {
        input.split(",").map(hash).sum()
    } else {
        let mut map: Vec<Vec<(String, u32)>> = vec![vec![]; 256];
        for instruction in input.split(",") {
            let instruction_index = instruction.chars().position(|c| "-=".contains(c)).unwrap();
            let label = instruction.chars().take(instruction_index).collect::<String>();
            let hash = hash(&label);
            let op = instruction.chars().nth(instruction_index).unwrap();
            let focal = instruction.chars().nth(instruction_index + 1);
            let o_i = map[hash].iter().position(|(l, _)| label == *l);
            match op {
                '-' => {
                    if let Some(i) = o_i {
                        map[hash].remove(i);
                    }
                }
                '=' => {
                    let f = focal.unwrap().to_digit(10).unwrap();
                    if let Some(i) = o_i {
                        map[hash][i] = (label, f);
                    } else {
                        map[hash].push((label, f));
                    }
                }
                _ => panic!("Cannot handle {op}")
            }
        }
        map.iter().enumerate()
            .map(|(b_i, b)| b.iter().enumerate().map(|(i_i, (_, f))| (b_i + 1) * (i_i + 1) * *f as usize).sum::<usize>())
            .sum()
    };

    result(solution, now.elapsed(), &args);
}
