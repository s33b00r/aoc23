use std::{time::Instant, str::FromStr, string::ParseError, collections::{HashSet, VecDeque}, ops::Add};
use y23::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

struct Pipes {
    start: Coords,
    pipes: Vec<Vec<(Coords, Coords)>>,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct Coords { 
    x: i32,
    y: i32
}

impl Coords {
    fn new(x: i32, y: i32) -> Self {
        Coords { x, y }
    }
}

impl Add for Coords {
    type Output = Coords;

    fn add(self, rhs: Self) -> Self::Output {
        Coords::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl FromStr for Pipes {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn get_connected(t: char, x: i32, y: i32) -> (Coords, Coords) {
            match t {
                '|' => (Coords::new(x, y-1), Coords::new(x, y+1)),
                '-' => (Coords::new(x-1, y), Coords::new(x+1, y)),
                'L' => (Coords::new(x, y-1), Coords::new(x+1, y)),
                'J' => (Coords::new(x, y-1), Coords::new(x-1, y)),
                '7' => (Coords::new(x-1, y), Coords::new(x, y+1)),
                'F' => (Coords::new(x+1, y), Coords::new(x, y+1)),
                '.'|'S' => (Coords::new(-1, -1), Coords::new(-1, -1)),
                _ => panic!("Cannot handle {t}")
            }
        }

        let mut pipes = s.lines().enumerate()
            .map(|(y, l)| l.chars().enumerate()
                 .map(|(x, c)| get_connected(c, x as i32, y as i32))
                 .collect::<Vec<(Coords, Coords)>>())
            .collect::<Vec<Vec<(Coords, Coords)>>>();
        let s_i = s.find("S").unwrap();
        let w = s.find("\n").unwrap() + 1;
        let start = Coords::new((s_i % w) as i32, (s_i / w) as i32);
        let connections = vec![(start.x - 1, start.y), (start.x + 1, start.y), 
            (start.x, start.y - 1), (start.x, start.y + 1)]
            .iter().filter(|(x, y)| *x >= 0 && *x < pipes[0].len() as i32 && *y >= 0 && *y < pipes.len() as i32)
            .filter_map(|(x, y)| {
                let around = pipes[*y as usize][*x as usize];
                if around.0 == start || around.1 == start {
                    Some(Coords::new(*x, *y))
                } else {
                    None
                }
            }).collect::<Vec<Coords>>();
        pipes[start.y as usize][start.x as usize] = (connections[0], connections[1]);
        Ok(Pipes { pipes, start })
    }
}

impl Pipes {
    fn get(&self, coords: &Coords) -> &(Coords, Coords) {
        &self.pipes[coords.y as usize][coords.x as usize]
    }

    fn get_next_pipe(&self, coords: &Coords, prev: &Coords) -> Coords {
        let possible = self.get(coords);
        if &possible.0 == prev {
            return possible.1;
        }
        possible.0
    }
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();
    let pipes = args.input.parse::<Pipes>().unwrap();

    let solution: i32 = if !args.second {
        let mut n = 1;
        let mut c1 = pipes.get(&pipes.start).0;
        let mut p1 = pipes.start;
        let mut c2 = pipes.get(&pipes.start).1;
        let mut p2 = pipes.start;
        while c1 != c2 {
            n += 1;
            let tmp1 = c1;
            c1 = pipes.get_next_pipe(&c1, &p1);
            p1 = tmp1;
            let tmp2 = c2;
            c2 = pipes.get_next_pipe(&c2, &p2);
            p2 = tmp2;
        }
        n
    } else {
        // This version can only handle when going through the loop in a clockwise manner, which
        // means setting the first cur to the correct value and need to manually specify the 
        // orth_vec otherwise it is good.
        let mut cur = pipes.get(&pipes.start).1;
        let mut prev = pipes.start;
        let mut loop_pipes = HashSet::new();
        loop {
            loop_pipes.insert(cur);
            if cur == pipes.start { break; }
            let tmp = cur;
            cur = pipes.get_next_pipe(&cur, &prev);
            prev = tmp;
        }
        let mut inside = HashSet::new();
        let mut orth_vec = Coords::new(0, -1);
        loop {
            if !loop_pipes.contains(&(cur + orth_vec)) {
                inside.insert(cur + orth_vec);
            }
            let new = pipes.get_next_pipe(&cur, &prev);
            let mut change = false;
            if new.y != cur.y && orth_vec.y != 0 {
                orth_vec = Coords::new(cur.y - new.y, 0);
                change = true;
            } else if new.x != cur.x && orth_vec.x != 0 { 
                orth_vec = Coords::new(0, new.x - cur.x);
                change = true;
            }
            if change && !loop_pipes.contains(&(cur + orth_vec)) {
                inside.insert(cur + orth_vec);
            }
            if new == pipes.start { break; }
            prev = cur;
            cur = new;
        }
        let mut queue: VecDeque<Coords> = inside.iter().map(|x| *x).collect();
        let mut seen = HashSet::new();
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            if seen.contains(&current) || loop_pipes.contains(&current) { continue; }
            seen.insert(current);
            for to_check in vec![Coords::new(0, -1), Coords::new(0, 1), Coords::new(-1, 0), Coords::new(1, 0)] {
                queue.push_back(current + to_check);
            }
        }
        seen.len() as i32
    };

    result(solution, now.elapsed(), &args);
}
