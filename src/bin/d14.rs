use std::{time::Instant, ops::Range, collections::{HashSet, BTreeSet}};
use y23::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

fn move_dir(stones: &mut Vec<Vec<char>>, dir: char) {
    if "ns".contains(dir) {
        let y_iter: Box<dyn Iterator<Item=usize>> = 
            if dir == 'n' { Box::new(0..stones.len()) } else { Box::new((0..stones.len()).rev()) };
        let floor: i32 = if dir == 'n' { 0 } else { stones.len() as i32 - 1 };
        let dir: i32 = if dir == 'n' { -1 } else { 1 };
        for y in y_iter {
            for x in 0..stones[0].len() {
                if stones[y][x] != 'O' { continue; }
                let mut new_y = y as i32;
                loop {
                    let tmp = new_y + dir;
                    if y as i32 == floor || new_y == floor || stones[tmp as usize][x] != '.' { break; }
                    new_y = tmp;
                }
                stones[y][x] = '.';
                stones[new_y as usize][x] = 'O';
            }
        }
    } else if "ew".contains(dir) {
        let x_iter: Box<dyn Iterator<Item=usize>> = 
            if dir == 'w' { Box::new(0..stones[0].len()) } else { Box::new((0..stones[0].len()).rev()) };
        let floor: i32 = if dir == 'w' { 0 } else { stones[0].len() as i32 - 1 };
        let dir: i32 = if dir == 'w' { -1 } else { 1 };
        for x in x_iter {
            for y in 0..stones.len() {
                if stones[y][x] != 'O' { continue; }
                let mut new_x = x as i32;
                loop {
                    let tmp = new_x + dir;
                    if x as i32 == floor || new_x == floor || stones[y][tmp as usize] != '.' { break; }
                    new_x = tmp;
                }
                stones[y][x] = '.';
                stones[y][new_x as usize] = 'O';
            }
        }
    }
}

fn calculate_load(stones: &Vec<Vec<char>>) -> usize {
    let len = stones.len();
    stones.iter().enumerate()
        .map(|(y, r)| r.iter().map(|c| if *c == 'O' { len-y } else { 0 }).sum::<usize>())
        .sum()
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();
    let mut input: Vec<Vec<char>> = args.input.lines()
        .map(|l| l.chars().collect())
        .collect();

    let solution: usize = if !args.second {
        move_dir(&mut input, 'n');
        calculate_load(&input)
    } else {
        let mut seen = vec![input.iter().flatten().collect::<String>()];
        loop {
            for d in vec!['n', 'w', 's', 'e'] {
                move_dir(&mut input, d);
            }
            let as_str = input.iter().flatten().collect::<String>();
            if seen.contains(&as_str) { break; }
            seen.push(as_str);
        }

        let as_str = input.iter().flatten().collect::<String>();
        let start = seen.iter().position(|s| *s == as_str).unwrap();
        let cycle_len = seen.len() - start;

        let wanted = (1000000000 - seen.len()) % cycle_len;
        for _ in 0..wanted {
            for d in vec!['n', 'w', 's', 'e'] {
                move_dir(&mut input, d);
            }
        }
        calculate_load(&input)
    };

    result(solution, now.elapsed(), &args);
}
