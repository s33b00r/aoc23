use std::{time::Instant, collections::HashSet};
use y23::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

fn get_next_laser_pos(map: &Vec<Vec<char>>, pos: (i32, i32), dir: (i32, i32)) -> Vec<((i32, i32), (i32, i32))> {
    let mut new_lasers = vec![];
    match map[pos.1 as usize][pos.0 as usize] {
        '.' => new_lasers.push((pos, dir)),
        '/' => new_lasers.push((pos, (-dir.1, -dir.0))),
        '\\' => new_lasers.push((pos, (dir.1, dir.0))),
        '|' => {
            if dir.0 == 0 {
                new_lasers.push((pos, dir));
            } else {
                new_lasers.push((pos, (0, 1)));
                new_lasers.push((pos, (0, -1)));
            }
        }
        '-' => {
            if dir.1 == 0 {
                new_lasers.push((pos, dir));
            } else {
                new_lasers.push((pos, (1, 0)));
                new_lasers.push((pos, (-1, 0)));
            }
        }
        _ => panic!()
    }
    new_lasers
}

fn get_amount_of_lasers(map: &Vec<Vec<char>>, start_point: (i32, i32), dir: (i32, i32)) -> usize {
    let mut lasers = get_next_laser_pos(&map, start_point, dir);
    let mut seen = HashSet::new();
    seen.insert(start_point);
    let mut seen_laser = HashSet::new();
    loop {
        if lasers.is_empty() { break; }
        let mut new_lasers = vec![];
        for (pos, dir) in lasers.into_iter() {
            if !seen_laser.insert((pos, dir)) { continue; }
            if pos.0 < 0 || pos.1 < 0 || 
                pos.0 >= map[0].len() as i32 || pos.1 >= map.len() as i32 {
                continue;
            }
            seen.insert(pos); 
            let n_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if n_pos.0 < 0 || n_pos.1 < 0 || 
                n_pos.0 >= map[0].len() as i32 || n_pos.1 >= map.len() as i32 {
                continue;
            }
            new_lasers.append(&mut get_next_laser_pos(&map, n_pos, dir));
        }
        lasers = new_lasers;
    }
    seen.len()
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();
    let input: Vec<Vec<char>> = args.input.lines().map(|l| l.chars().collect()).collect();

    let solution: usize = if !args.second {
        get_amount_of_lasers(&input, (0, 0), (1, 0))
    } else {
        let mut max = 0;
        for x in 0..input[0].len() {
            max = max.max(get_amount_of_lasers(&input, (x as i32, 0), (0, 1)));
            max = max.max(get_amount_of_lasers(&input, (x as i32, input.len() as i32-1), (0, -1)));
        }
        for y in 0..input.len() {
            max = max.max(get_amount_of_lasers(&input, (0, y as i32), (1, 0)));
            max = max.max(get_amount_of_lasers(&input, (input[0].len() as i32-1, y as i32), (-1, 0)));
        }
        max
    };

    result(solution, now.elapsed(), &args);
}
