use std::{time::Instant, str::FromStr, string::ParseError, collections::HashSet};
use y23::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

struct GalaxyMap {
    galaxies: Vec<(usize, usize)>,
    empty_horizontal: Vec<usize>,
    empty_vertical: Vec<usize>,
    empty_space_multiplier: usize
}

impl FromStr for GalaxyMap {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let galaxies = s.lines().enumerate()
            .map(|(y, l)| l.chars().enumerate()
                 .filter_map(|(x, c)| if c == '#' { Some((x, y)) } else { None })
                 .collect::<Vec<(usize, usize)>>())
            .flatten()
            .collect::<Vec<(usize, usize)>>();
        let mut horizontal = (0..s.lines().count()).collect::<HashSet<usize>>();
        let mut vertical = (0..s.lines().next().unwrap().chars().count()).collect::<HashSet<usize>>();
        for galaxy in galaxies.iter() {
            horizontal.remove(&galaxy.1);
            vertical.remove(&galaxy.0);
        }
        let empty_horizontal = horizontal.iter().map(|x| *x).collect();
        let empty_vertical = vertical.iter().map(|x| *x).collect();
        Ok(GalaxyMap { galaxies, empty_horizontal, empty_vertical, empty_space_multiplier: 2 })
    }
}

impl GalaxyMap {
    fn distance(&self, from: usize, to: usize, extra_vec: &Vec<usize>) -> usize {
        let max = from.max(to);
        let min = from.min(to);
        let extra = extra_vec.iter().filter(|r| **r > min && **r < max).count();
        return max - min + extra * (self.empty_space_multiplier - 1);
    }
    fn vertical_distance(&self, from: usize, to: usize) -> usize {
        self.distance(from, to, &self.empty_horizontal)
    }

    fn horizontal_distance(&self, from: usize, to: usize) -> usize {
        self.distance(from, to, &self.empty_vertical)
    }
}

fn sum(map: &GalaxyMap) -> usize {
    let mut sum = 0;
    for i in 0..(map.galaxies.len()-1) {
        for j in (i+1)..map.galaxies.len() {
            sum += map.horizontal_distance(map.galaxies[i].0, map.galaxies[j].0);
            sum += map.vertical_distance(map.galaxies[i].1, map.galaxies[j].1);
        }
    }
    sum
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();
    let mut map = args.input.parse::<GalaxyMap>().unwrap();

    let solution: usize = if !args.second {
        sum(&map)
    } else {
        map.empty_space_multiplier = 1_000_000;
        sum(&map)
    };

    result(solution, now.elapsed(), &args);
}
