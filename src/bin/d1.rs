use std::time::Instant;
use y23::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

fn get_number(cal: &str) -> u32 {
    let v: Vec<u32> = cal.chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    if !v.is_empty() {
        return v[0] * 10 + v.last().unwrap();
    }
    return 0;
}


fn get_number_second(cal: &str) -> i32 {
    let mapping = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", 
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"
    ];
    let mut smallest = (0, i32::MAX);
    let mut largest = (0, i32::MIN);
    for (nr, map) in mapping.iter().enumerate() {
        cal.match_indices(map).for_each(|(i, _)| {
            if (i as i32) < smallest.1 { smallest = ((nr as i32) % 10, i as i32); }
            if (i as i32) > largest.1 { largest = ((nr as i32) % 10, i as i32); }
        });
    }
    return smallest.0 * 10 + largest.0;
}



fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let solution: i32 = if !args.second {
        args.input.lines()
            .map(|s| get_number(s))
            .fold(0, |acc, x| acc + x as i32)
    } else {
        args.input.lines()
            .map(|s| get_number_second(s))
            .fold(0, |acc, x| acc + x)
    };

    result(solution, now.elapsed(), &args);
}
