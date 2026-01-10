use std::{collections::VecDeque, io};

fn main() {
    let mut grid: Vec<Vec<Option<usize>>> = io::stdin()
        .lines()
        .flatten()
        .map(|line| line.chars().map(|ch| (ch == '@').then_some(0)).collect())
        .collect();

    // count neighbours
    points(&grid)
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|p @ (x, y)| {
            let ns = neighbours(p, &grid).count();
            if let Some(n) = grid[y][x].as_mut() {
                *n = ns;
            }
        });

    let mut stack = points(&grid)
        .filter(|&(x, y)| grid[y][x].is_some_and(|n| n < 4))
        .collect::<VecDeque<_>>();
    let silver = stack.len();
    let mut gold = 0;

    while let Some(p @ (x, y)) = stack.pop_back() {
        if grid[y][x].take().is_none() {
            continue;
        }
        gold += 1;
        // reduce neighbours' neighbour counts by 1
        neighbours(p, &grid)
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(nx, ny)| {
                if let Some(n) = grid[ny][nx].as_mut() {
                    *n -= 1;
                    if *n < 4 {
                        stack.push_back((nx, ny))
                    }
                }
            });
    }

    println!("silver: {silver}");
    println!("gold: {gold}");
}

fn neighbours<T>(
    (x, y): (usize, usize),
    grid: &Vec<Vec<Option<T>>>,
) -> impl Iterator<Item = (usize, usize)> {
    let xs = x.saturating_sub(1)..=(x + 1).min(grid.first().unwrap().len() - 1);
    let ys = y.saturating_sub(1)..=(y + 1).min(grid.len() - 1);

    xs.flat_map(move |nx| ys.clone().map(move |ny| (nx, ny)))
        .filter(move |n| *n != (x, y))
        .filter(|&(nx, ny)| grid.get(ny).and_then(|row| row.get(nx)).unwrap().is_some())
}

fn points<T>(grid: &Vec<Vec<T>>) -> impl Iterator<Item = (usize, usize)> {
    let xs = 0..grid.first().unwrap().len();
    let ys = 0..grid.len();
    xs.flat_map(move |x| ys.clone().map(move |y| (x, y)))
}
