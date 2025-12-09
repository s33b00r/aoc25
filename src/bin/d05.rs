use std::time::Instant;
use y25::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

fn in_range(ingredient: u64, ranges: &Vec<(u64, u64)>) -> bool {
    for (s, l) in ranges {
        if ingredient >= *s && ingredient <= *l {
            return true;
        }
    }
    false
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let (ranges, ingredients) = args.input.split_once("\n\n").unwrap();
    let mut ranges: Vec<(u64, u64)> = ranges.lines()
        .map(|s| s.split_once('-').unwrap())
        .map(|(l,r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect();

    let ingredients: Vec<u64> = ingredients.lines()
        .map(|s| s.parse().unwrap())
        .collect();

    let solution: u64 = if !args.second {
        ingredients.into_iter().filter(|i| in_range(*i, &ranges)).count() as u64
    } else {
        ranges.sort();
        let mut new_ranges: Vec<(u64, u64)> = vec![];
        let mut current_range = ranges[0];
        for i in 1..ranges.len() {
            if current_range.1 >= ranges[i].1 { continue; }
            if current_range.1 >= ranges[i].0 {
                current_range.1 = ranges[i].1;
                continue;
            } 
            new_ranges.push(current_range);
            current_range = ranges[i];
        }
        new_ranges.push(current_range);
        new_ranges.into_iter().map(|(l, r)| r - l + 1).sum()
    };

    result(solution, now.elapsed(), &args);
}
