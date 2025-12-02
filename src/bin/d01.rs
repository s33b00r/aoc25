use std::{str::FromStr, time::Instant};
use y25::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32)
}

#[derive(Debug)]
struct RotationError;

impl FromStr for Rotation {
    type Err = RotationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() { return Err(Self::Err {}); }
        match s.chars().next().unwrap() {
            'L' => Ok(Self::Left(s[1..s.len()].parse().unwrap())),
            'R' => Ok(Self::Right(s[1..s.len()].parse().unwrap())),
            _ => Err(Self::Err {})
        }
    }
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();
    let input: Vec<Rotation> = args.input.lines().map(|l| l.parse().unwrap()).collect();

    let solution: i32 = if !args.second {
        input.iter().fold((0, 50), |(zeros, pos), r| {
            match r {
                Rotation::Left(x) => {
                    if (pos - x) % 100 == 0 { (zeros + 1, (pos - x).rem_euclid(100)) }
                    else { (zeros, (pos - x).rem_euclid(100)) }
                }
                Rotation::Right(x) => {
                    if (pos + x) % 100 == 0 { (zeros + 1, (pos + x).rem_euclid(100)) }
                    else { (zeros, (pos + x).rem_euclid(100)) }
                },
            }
        }).0
    } else {
        input.iter().fold((0i32, 50), |(zeros, pos), r| {
            match r {
                Rotation::Left(x) => {
                    if pos == 0 { (zeros - (pos - x) / 100, (pos - x).rem_euclid(100)) }
                    else if (pos - x) <= 0 { (zeros + 1 - (pos - x) / 100, (pos - x).rem_euclid(100)) }
                    else { (zeros, pos - x) }
                }
                Rotation::Right(x) => (zeros + (pos + x) / 100, (pos + x).rem_euclid(100))
            }
        }).0
    };

    result(solution, now.elapsed(), &args);
}
