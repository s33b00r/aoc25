use std::{collections::HashSet, time::Instant, usize};
use y25::{args, result};

const BIN: &str = env!("CARGO_BIN_NAME");


fn number_neighbors(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> u32 {
    let mut neighbors = 0;
    for d_y in -1..=1 {
        let n_y = y as i32 + d_y;
        if n_y < 0 || n_y as usize >= grid.len() { continue; }
        for d_x in -1..=1 {
            if d_y == 0 && d_x == 0 { continue; }
            let n_x = x as i32 + d_x;
            if n_x < 0 || n_x as usize >= grid[0].len() { continue; }

            if grid[n_y as usize][n_x as usize] { neighbors += 1; }
        }
    }
    neighbors
}

fn remove_fewer_than_4(grid: &mut Vec<Vec<bool>>) -> Option<u32> {
    let mut to_remove = HashSet::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if !grid[y][x] { continue; }
            if number_neighbors(&grid, x, y) < 4 { 
                to_remove.insert((x, y));
            }
        }
    }

    for (x, y) in to_remove.iter() {
        grid[*y][*x] = false;
    }

    return if to_remove.len() > 0 { 
        Some(to_remove.len() as u32)
    } else {
        None
    }
}

fn main() {
    let args = args(BIN);
    let now = Instant::now();

    let mut input: Vec<Vec<bool>> =  args.input.lines()
        .map(|l| l.chars().map(|c| c == '@').collect())
        .collect();

    let solution: u32 = if !args.second {
        remove_fewer_than_4(&mut input).unwrap()
    } else {
        let mut removed = 0;
        while let Some(x) = remove_fewer_than_4(&mut input) {
            removed += x;
        }
        removed
    };

    result(solution, now.elapsed(), &args);
}
