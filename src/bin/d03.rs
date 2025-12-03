use std::time::Instant;
use y25::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

fn find_largest_between(min_index: usize, max_index: usize, s: &str) -> (usize, char) {
    let mut max = (0, '0');
    for (i, c) in s[min_index..max_index].char_indices() {
        if c > max.1 {
            max = (i, c);
        }
    }
    (max.0 + min_index, max.1)
}

fn find_largest_batteries(nr: usize, all_batteries: &str) -> String {
    let mut used_batteries = String::new();
    let mut start_point = 0;
    for batteries_remaining in (1..=nr).rev() {
        let (tmp, c) = find_largest_between(
            start_point, 
            all_batteries.len()-batteries_remaining+1, 
            all_batteries
        );
        start_point = tmp+1;
        used_batteries.push(c);
    }
    used_batteries
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let solution: u64 = if !args.second {
        args.input.lines()
            .map(|l| find_largest_batteries(2, l).parse::<u64>().unwrap())
            .sum()
    } else {
        args.input.lines()
            .map(|l| find_largest_batteries(12, l).parse::<u64>().unwrap())
            .sum()
    };

    result(solution, now.elapsed(), &args);
}
