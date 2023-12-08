use std::{time::Instant, collections::{HashMap, HashSet}, str::FromStr, string::ParseError};
use y23::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

struct Map {
    path: HashMap<String, (String, String)>,
    instructions: String
}

impl Map {
    fn get_instruction(&self, step: usize) -> char {
        let l = self.instructions.len();
        self.instructions.chars().nth(step % l).unwrap()
    }

    fn go_step(&self, current: &String, dir: char) -> &String {
        match dir {
            'L' => &self.path[current].0,
            'R' => &self.path[current].1,
            _ => panic!("Cannot handle {dir} directions")
        }
        
    }

    /// Finds the loop from a starting point on the map
    /// Returns: (Vector of how many steps to reach an endpoints in loop, the loop size)
    fn find_loop(&self, start_point: &String) -> (Vec<usize>, usize, usize) {
        let mut seen: HashSet<(usize, &String)> = HashSet::new();
        let mut current = (0, start_point);
        let mut ends = vec![];

        loop {
            if !seen.insert(current) { break; }
            let dir = self.get_instruction(current.0);
            let step = (current.0 + 1) % self.instructions.len();
            let pos = self.go_step(current.1, dir);
            current = (step, pos);
            if pos.ends_with("Z") { ends.push(seen.len()); }
        }
        let mut tail = (0, start_point);
        while tail.1 != current.1 {
            let dir = self.get_instruction(tail.0);
            let pos = self.go_step(tail.1, dir);
            tail = (tail.0 + 1, pos);
        }
        (ends, seen.len(), tail.0)
    }
}

impl FromStr for Map {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let instructions = lines.next().unwrap().to_string();
        let path = lines.skip(1).map(|l| {
            let (key, end) = l.split_once(" = ").unwrap();
            let value = end.trim_matches(|c| c == '(' || c == ')')
                .split_once(", ").unwrap();
            (key.to_string(), (value.0.to_string(), value.1.to_string()))
        })
        .collect::<HashMap<String, (String, String)>>();
        let path_index = path.iter().enumerate().map(|(i, (k, _))| (k, i)).collect::<HashMap<_, _>>();
        let mut path_vec = vec![(0, 0); path_index.len()];
        for k in path_index.keys() {
            let i = path_index[k];
            let val = path[*k].clone();
            let i_val = (path_index[&val.0], path_index[&val.1]);
            path_vec[i] = i_val;
        }
        Ok(Map { path, instructions })
    }
}

trait extra_math {
    fn lcm(self, other: Self) -> Self;
    fn gcd(self, other: Self) -> Self;
}

impl extra_math for usize {
    fn lcm(self, other: Self) -> Self {
        self * other / self.gcd(other)
    }

    fn gcd(self, other: Self) -> Self {
        let mut a = self;
        let mut b = other;
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();
    let map = args.input.parse::<Map>().unwrap();

    let mut steps = 0;
    let solution: usize = if !args.second {
        let mut current = &"AAA".to_string();
        loop {
            let i = map.get_instruction(steps);
            steps += 1;
            current = map.go_step(current, i);
            if current == "ZZZ" { break; }
        }
        steps
    } else {
        let current = map.path.iter()
            .filter_map(|(k, _)| if k.ends_with("A") { Some(k) } else { None })
            .map(|s| map.find_loop(s))
            .map(|(v, l, t)| (v.iter().map(|e| e % (l - t)).collect::<Vec<usize>>(), l - t))
            .collect::<Vec<(Vec<usize>, usize)>>();
        // This answer only works because the z (end) point is equally as far from the start, as
        // the whole loop is.
        // Could continue work on this if bored, but a general formula cannot really be optimized
        // that well, from what I know.
        // The hard part is if the end point was not at "zero", because then the general formula
        // goes from [LCM * n, where n is a whole number] to [LCM * n + c, where n is a whole
        // number] and knowing what c is, I have not found a fast algorithm for, more than just
        // trying.
        current.iter().fold(1, |acc, c| acc.lcm(c.1))
    };

    result(solution, now.elapsed(), &args);
}
