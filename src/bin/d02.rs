use std::{str::FromStr, time::Instant};
use y25::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

struct Range {
    from: u64,
    to: u64
}

#[derive(Debug)]
struct RangeError;

impl FromStr for Range {
    type Err = RangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split_once('-').ok_or(RangeError)?;
        Ok(Self { 
            from: l.parse().or_else(|_| Err(RangeError))?, 
            to: r.parse().or_else(|_| Err(RangeError))?, 
        })
    }
}

fn is_repeated(sub_str: &str, full: &str) -> bool {
    if full.len() % sub_str.len() != 0 { return false; }

    for i in (0..full.len()).step_by(sub_str.len()) {
        if &full[i..i+sub_str.len()] != sub_str { return false; }
    }

    true
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let input: Vec<Range> = args.input.trim().split(',').map(|s| s.parse().unwrap()).collect();

    let solution: u64 = if !args.second {
        let mut sum = 0;
        for i in input {
            for x in i.from..=i.to {
                let as_str = format!("{x}");
                let (l, r) = as_str.split_at(as_str.len() / 2);
                if l == r { sum += x; }
            }
        }
        sum
    } else {
        let mut sum = 0;
        for i in input {
            for x in i.from..=i.to {
                let as_str = format!("{x}");
                for split_at in 1..=(as_str.len()/2) {
                    if is_repeated(&as_str[0..split_at], as_str.as_str()) { 
                        sum += x; 
                        break;
                    }
                }
            }
        }
        sum
    };

    result(solution, now.elapsed(), &args);
}
