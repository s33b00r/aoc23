use std::time::Instant;
use y23::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

fn get_triangle(seq: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut triangle = vec![seq.clone()];

    while !triangle.last().unwrap().iter().all(|x| *x == 0) {
        let last = triangle.last().unwrap();
        let mut next = vec![];
        for i in 1..last.len() {
            next.push(last[i] - last[i - 1]);
        }
        triangle.push(next);
    }
    triangle
}

fn get_next(seq: &Vec<i32>) -> i32 {
    let triangle = get_triangle(seq);
    let mut sum = 0;
    for i in 0..triangle.len() {
        sum += triangle[i].last().unwrap();
    }
    sum
}

fn get_prev(seq: &Vec<i32>) -> i32 {
    let triangle = get_triangle(seq);
    let mut sum = 0;
    for i in (0..triangle.len()).rev() {
        sum = triangle[i][0] - sum;
    }
    sum
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();
    let input: Vec<Vec<i32>> = args.input.lines()
        .map(|l| l.split(" ").map(|n| n.parse().unwrap()).collect())
        .collect();

    let solution: i32 = if !args.second {
        input.iter().map(get_next).sum()
    } else {
        input.iter().map(get_prev).sum()
    };

    result(solution, now.elapsed(), &args);
}
