use std::{str::FromStr, time::Instant};
use y25::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");

#[derive(Debug)]
enum Rotation {
    Left(u32),
    Right(u32)
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

    let mut dial_pos = 50i32;
    let mut nr_zero = 0;

    let solution: i32 = if !args.second {
        for i in input {
            match i {
                Rotation::Left(x) => dial_pos -= x as i32,
                Rotation::Right(x) => dial_pos += x as i32
            }
            dial_pos = dial_pos.rem_euclid(100);
            if dial_pos == 0 { nr_zero += 1; }
        }
        nr_zero
    } else {
        for i in input {
            match i {
                Rotation::Left(x) => {
                    if dial_pos == 0 {
                        nr_zero -= 1;
                    }
                    dial_pos -= x as i32;
                }
                Rotation::Right(x) => dial_pos += x as i32
            }
            if dial_pos >= 100 { 
                nr_zero += dial_pos / 100; 
            } 
            if dial_pos <= 0 {
                nr_zero += -dial_pos / 100 + 1;
            }
            dial_pos = dial_pos.rem_euclid(100);
        }
        nr_zero
    };

    result(solution, now.elapsed(), &args);
}
